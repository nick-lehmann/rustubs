#![allow(dead_code)]

use x86::irq::{disable as disable_interrupts, enable as enable_interrupts};

use super::Gate;
use crate::utils::ring_buffer::RingBuffer;

pub static mut LEVEL1: Level1 = Level1::new();
static LEVEL1_GUARD: spin::Mutex<()> = spin::Mutex::new(());

pub type Level1Lock<'a> = spin::MutexGuard<'a, ()>;

// WARN: Not really, but required for now!
unsafe impl Sync for Level1 {}

pub struct Level1 {
    epilogues: RingBuffer<Gate, 100>,
}

impl Level1 {
    pub const fn new() -> Self {
        Self {
            epilogues: RingBuffer::new(),
        }
    }

    /// Regular control flow leaves the critical section. Accumulated
    /// epilogues are processed.
    ///
    /// Invoke while interrupts are still disabled.
    pub fn leave(&mut self) {
        let mut epilogue = self.epilogues.pop();

        while let Some(f) = epilogue {
            unsafe { enable_interrupts() };
            (f.epilogue)();
            unsafe { disable_interrupts() };
            epilogue = self.epilogues.pop();
        }
    }

    /// This method is called by guardian () if the previously executed prologue
    /// has indicated by a return value of true that its epilogue should be
    /// executed. Whether the epilogue is handled immediately or only added to
    /// the epilogue queue depends on whether the critical section is free or
    /// occupied.
    pub fn relay(&mut self, gate: &'static Gate) {
        match LEVEL1_GUARD.try_lock() {
            Some(_) => {
                unsafe { enable_interrupts() };
                (gate.epilogue)();
            }
            None => {
                // TODO: Check if already queued.
                self.epilogues.push(gate);
            }
        }
    }
}
