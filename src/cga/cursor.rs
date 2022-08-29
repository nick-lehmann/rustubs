use x86::io::outb;

use super::buffer::{Position, CGA_BUFFER_WIDTH};

const CURSOR_INDEX_PORT: u16 = 0x3d4;
const CURSOR_DATA_PORT: u16 = 0x3d5;

#[repr(u8)]
enum CursorCommands {
    SetHighByte = 14,
    SetLowByte = 15,
}

/// Set the blinking cursor to the given position.
pub fn set_cursor_position(position: &Position) {
    let pos = position.line * CGA_BUFFER_WIDTH + position.offset;
    let high_byte: u8 = (pos >> 8) as u8;
    let low_byte: u8 = (pos & 255) as u8;

    unsafe {
        outb(CURSOR_INDEX_PORT, CursorCommands::SetHighByte as u8);
        outb(CURSOR_DATA_PORT, high_byte);
        outb(CURSOR_INDEX_PORT, CursorCommands::SetLowByte as u8);
        outb(CURSOR_DATA_PORT, low_byte);
    };
}
