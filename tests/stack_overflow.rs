#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use rustubs::{exit_qemu, gdt::tss::DOUBLE_FAULT_IST_INDEX, serial_print, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_handle_double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

extern "x86-interrupt" fn test_handle_double_fault(
    _stack_frame: InterruptStackFrame,
    _code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(rustubs::QemuExitCode::Success);

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...");

    rustubs::gdt::init();
    IDT.load();

    stack_overflow();

    panic!("Executing continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();

    // Prevent the compiler from optimizing away the recursion.
    volatile::ReadOnly::new(0).read();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rustubs::test_panic_handler(info)
}
