use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::gate::Gate;
use super::traps;
use super::{handlers::INTERRUPT_HANDLERS, pic::PICS};
use crate::{gdt, interrupts::pic::PICLine};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

lazy_static! {
    /// Global `plugbox` instance.
    pub static ref PLUGBOX: spin::Mutex<Plugbox> = {
        let plugbox = Plugbox::new();
        spin::Mutex::new(plugbox)
    };
}

const NUMBER_OF_GATES: usize = 32;
type Gates = [Option<&'static Gate>; NUMBER_OF_GATES];

/// The Plugbox is a collection of interrupt handlers.
///
/// It keeps track of all the registered interrupt handlers and also manages the
/// Interrupt Descriptor Table (IDT).
pub struct Plugbox {
    pub gates: Gates,
}

impl Plugbox {
    pub fn new() -> Self {
        let mut pics = PICS.lock();

        unsafe {
            pics.initialize();
            pics.disable();
        };

        Plugbox::setup_exceptions();
        let gates = Plugbox::initialize_gates();

        Plugbox { gates }
    }

    /// Exceptions are signals from the CPU that something has gone wrong.
    ///
    /// Exceptions are categorized into the following:
    /// - Faults: These can be corrected and the program may continue as if
    ///   nothing happened.
    /// - Traps: Traps are reported immediately after the execution of the
    ///   trapping instruction.
    /// - Aborts: Some severe unrecoverable error.
    ///
    /// Source: https://wiki.osdev.org/Exceptions
    ///
    /// These are part of RUSTUBS. Hence, the user cannot configure them.
    fn setup_exceptions() {
        unsafe {
            IDT.breakpoint.set_handler_fn(traps::handle_breakpoint);

            IDT.double_fault
                .set_handler_fn(traps::handle_double_fault)
                .set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX);
        }
    }

    /// Set handlers which all refer to the guardian function.
    fn initialize_gates() -> Gates {
        for i in 32..32 + NUMBER_OF_GATES {
            unsafe {
                IDT[i].set_handler_fn(INTERRUPT_HANDLERS[i - 32]);
            }
        }

        [None; NUMBER_OF_GATES]
    }

    pub fn load(&self) {
        unsafe {
            IDT.load();
        }
    }

    fn allow_pic_line(&self, line: &PICLine) {
        let mut pics = PICS.lock();
        let mut masks = unsafe { pics.read_masks() };

        let i: usize = line.clone().into();
        let pic_index: usize = i / 8;
        masks[pic_index] &= !(1 << (i % 8)); // Unset correct bit.

        unsafe { pics.write_masks(masks[0], masks[1]) };
    }

    pub fn assign(&mut self, line: PICLine, gate: &'static Gate) {
        self.allow_pic_line(&line);

        let index: usize = line.into();
        self.gates[index] = Some(gate);
    }
}
