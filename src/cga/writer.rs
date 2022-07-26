use super::{
    buffer::{Buffer, Position, CGA_BUFFER_ADDRESS, CGA_BUFFER_WIDTH},
    characters::DisplayChar,
    Color, ColorCode,
};

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

                self.buffer.chars[row][col] = DisplayChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.position.offset += 1;
            }
        }
    }

    /// Writes a complete string to the current position of the CGA buffer.
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn new_line(&mut self) {
        todo!()
    }
}
