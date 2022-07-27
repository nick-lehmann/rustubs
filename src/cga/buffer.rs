use volatile::Volatile;

use super::characters::DisplayChar;

pub const CGA_BUFFER_HEIGHT: usize = 25;
pub const CGA_BUFFER_WIDTH: usize = 80;
pub const CGA_BUFFER_ADDRESS: usize = 0xb8000;

// TODO: Make chars not public, use setter instead
pub struct Buffer {
    pub chars: [[Volatile<DisplayChar>; CGA_BUFFER_WIDTH]; CGA_BUFFER_HEIGHT],
}

pub struct Position {
    pub line: usize,
    pub offset: usize,
}
