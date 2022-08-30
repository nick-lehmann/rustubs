use super::cmd::{AsPitCommand, PitCommands};

pub enum Mode {
    SingleInterrupt,
    SingleInterruptHardwareControl,
    PeriodicInterrupt,
    SquareWaveSignalGenerator,
    SoftwareControlledInterrupt,
    HardwareControlledInterrupt,
}

impl AsPitCommand for Mode {
    fn as_pit_command(&self) -> PitCommands {
        match self {
            Mode::SingleInterrupt => PitCommands::SingleInterrupt,
            Mode::SingleInterruptHardwareControl => PitCommands::SingleInterruptHardwareControl,
            Mode::PeriodicInterrupt => PitCommands::PeriodicInterrupt,
            Mode::SquareWaveSignalGenerator => PitCommands::SquareWaveSignalGenerator,
            Mode::SoftwareControlledInterrupt => PitCommands::SoftwareControlledInterrupt,
            Mode::HardwareControlledInterrupt => PitCommands::HardwareControlledInterrupt,
        }
    }
}
