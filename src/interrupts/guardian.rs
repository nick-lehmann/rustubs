use super::{PICS, PLUGBOX};
use crate::println;

/// Invoked for every interrupt.
///
/// Provides the skeleton that is common among every handling of an interrupt.
// TODO: Enqueue epilogues.
// TODO: Better error handling?
// TODO: Enable interrupts around epilogues.
pub fn guardian(slot: usize) {
    let plugbox = PLUGBOX.lock();

    match plugbox.gates[slot - 32] {
        Some(gate) => {
            if (gate.prologue)() {
                (gate.epilogue)();
            }
        }
        None => println!("No handler for slot {}", slot),
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
