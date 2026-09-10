use crate::config::EmulatorConfig;
use crate::error::EmulatorError;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};

pub struct Pty {
    master: Box<dyn MasterPty + Send>,
    child: Box<dyn Child + Send + Sync>,
    reader: Option<Box<dyn Read + Send>>,
    writer: Option<Box<dyn Write + Send>>,
}

impl Pty {
    pub fn new(config: &EmulatorConfig) -> Result<Self, EmulatorError> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: config.rows as u16,
                cols: config.cols as u16,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        if !std::path::Path::new(&config.shell).exists() {
            return Err(EmulatorError::Config(format!(
                "Stardew shell not found at {}. Run the bootstrap first.",
                config.shell
            )));
        }

        let mut cmd = CommandBuilder::new(&config.shell);
        cmd.arg("-l");
        cmd.env("TERM", &config.term);
        cmd.env("COLORTERM", &config.colorterm);
        cmd.env("HOME", &config.home);
        cmd.env("PREFIX", &config.prefix);
        cmd.env("PATH", &config.path);
        cmd.env("LD_LIBRARY_PATH", &config.ld_library_path);
        cmd.env("LANG", &config.lang);
        cmd.env("LC_ALL", &config.lang);
        cmd.cwd(&config.home);

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        let writer = pair
            .master
            .take_writer()
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        Ok(Self {
            master: pair.master,
            child,
            reader: Some(reader),
            writer: Some(writer),
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), EmulatorError> {
        if let Some(writer) = self.writer.as_mut() {
            writer.write_all(data)?;
            writer.flush()?;
        }
        Ok(())
    }

    pub fn read_available(&mut self) -> Result<Vec<u8>, EmulatorError> {
        let mut all = Vec::new();
        if let Some(reader) = self.reader.as_mut() {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        all.extend_from_slice(&buf[..n]);
                        if n < buf.len() {
                            break;
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        }
        Ok(all)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<(), EmulatorError> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;
        Ok(())
    }

    pub fn child(&mut self) -> &mut Box<dyn Child + Send + Sync> {
        &mut self.child
    }

    pub fn kill(&mut self) -> Result<(), EmulatorError> {
        self.child
            .kill()
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;
        Ok(())
    }

    pub fn wait(&mut self) -> Result<portable_pty::ExitStatus, EmulatorError> {
        self.child
            .wait()
            .map_err(|e| EmulatorError::Pty(e.to_string()))
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
