pub struct EmulatorConfig {
    pub cols: usize,
    pub rows: usize,
    pub shell: String,
    pub scrollback: usize,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            shell: std::env::var("SHELL").unwrap_or_else(|_| "/system/bin/sh".to_string()),
            scrollback: 10000,
        }
    }
}
