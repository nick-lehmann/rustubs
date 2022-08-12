use x86_64::structures::idt::InterruptStackFrame;

use crate::{interrupts::PICS, print};

pub extern "x86-interrupt" fn handle_timer_interrupt(_stack: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
}
