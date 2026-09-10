use crate::error::EmulatorError;
use crate::snapshot::{Cell, Color, TerminalMode, TerminalSnapshot};
use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Column, Line, Point};
use alacritty_terminal::term::cell::Cell as AlacrittyCell;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::vte::ansi::{Processor, StdSyncHandler};
use std::sync::mpsc::{channel, Sender};

#[derive(Clone)]
pub struct EventProxy {
    sender: Sender<Event>,
}

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}

pub struct Engine {
    term: Term<EventProxy>,
    processor: Processor<StdSyncHandler>,
    cols: usize,
    rows: usize,
}

impl Engine {
    pub fn new(cols: usize, rows: usize) -> Self {
        let (sender, _receiver) = channel();
        let event_proxy = EventProxy { sender };
        let config = Config::default();
        let dimensions = TermDimensions { cols, rows };
        let term = Term::new(config, &dimensions, event_proxy);
        let processor = Processor::new();

        Self {
            term,
            processor,
            cols,
            rows,
        }
    }

    pub fn process(&mut self, data: &[u8]) {
        self.processor.advance(&mut self.term, data);
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        let dimensions = TermDimensions { cols, rows };
        self.term.resize(dimensions);
        self.cols = cols;
        self.rows = rows;
    }

    pub fn snapshot(&self) -> TerminalSnapshot {
        let grid = self.term.grid();
        let mut cells = Vec::with_capacity(self.cols * self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let line = Line(row as i32);
                let column = Column(col);
                let point = Point::new(line, column);
                let cell = &grid[point];
                cells.push(convert_cell(cell));
            }
        }

        let cursor_point = self.term.grid().cursor.point;
        let cursor_col = cursor_point.column.0;
        let cursor_row = cursor_point.line.0 as usize;

        let mode = TerminalMode {
            alt_screen: self.term.mode().contains(alacritty_terminal::term::TermMode::ALT_SCREEN),
            cursor_visible: self.term.mode().contains(alacritty_terminal::term::TermMode::SHOW_CURSOR),
            bracketed_paste: self.term.mode().contains(alacritty_terminal::term::TermMode::BRACKETED_PASTE),
            app_cursor_keys: self.term.mode().contains(alacritty_terminal::term::TermMode::APP_CURSOR),
        };

        TerminalSnapshot {
            cols: self.cols,
            rows: self.rows,
            cells,
            cursor_col,
            cursor_row,
            mode,
            scroll_offset: self.term.grid().display_offset(),
        }
    }

    pub fn term(&self) -> &Term<EventProxy> {
        &self.term
    }

    pub fn term_mut(&mut self) -> &mut Term<EventProxy> {
        &mut self.term
    }
}

fn convert_cell(cell: &AlacrittyCell) -> Cell {
    let fg = convert_color(cell.fg);
    let bg = convert_color(cell.bg);
    let flags = cell.flags;

    Cell {
        ch: cell.c,
        fg,
        bg,
        bold: flags.contains(alacritty_terminal::term::cell::Flags::BOLD),
        italic: flags.contains(alacritty_terminal::term::cell::Flags::ITALIC),
        underline: flags.contains(alacritty_terminal::term::cell::Flags::UNDERLINE),
        strikethrough: flags.contains(alacritty_terminal::term::cell::Flags::STRIKEOUT),
    }
}

fn convert_color(color: alacritty_terminal::vte::ansi::Color) -> Color {
    use alacritty_terminal::vte::ansi::Color as AnsiColor;
    match color {
        AnsiColor::Named(named) => {
            let (r, g, b) = named_color_to_rgb(named);
            Color { r, g, b, a: 255 }
        }
        AnsiColor::Spec(rgb) => Color {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 255,
        },
        AnsiColor::Indexed(index) => {
            let (r, g, b) = indexed_color_to_rgb(index);
            Color { r, g, b, a: 255 }
        }
    }
}

fn named_color_to_rgb(color: alacritty_terminal::vte::ansi::NamedColor) -> (u8, u8, u8) {
    use alacritty_terminal::vte::ansi::NamedColor;
    match color {
        NamedColor::Black => (0, 0, 0),
        NamedColor::Red => (205, 49, 49),
        NamedColor::Green => (13, 188, 121),
        NamedColor::Yellow => (229, 229, 16),
        NamedColor::Blue => (36, 114, 200),
        NamedColor::Magenta => (188, 63, 188),
        NamedColor::Cyan => (17, 168, 205),
        NamedColor::White => (229, 229, 229),
        NamedColor::BrightBlack => (102, 102, 102),
        NamedColor::BrightRed => (241, 76, 76),
        NamedColor::BrightGreen => (35, 209, 139),
        NamedColor::BrightYellow => (245, 245, 67),
        NamedColor::BrightBlue => (59, 142, 234),
        NamedColor::BrightMagenta => (214, 112, 214),
        NamedColor::BrightCyan => (41, 184, 219),
        NamedColor::BrightWhite => (255, 255, 255),
        _ => (255, 255, 255),
    }
}

fn indexed_color_to_rgb(index: u8) -> (u8, u8, u8) {
    if index < 16 {
        let named = match index {
            0 => alacritty_terminal::vte::ansi::NamedColor::Black,
            1 => alacritty_terminal::vte::ansi::NamedColor::Red,
            2 => alacritty_terminal::vte::ansi::NamedColor::Green,
            3 => alacritty_terminal::vte::ansi::NamedColor::Yellow,
            4 => alacritty_terminal::vte::ansi::NamedColor::Blue,
            5 => alacritty_terminal::vte::ansi::NamedColor::Magenta,
            6 => alacritty_terminal::vte::ansi::NamedColor::Cyan,
            7 => alacritty_terminal::vte::ansi::NamedColor::White,
            8 => alacritty_terminal::vte::ansi::NamedColor::BrightBlack,
            9 => alacritty_terminal::vte::ansi::NamedColor::BrightRed,
            10 => alacritty_terminal::vte::ansi::NamedColor::BrightGreen,
            11 => alacritty_terminal::vte::ansi::NamedColor::BrightYellow,
            12 => alacritty_terminal::vte::ansi::NamedColor::BrightBlue,
            13 => alacritty_terminal::vte::ansi::NamedColor::BrightMagenta,
            14 => alacritty_terminal::vte::ansi::NamedColor::BrightCyan,
            _ => alacritty_terminal::vte::ansi::NamedColor::BrightWhite,
        };
        named_color_to_rgb(named)
    } else if index < 232 {
        let i = index - 16;
        let r = (i / 36) * 51;
        let g = ((i % 36) / 6) * 51;
        let b = (i % 6) * 51;
        (r, g, b)
    } else {
        let v = (index - 232) * 10 + 8;
        (v, v, v)
    }
}

struct TermDimensions {
    cols: usize,
    rows: usize,
}

impl Dimensions for TermDimensions {
    fn total_lines(&self) -> usize {
        self.rows
    }

    fn screen_lines(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.cols
    }
}
