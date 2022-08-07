use x86_64::structures::idt::{HandlerFunc, InterruptDescriptorTable};

use super::pic::PICS;
use super::traps;
use crate::{gdt, interrupts::pic::PICLine};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

type Gate = HandlerFunc;

pub struct Plugbox {}

impl Plugbox {
    pub fn new() -> Self {
        let mut pics = PICS.lock();

        unsafe {
            pics.initialize();
            // Disable all interrupts by default.
            // pics.write_masks(0xff, 0xff);
        };

        unsafe {
            IDT.breakpoint.set_handler_fn(traps::handle_breakpoint);

            IDT.double_fault
                .set_handler_fn(traps::handle_double_fault)
                .set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX); // new
        }

        Plugbox {}
    }

    pub fn load(&self) {
        unsafe {
            IDT.load();
        }
    }

    pub fn assign(&mut self, index: PICLine, gate: Gate) {
        let offset: usize = index.into();

        unsafe {
            IDT[32 + offset].set_handler_fn(gate);
            // IDT.load();
        }
        // let mut pics = PICS.lock();

        // let mut masks = unsafe { pics.read_masks() };

        // let i: usize = index.into();
        // let pic_index: usize = i / 8;
        // masks[pic_index] |= 1 << (i % 8);

        // masks[0] = 0xFF;
        // masks[1] = 0xFF;

        // unsafe { pics.write_masks(masks[0], masks[1]) };
    }
}
