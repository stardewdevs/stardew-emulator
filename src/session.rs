use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub active: bool,
    pub created_at: u128,
}

impl Session {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        Self {
            id: format!("{:x}", now),
            title: "Stardew".to_string(),
            active: true,
            created_at: now,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn close(&mut self) {
        self.active = false;
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
