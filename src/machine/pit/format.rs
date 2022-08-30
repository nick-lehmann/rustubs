use super::cmd::{AsPitCommand, PitCommands};

pub enum CountingFormat {
    Binary,
    /// Binary Coded Decimals
    Bcd,
}

impl AsPitCommand for CountingFormat {
    fn as_pit_command(&self) -> super::cmd::PitCommands {
        match self {
            CountingFormat::Binary => PitCommands::Binary,
            CountingFormat::Bcd => PitCommands::BCD,
        }
    }
}
