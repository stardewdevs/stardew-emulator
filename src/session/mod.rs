pub struct Session {
    pub id: String,
    pub title: String,
    pub active: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: uuid_simple(),
            title: "Stardew".to_string(),
            active: true,
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", now)
}
