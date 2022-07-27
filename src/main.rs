#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod cga;

/// The linker expects the `_start()` symbol. Hence, we disable name mangling to
/// make sure the compiler does not change the name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = cga::Writer::default();

    writer.set_color(cga::ColorCode::new(cga::Color::Green, cga::Color::Black));

    writer.write_string("Hellö brave new world! 1\n");
    writer.write_string("Hellö brave new world! 2\n");
    writer.write_string("Hellö brave new world! 3\n");
    writer.write_string("Hellö brave new world! 4\n");
    writer.write_string("Hellö brave new world! 5\n");
    writer.write_string("Hellö brave new world! 6\n");
    writer.write_string("Hellö brave new world! 7\n");
    writer.write_string("Hellö brave new world! 8\n");
    writer.write_string("Hellö brave new world! 9\n");
    writer.write_string("Hellö brave new world! 10\n");

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
