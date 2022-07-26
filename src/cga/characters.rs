use super::ColorCode;

#[repr(C)]
pub struct DisplayChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}
