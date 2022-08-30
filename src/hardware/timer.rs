#![allow(dead_code)]
use crate::{interrupts::Gate, machine::pit, print};

pub static mut WATCH: Watch = Watch::new();

pub struct Watch {
    multiplier: u64,
    counter: u16,
    pit: pit::Pit,
}

type Miliseconds = u64;

impl Watch {
    pub const fn new() -> Watch {
        Watch {
            multiplier: 0,
            counter: 0,
            pit: pit::Pit::new(),
        }
    }

    /// Converts the given amount of miliseconds to the needed.
    // TODO: Make units type-safe.
    pub const fn miliseconds_to_pit_cycles(miliseconds: Miliseconds) -> u64 {
        miliseconds * 1_000_000 / pit::NANOSECONDS_PER_CYCLE as u64
    }

    /// Sets the associated PIT to the given amount of miliseconds.
    pub fn set(&mut self, interval: Miliseconds) {
        let total_cycles = Watch::miliseconds_to_pit_cycles(interval);

        let (cycles, multipler) = match total_cycles > pit::MAXIMUM_CYCLES as u64 {
            true => {
                let multiplier = (total_cycles / pit::MAXIMUM_CYCLES as u64) + 1;
                (total_cycles / multiplier, multiplier)
            }
            false => (total_cycles, 0),
        };

        self.multiplier = multipler;
        self.pit.set(
            pit::Counter::Counter0,
            pit::Mode::PeriodicInterrupt,
            pit::CountingFormat::Binary,
            cycles as u16,
        )
    }

    /// Signal to the watch that another cycle has passed.
    ///
    /// Returns true if the desired interval has passed. If so, the internal
    /// counter is reset.
    pub fn tick(&mut self) -> bool {
        self.counter += 1;
        match self.multiplier {
            _ if self.multiplier == self.counter as u64 => {
                self.counter = 0;
                true
            }
            0 => true,
            _ => false,
        }
    }
}

pub static TIMER_GATE: Gate = Gate {
    prologue: timer_prologue,
    epilogue: timer_epilogue,
};

fn timer_prologue() -> bool {
    unsafe { WATCH.tick() }
}

fn timer_epilogue() {
    print!(".");
}
