#![allow(non_upper_case_globals)]
use bitflags::bitflags;

bitflags! {
    pub struct PitCommands: u8 {
        // Counters
        const Counter0 = 0b00000000;
        const Counter1 = 0b01000000;
        const Counter2 = 0b10000000;

        // Read/Write
        const LatchCommand = 0b00000000;
        const LowOrderByteOnly = 0b00010000;
        const HighOrderByteOnly = 0b00100000;
        const BothBytes = 0b00110000;

        // Mode
        const SingleInterrupt = 0b00000000;
        const SingleInterruptHardwareControl = 0b00000010;
        const PeriodicInterrupt = 0b00000100;
        const SquareWaveSignalGenerator = 0b00000110;
        const SoftwareControlledInterrupt = 0b00001000;
        const HardwareControlledInterrupt = 0b00001010;

        // Counting Format
        const Binary = 0b00000000;
        const BCD = 0b00000001;
    }
}

pub trait AsPitCommand {
    fn as_pit_command(&self) -> PitCommands;
}
