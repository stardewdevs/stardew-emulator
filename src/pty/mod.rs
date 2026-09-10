use crate::error::EmulatorError;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize, SlavePty};
use std::io::{Read, Write};

pub struct Pty {
    master: Box<dyn MasterPty + Send>,
    slave: Box<dyn SlavePty + Send>,
    child: Box<dyn Child + Send + Sync>,
    reader: Option<Box<dyn Read + Send>>,
    writer: Option<Box<dyn Write + Send>>,
}

impl Pty {
    pub fn new(cols: u16, rows: u16) -> Result<Self, EmulatorError> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/system/bin/sh".to_string());
        let mut cmd = CommandBuilder::new(shell);
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");

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
            slave: pair.slave,
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
