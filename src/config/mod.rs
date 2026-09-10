#[derive(Clone, Debug)]
pub struct EmulatorConfig {
    pub cols: usize,
    pub rows: usize,
    pub shell: String,
    pub scrollback: usize,
    pub term: String,
    pub colorterm: String,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            shell: std::env::var("SHELL").unwrap_or_else(|_| "/system/bin/sh".to_string()),
            scrollback: 10_000,
            term: "xterm-256color".to_string(),
            colorterm: "truecolor".to_string(),
        }
    }
}
