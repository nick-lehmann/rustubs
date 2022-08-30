use super::cmd::{AsPitCommand, PitCommands};

pub enum Counter {
    Counter0,
    Counter1,
    Counter2,
}

impl AsPitCommand for Counter {
    fn as_pit_command(&self) -> PitCommands {
        match self {
            Counter::Counter0 => PitCommands::Counter0,
            Counter::Counter1 => PitCommands::Counter1,
            Counter::Counter2 => PitCommands::Counter2,
        }
    }
}
