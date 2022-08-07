use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PICLine {
    Timer = 0,
    Keyboard = 1,
}

impl From<PICLine> for usize {
    fn from(i: PICLine) -> usize {
        i as usize
    }
}

impl From<PICLine> for u8 {
    fn from(i: PICLine) -> Self {
        i as u8
    }
}
