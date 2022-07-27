use super::{
    buffer::{Buffer, Position, CGA_BUFFER_ADDRESS, CGA_BUFFER_HEIGHT, CGA_BUFFER_WIDTH},
    characters::{DisplayChar, HighASCII},
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

                self.buffer.chars[row][col].write(DisplayChar {
                    character: HighASCII::from_byte(byte),
                    color_code: self.color_code,
                });

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

    /// Sets the color code used for all subsequent writes.
    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    /// Clears all characters on the given line by replacing it with blanks in
    /// the current color code.
    pub fn clear_line(&mut self, line: usize) {
        for col in 0..CGA_BUFFER_WIDTH {
            self.buffer.chars[line][col].write(DisplayChar {
                character: HighASCII::from_byte(b' '),
                color_code: self.color_code,
            });
        }
    }

    /// Scrolls a single line up.
    // TODO: Copy whole rows instead of the single charactdrs.
    pub fn scroll(&mut self) {
        for line in 1..CGA_BUFFER_HEIGHT {
            self.clear_line(line - 1);
            for col in 0..CGA_BUFFER_WIDTH {
                self.buffer.chars[line - 1][col].write(self.buffer.chars[line][col].read());
            }
        }

        self.clear_line(CGA_BUFFER_HEIGHT - 1);
    }

    /// Moves the current position to the next beginning of the next line and
    /// scrolls the whole screen if necessary.
    pub fn new_line(&mut self) {
        self.position.offset = 0;

        if self.position.line == CGA_BUFFER_HEIGHT - 1 {
            self.scroll();
            return;
        }

        self.position.line += 1;
    }
}
