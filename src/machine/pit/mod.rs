#![allow(dead_code)]
mod cmd;
mod counter;
mod format;
mod mode;

pub use counter::Counter;
pub use format::CountingFormat;
pub use mode::Mode;
use x86_64::instructions::port::Port;

use self::cmd::AsPitCommand;

pub type Cycles = u16;

/// Frequency of the PIT.
///
/// This is the frequency of the original PC. It is derived from the base
/// frequency of oscillator chip used in american television back then. This
/// chip was used in computer because it was cheap and widely available.
pub const FREQUENCY: f32 = 1.19318e6;

/// Number of nanoseconds it takes the PIT to do a single cycle.
///
/// Helper constant to calculate the number of cycles needed for a given
/// interval.
pub const NANOSECONDS_PER_CYCLE: u32 = (1e9 / FREQUENCY) as u32;

pub const MAXIMUM_CYCLES: Cycles = u16::MAX;

pub struct Pit {
    control_port: Port<u8>,
    counter0: Port<u8>,
}

impl Pit {
    /// Initialize the Pit.
    // TODO: Allow usage of multiple pits.
    pub const fn new() -> Pit {
        Pit {
            control_port: Port::new(0x43),
            counter0: Port::new(0x40),
        }
    }

    /// Sets the frequency and other parameters for a Pit counter.
    pub fn set(&mut self, counter: Counter, mode: Mode, format: CountingFormat, cycles: Cycles) {
        let cmd = counter.as_pit_command() | mode.as_pit_command() | format.as_pit_command();
        unsafe { self.control_port.write(cmd.bits()) }

        unsafe {
            self.counter0.write(cycles as u8);
            self.counter0.write((cycles >> 8) as u8);
        }
    }
}
