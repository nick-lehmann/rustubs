use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::{interrupts::PICS, print};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

const KEYBOARD_DATA_PORT: u16 = 0x60;

pub extern "x86-interrupt" fn handle_keyboard_interrupt(_stack: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();

    let mut data_port = Port::new(KEYBOARD_DATA_PORT);

    let scancode: u8 = unsafe { data_port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
