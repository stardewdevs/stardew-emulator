pub fn encode_key(c: char) -> Vec<u8> {
    c.to_string().into_bytes()
}

pub fn encode_enter() -> Vec<u8> {
    vec![b'\r']
}

pub fn encode_backspace() -> Vec<u8> {
    vec![0x7f]
}

pub fn encode_tab() -> Vec<u8> {
    vec![b'\t']
}

pub fn encode_escape() -> Vec<u8> {
    vec![0x1b]
}

pub fn encode_ctrl(c: char) -> Vec<u8> {
    let lower = c.to_ascii_lowercase();
    let code = (lower as u8) & 0x1f;
    vec![code]
}

pub fn encode_arrow(up: bool, down: bool, left: bool, right: bool) -> Vec<u8> {
    if up {
        vec![0x1b, b'[', b'A']
    } else if down {
        vec![0x1b, b'[', b'B']
    } else if right {
        vec![0x1b, b'[', b'C']
    } else if left {
        vec![0x1b, b'[', b'D']
    } else {
        Vec::new()
    }
}
