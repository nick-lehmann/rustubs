use super::ColorCode;

const UNKNOWN_CHARACTER_REPLACEMENT: u8 = 0xfe;

/// Represents a character in the "Code page 437"
///
/// Source: https://en.wikipedia.org/wiki/Code_page_437
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighASCII(u8);

impl HighASCII {
    /// Checks that the given byte is part of the "Code page 437". If it's not,
    /// it will be mapped to the unknown character ■.
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            // printable ASCII byte or newline
            0x20..=0x7e | b'\n' => HighASCII(byte),
            // not part of printable ASCII range
            _ => HighASCII(UNKNOWN_CHARACTER_REPLACEMENT),
        }
    }
}

// TODO: Custom Display implementation?
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DisplayChar {
    pub character: HighASCII,
    pub color_code: ColorCode,
}
