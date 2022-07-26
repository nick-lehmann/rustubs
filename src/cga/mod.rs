#![allow(dead_code)]
/// IBM CGA specification: http://minuszerodegrees.net/oa/OA%20-%20IBM%20Color%20Graphics%20Monitor%20Adapter%20(CGA).pdf
mod color;

pub use color::{Color, ColorCode};

const CGA_BUFFER_HEIGHT: usize = 25;
const CGA_BUFFER_WIDTH: usize = 80;
const CGA_BUFFER_ADDRESS: usize = 0xb8000;

#[repr(C)]
pub struct DisplayChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub struct Buffer {
    buffer: [[DisplayChar; CGA_BUFFER_WIDTH]; CGA_BUFFER_HEIGHT],
}

pub struct Position {
    line: usize,
    offset: usize,
}

pub struct Writer {
    position: Position,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Default for Writer {
    fn default() -> Self {
        Writer {
            position: Position { line: 0, offset: 0 },
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(CGA_BUFFER_ADDRESS as *mut Buffer) },
        }
    }
}

impl Writer {
    /// Writes a single byte to the current position of the CGA buffer.
    /// Afterwards, it updates the current position.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.position.offset >= CGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.position.line;
                let col = self.position.offset;

                self.buffer.buffer[row][col] = DisplayChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.position.offset += 1;
            }
        }
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn new_line(&mut self) {
        todo!()
    }
}
