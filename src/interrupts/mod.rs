use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{gdt::tss::DOUBLE_FAULT_IST_INDEX, interrupts::hardware::InterruptIndex};

pub mod hardware;
mod traps;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(traps::handle_breakpoint);
        unsafe {
            idt.double_fault
                .set_handler_fn(traps::handle_double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.into()].set_handler_fn(hardware::handle_timer_interrupt);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

/// If the function executes until the end, we know that the interrupt handlings
/// still works.
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
