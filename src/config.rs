#[derive(Clone, Debug)]
pub struct EmulatorConfig {
    pub cols: usize,
    pub rows: usize,
    pub shell: String,
    pub scrollback: usize,
    pub term: String,
    pub colorterm: String,
    pub prefix: String,
    pub home: String,
    pub path: String,
    pub ld_library_path: String,
    pub lang: String,
}

impl EmulatorConfig {
    pub fn stardew_defaults(prefix: &str, home: &str) -> Self {
        let usr = format!("{}/usr", prefix);
        let bin = format!("{}/bin", usr);
        let lib = format!("{}/lib", usr);

        let path = format!("{}:{}:/system/bin:/system/xbin", bin, usr);

        Self {
            cols: 80,
            rows: 24,
            shell: format!("{}/bash", bin),
            scrollback: 10_000,
            term: "xterm-256color".to_string(),
            colorterm: "truecolor".to_string(),
            prefix: prefix.to_string(),
            home: home.to_string(),
            path,
            ld_library_path: lib,
            lang: "en_US.UTF-8".to_string(),
        }
    }
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self::stardew_defaults(
            "/data/data/io.stardew/files",
            "/data/data/io.stardew/files/home",
        )
    }
}
