/// Represents the prologue and epilogue of an interrupt handler.
pub struct Gate {
    pub prologue: fn() -> bool,
    pub epilogue: fn(),
}
