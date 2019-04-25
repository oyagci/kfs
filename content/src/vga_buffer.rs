#![allow(unused)]

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    c: u8,
    color: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col: usize,
    row: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        let new_color = ColorCode::new(fg, bg);

        self.color = new_color;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            127 => {
                if self.col > 0 {
                    self.col -= 1;
                }
                else if self.row > 0 {
                    self.row -= 1;
                    self.col = BUFFER_WIDTH - 1;
                }
                self.buffer.chars[self.row][self.col].write(ScreenChar {
                    c: b' ',
                    color: self.color,
                });
            }
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.row][self.col].write(ScreenChar {
                    c: byte,
                    color: self.color,
                });
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                0x20...0x7f | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe),
            }
        }
        Cursor::set_pos((self.row * 80 + self.col) as u16);
    }

    fn new_line(&mut self) {
        if self.row >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let c = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(c)
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.col = 0;
        } else {
            self.row += 1;
            self.col = 0;
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            c: b' ',
            color: self.color,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

pub fn set_global_color(fg: Color, bg: Color) {
    WRITER.lock().set_color(fg, bg);
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use crate::utils::outb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor(u16);

impl Cursor {
    pub fn inc(amount: u16) {
        let pos = CURSOR.lock().0 + amount;
        Cursor::set_pos(pos);
    }

    pub fn new_line() {
        let mut pos = CURSOR.lock().0;

        if pos % 80 == 0 {
            pos += 80;
        } else {
            while pos % 80 != 0 {
                pos += 1;
            }
        }
        Cursor::set_pos(pos);
    }

    fn set_pos(pos: u16) {
        CURSOR.lock().0 = pos;

        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, (pos >> 8) as u8);
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col: 0,
        row: 0,
        color: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    pub static ref CURSOR: Mutex<Cursor> = Mutex::new(Cursor(0));
}

#[macro_export]
macro_rules! println {
	() => (print!("\n"));
	($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
