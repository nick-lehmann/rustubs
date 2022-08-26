use super::{guard::LEVEL1, PICS, PLUGBOX};
use crate::println;

/// Invoked for every interrupt.
///
/// Provides the skeleton that is common among every handling of an interrupt.
// TODO: Better error handling?
pub fn guardian(slot: usize) {
    let plugbox = PLUGBOX.lock();

    match plugbox.gates[slot - 32] {
        Some(gate) => {
            if (gate.prologue)() {
                unsafe { LEVEL1.relay(gate) };
            }
        }
        None => println!("No handler for slot {}", slot),
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
