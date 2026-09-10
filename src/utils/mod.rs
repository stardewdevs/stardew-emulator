pub fn bytes_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}
