#![no_std]
#![no_main]
#![feature(type_ascription)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

// use cga::buffer::Position;

mod cga;
mod serial;

/// The linker expects the `_start()` symbol. Hence, we disable name mangling to
/// make sure the compiler does not change the name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    cga::writer::WRITER
        .lock()
        .set_color(cga::ColorCode::new(cga::Color::Green, cga::Color::Black));

    // for i in 0..10 {
    //     println!("Hello world {}", i);
    // }

    // cga::cursor::set_cursor_position(Position { line: 5, offset: 3 });

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called whenever a panic occurs.
///
/// The `!` return type marks this function as `diverging`, meaning that it
/// never returns. `PanicInfo` gives us some insight into where the panic has
/// happened. For now, we can not use this information at all.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn sample() -> u8 {
    1
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, sample());
}
