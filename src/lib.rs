pub mod config;
pub mod engine;
pub mod error;
pub mod ffi;
pub mod input;
pub mod logger;
pub mod output;
pub mod parser;
pub mod platform;
pub mod pty;
pub mod session;
pub mod snapshot;
pub mod utils;

#[cfg(feature = "jni")]
pub mod jni;

pub use config::EmulatorConfig;
pub use engine::Engine;
pub use error::EmulatorError;
pub use pty::Pty;
pub use session::Session;
pub use snapshot::{Cell, Color, TerminalMode, TerminalSnapshot};

use std::sync::{Arc, Mutex};

pub struct StardewEmulator {
    engine: Arc<Mutex<Engine>>,
    pty: Arc<Mutex<Pty>>,
    session: Arc<Mutex<Session>>,
    snapshot: TerminalSnapshot,
}

impl StardewEmulator {
    pub fn new(cols: usize, rows: usize) -> Result<Self, EmulatorError> {
        let engine = Engine::new(cols, rows);
        let pty = Pty::new(cols as u16, rows as u16)?;
        let session = Session::new();
        let snapshot = TerminalSnapshot::empty(cols, rows);

        Ok(Self {
            engine: Arc::new(Mutex::new(engine)),
            pty: Arc::new(Mutex::new(pty)),
            session: Arc::new(Mutex::new(session)),
            snapshot,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), EmulatorError> {
        self.pty.lock().unwrap().write(data)
    }

    pub fn resize(&mut self, cols: usize, rows: usize) -> Result<(), EmulatorError> {
        self.engine.lock().unwrap().resize(cols, rows);
        self.pty.lock().unwrap().resize(cols as u16, rows as u16)?;
        self.snapshot = TerminalSnapshot::empty(cols, rows);
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), EmulatorError> {
        let output = self.pty.lock().unwrap().read_available()?;
        if !output.is_empty() {
            let mut engine = self.engine.lock().unwrap();
            engine.process(&output);
            self.snapshot = engine.snapshot();
        }
        Ok(())
    }

    pub fn snapshot(&self) -> &TerminalSnapshot {
        &self.snapshot
    }

    pub fn engine(&self) -> Arc<Mutex<Engine>> {
        Arc::clone(&self.engine)
    }

    pub fn pty(&self) -> Arc<Mutex<Pty>> {
        Arc::clone(&self.pty)
    }

    pub fn session(&self) -> Arc<Mutex<Session>> {
        Arc::clone(&self.session)
    }
}
