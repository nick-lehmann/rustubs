#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod cga;

/// The linker expects the `_start()` symbol. Hence, we disable name mangling to
/// make sure the compiler does not change the name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    cga::WRITER
        .lock()
        .set_color(cga::ColorCode::new(cga::Color::Green, cga::Color::Black));

    for i in 0..10 {
        println!("Hello world {}", i);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called whenever a panic occurs.
///
/// The `!` return type marks this function as `diverging`, meaning that it
/// never returns. `PanicInfo` gives us some insight into where the panic has
/// happened. For now, we can not use this information at all.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
