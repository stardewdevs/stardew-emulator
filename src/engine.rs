use crate::snapshot::{Cell, Color, TerminalMode, TerminalSnapshot};
use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Column, Line, Point};
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::{Config, Term, TermMode};
use alacritty_terminal::vte::ansi::{Color as AnsiColor, NamedColor, Processor, StdSyncHandler};
use std::sync::mpsc::{channel, Receiver, Sender};

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
    event_receiver: Receiver<Event>,
    cols: usize,
    rows: usize,
    title: String,
}

impl Engine {
    pub fn new(cols: usize, rows: usize) -> Self {
        let (sender, event_receiver) = channel();
        let event_proxy = EventProxy { sender };
        let config = Config::default();
        let dimensions = TermDimensions { cols, rows };
        let term = Term::new(config, &dimensions, event_proxy);
        let processor = Processor::new();

        Self {
            term,
            processor,
            event_receiver,
            cols,
            rows,
            title: String::new(),
        }
    }

    pub fn process(&mut self, data: &[u8]) {
        for byte in data {
            self.processor.advance(&mut self.term, *byte);
        }
        self.drain_events();
    }

    fn drain_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::Title(title) => {
                    self.title = title;
                }
                Event::ResetTitle => {
                    self.title.clear();
                }
                _ => {}
            }
        }
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

        let cursor = grid.cursor.point;
        let cursor_col = cursor.column.0;
        let cursor_row = cursor.line.0.max(0) as usize;

        let mode_flags = self.term.mode();
        let mode = TerminalMode {
            alt_screen: mode_flags.contains(TermMode::ALT_SCREEN),
            cursor_visible: mode_flags.contains(TermMode::SHOW_CURSOR),
            bracketed_paste: mode_flags.contains(TermMode::BRACKETED_PASTE),
            app_cursor_keys: mode_flags.contains(TermMode::APP_CURSOR),
            app_keypad: mode_flags.contains(TermMode::APP_KEYPAD),
            mouse_reporting: mode_flags.intersects(
                TermMode::MOUSE_REPORT_CLICK
                    | TermMode::MOUSE_DRAG
                    | TermMode::MOUSE_MOTION
                    | TermMode::SGR_MOUSE,
            ),
        };

        TerminalSnapshot {
            cols: self.cols,
            rows: self.rows,
            cells,
            cursor_col,
            cursor_row,
            mode,
            scroll_offset: grid.display_offset(),
            title: self.title.clone(),
        }
    }

    pub fn term(&self) -> &Term<EventProxy> {
        &self.term
    }

    pub fn term_mut(&mut self) -> &mut Term<EventProxy> {
        &mut self.term
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

fn convert_cell(cell: &alacritty_terminal::term::cell::Cell) -> Cell {
    let flags = cell.flags;
    let inverse = flags.contains(Flags::INVERSE);

    let mut fg = convert_color(cell.fg);
    let mut bg = convert_color(cell.bg);

    if inverse {
        std::mem::swap(&mut fg, &mut bg);
    }

    Cell {
        ch: cell.c,
        fg,
        bg,
        bold: flags.contains(Flags::BOLD),
        italic: flags.contains(Flags::ITALIC),
        underline: flags.contains(Flags::UNDERLINE),
        strikethrough: flags.contains(Flags::STRIKEOUT),
        inverse,
    }
}

fn convert_color(color: AnsiColor) -> Color {
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

fn named_color_to_rgb(color: NamedColor) -> (u8, u8, u8) {
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
        NamedColor::DimBlack => (0, 0, 0),
        NamedColor::DimRed => (140, 30, 30),
        NamedColor::DimGreen => (8, 125, 80),
        NamedColor::DimYellow => (150, 150, 10),
        NamedColor::DimBlue => (24, 76, 133),
        NamedColor::DimMagenta => (125, 42, 125),
        NamedColor::DimCyan => (11, 112, 137),
        NamedColor::DimWhite => (150, 150, 150),
        NamedColor::Foreground => (255, 255, 255),
        NamedColor::Background => (0, 0, 0),
        NamedColor::Cursor => (255, 255, 255),
    }
}

fn indexed_color_to_rgb(index: u8) -> (u8, u8, u8) {
    if index < 16 {
        let named = match index {
            0 => NamedColor::Black,
            1 => NamedColor::Red,
            2 => NamedColor::Green,
            3 => NamedColor::Yellow,
            4 => NamedColor::Blue,
            5 => NamedColor::Magenta,
            6 => NamedColor::Cyan,
            7 => NamedColor::White,
            8 => NamedColor::BrightBlack,
            9 => NamedColor::BrightRed,
            10 => NamedColor::BrightGreen,
            11 => NamedColor::BrightYellow,
            12 => NamedColor::BrightBlue,
            13 => NamedColor::BrightMagenta,
            14 => NamedColor::BrightCyan,
            _ => NamedColor::BrightWhite,
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

pub struct TermDimensions {
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
