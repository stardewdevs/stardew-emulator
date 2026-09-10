use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TerminalMode {
    pub alt_screen: bool,
    pub cursor_visible: bool,
    pub bracketed_paste: bool,
    pub app_cursor_keys: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalSnapshot {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Cell>,
    pub cursor_col: usize,
    pub cursor_row: usize,
    pub mode: TerminalMode,
    pub scroll_offset: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color { r: 255, g: 255, b: 255, a: 255 },
            bg: Color { r: 0, g: 0, b: 0, a: 255 },
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
        }
    }
}

impl Default for TerminalMode {
    fn default() -> Self {
        Self {
            alt_screen: false,
            cursor_visible: true,
            bracketed_paste: false,
            app_cursor_keys: false,
        }
    }
}
