mod handlers;
mod pic;
mod plugbox;
mod traps;

pub use pic::{PICLine, PICS};
pub use plugbox::{guardian, Plugbox};

/// If the function executes until the end, we know that the interrupt handlings
/// still works.
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
