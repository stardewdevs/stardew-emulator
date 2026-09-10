pub fn encode_char(c: char) -> Vec<u8> {
    let mut buf = [0u8; 4];
    c.encode_utf8(&mut buf).as_bytes().to_vec()
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
    let lower = c.to_ascii_lowercase() as u8;
    if lower.is_ascii_lowercase() {
        vec![lower - b'a' + 1]
    } else if c == ' ' {
        vec![0]
    } else {
        Vec::new()
    }
}

pub fn encode_arrow_up() -> Vec<u8> {
    vec![0x1b, b'[', b'A']
}

pub fn encode_arrow_down() -> Vec<u8> {
    vec![0x1b, b'[', b'B']
}

pub fn encode_arrow_right() -> Vec<u8> {
    vec![0x1b, b'[', b'C']
}

pub fn encode_arrow_left() -> Vec<u8> {
    vec![0x1b, b'[', b'D']
}

pub fn encode_home() -> Vec<u8> {
    vec![0x1b, b'[', b'H']
}

pub fn encode_end() -> Vec<u8> {
    vec![0x1b, b'[', b'F']
}

pub fn encode_page_up() -> Vec<u8> {
    vec![0x1b, b'[', b'5', b'~']
}

pub fn encode_page_down() -> Vec<u8> {
    vec![0x1b, b'[', b'6', b'~']
}

pub fn encode_delete() -> Vec<u8> {
    vec![0x1b, b'[', b'3', b'~']
}

pub fn encode_insert() -> Vec<u8> {
    vec![0x1b, b'[', b'2', b'~']
}

pub fn encode_f1() -> Vec<u8> {
    vec![0x1b, b'O', b'P']
}

pub fn encode_f2() -> Vec<u8> {
    vec![0x1b, b'O', b'Q']
}

pub fn encode_f3() -> Vec<u8> {
    vec![0x1b, b'O', b'R']
}

pub fn encode_f4() -> Vec<u8> {
    vec![0x1b, b'O', b'S']
}

pub fn encode_f5() -> Vec<u8> {
    vec![0x1b, b'[', b'1', b'5', b'~']
}

pub fn encode_f6() -> Vec<u8> {
    vec![0x1b, b'[', b'1', b'7', b'~']
}

pub fn encode_f7() -> Vec<u8> {
    vec![0x1b, b'[', b'1', b'8', b'~']
}

pub fn encode_f8() -> Vec<u8> {
    vec![0x1b, b'[', b'1', b'9', b'~']
}

pub fn encode_f9() -> Vec<u8> {
    vec![0x1b, b'[', b'2', b'0', b'~']
}

pub fn encode_f10() -> Vec<u8> {
    vec![0x1b, b'[', b'2', b'1', b'~']
}

pub fn encode_f11() -> Vec<u8> {
    vec![0x1b, b'[', b'2', b'3', b'~']
}

pub fn encode_f12() -> Vec<u8> {
    vec![0x1b, b'[', b'2', b'4', b'~']
}
