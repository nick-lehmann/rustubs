use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

use crate::{interrupts::Gate, print};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

/// Port for reading scancodes from the keyboard.
static mut KEYBOARD_DATA_PORT: Port<u8> = Port::new(0x60);

pub static KEYBOARD_GATE: Gate = Gate {
    prologue: keyboard_prologue,
    epilogue: keyboard_epilogue,
};

/// Simple buffer to save the current key pressed.
///
/// The keyboard prologue will save the key pressed in this buffer.
/// After that, the epilogue will read the key from this buffer.
static mut KEYBOARD_BUFFER: u8 = 0x00;

/// The prologue of the keyboard interrupt handler.
fn keyboard_prologue() -> bool {
    unsafe { KEYBOARD_BUFFER = KEYBOARD_DATA_PORT.read() };
    true
}

/// The epilogue of the keyboard interrupt handler.
fn keyboard_epilogue() {
    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = unsafe { KEYBOARD_BUFFER };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}
