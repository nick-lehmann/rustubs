use super::types::{IRQId};
use super::{guard::LEVEL1, PICS, PLUGBOX};
use crate::println;

/// Invoked for every interrupt request.
///
/// Provides the skeleton that is common among every handling of an interrupt.
// TODO: Better error handling?
pub fn guardian(irq_id: IRQId) {
    let plugbox = PLUGBOX.lock();

    match plugbox.gates[irq_id as usize - 32] {
        Some(gate) => {
            if (gate.prologue)() {
                unsafe { LEVEL1.relay(gate) };
            }
        }
        None => println!("No handler for slot {}", irq_id),
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
