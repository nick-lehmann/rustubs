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
            pics.disable();
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

    fn allow_pic_line(&self, line: &PICLine) {
        let mut pics = PICS.lock();
        let mut masks = unsafe { pics.read_masks() };

        let i: usize = line.clone().into();
        let pic_index: usize = i / 8;
        masks[pic_index] &= !(1 << (i % 8)); // Unset correct bit.

        unsafe { pics.write_masks(masks[0], masks[1]) };
    }

    pub fn assign(&mut self, line: PICLine, gate: Gate) {
        let offset: usize = line.clone().into();
        unsafe {
            IDT[32 + offset].set_handler_fn(gate);
        }

        self.allow_pic_line(&line);
    }
}
