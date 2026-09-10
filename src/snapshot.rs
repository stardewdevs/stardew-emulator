use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub inverse: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TerminalMode {
    pub alt_screen: bool,
    pub cursor_visible: bool,
    pub bracketed_paste: bool,
    pub app_cursor_keys: bool,
    pub app_keypad: bool,
    pub mouse_reporting: bool,
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
    pub title: String,
}

impl TerminalSnapshot {
    pub fn empty(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); cols * rows],
            cursor_col: 0,
            cursor_row: 0,
            mode: TerminalMode::default(),
            scroll_offset: 0,
            title: String::new(),
        }
    }

    pub fn cell(&self, col: usize, row: usize) -> Option<&Cell> {
        if col >= self.cols || row >= self.rows {
            return None;
        }
        self.cells.get(row * self.cols + col)
    }
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
            inverse: false,
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
            app_keypad: false,
            mouse_reporting: false,
        }
    }
}
