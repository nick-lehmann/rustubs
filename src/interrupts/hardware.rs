use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

impl From<InterruptIndex> for usize {
    fn from(i: InterruptIndex) -> usize {
        i as usize
    }
}

impl From<InterruptIndex> for u8 {
    fn from(i: InterruptIndex) -> Self {
        i as u8
    }
}

pub extern "x86-interrupt" fn handle_timer_interrupt(_stack: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}
