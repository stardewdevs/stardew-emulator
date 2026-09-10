use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("PTY error: {0}")]
    Pty(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Terminal error: {0}")]
    Terminal(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("JNI error: {0}")]
    Jni(String),
}

pub type Result<T> = std::result::Result<T, EmulatorError>;
