use crate::error::EmulatorError;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize, SlavePty};
use std::io::{Read, Write};

pub struct Pty {
    master: Box<dyn MasterPty + Send>,
    slave: Box<dyn SlavePty + Send>,
    child: Box<dyn Child + Send + Sync>,
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
        let cmd = CommandBuilder::new(shell);
        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;

        Ok(Self {
            master: pair.master,
            slave: pair.slave,
            child,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), EmulatorError> {
        let mut writer = self
            .master
            .take_writer()
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;
        writer
            .write_all(data)
            .map_err(|e| EmulatorError::Io(e))?;
        writer.flush().map_err(|e| EmulatorError::Io(e))?;
        Ok(())
    }

    pub fn read_available(&mut self) -> Result<Vec<u8>, EmulatorError> {
        let mut reader = self
            .master
            .try_clone_reader()
            .map_err(|e| EmulatorError::Pty(e.to_string()))?;
        let mut buf = vec![0u8; 8192];
        match reader.read(&mut buf) {
            Ok(n) => {
                buf.truncate(n);
                Ok(buf)
            }
            Err(_) => Ok(Vec::new()),
        }
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
}
