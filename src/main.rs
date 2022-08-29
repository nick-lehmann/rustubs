#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustubs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rustubs::{clear, init, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Invoke this first to set all color codes in the CGA buffer.
    // Otherwise, the cursor which is always one cell ahead of the most recent byte,
    // will not show.
    clear!();

    println!("Hello brave new World!");

    init();

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustubs::test_panic_handler(info)
}
