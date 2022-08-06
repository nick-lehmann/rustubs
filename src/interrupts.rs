use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt::tss::DOUBLE_FAULT_IST_INDEX, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(handle_breakpoint);
        idt.double_fault.set_handler_fn(handle_double_fault);
        unsafe {
            idt.divide_error
                .set_handler_fn(handle_divide_error)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// x86 does not allow to return from a double fault handler
extern "x86-interrupt" fn handle_double_fault(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handle_divide_error(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

/// If the function executes until the end, we know that the interrupt handlings
/// still works.
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
