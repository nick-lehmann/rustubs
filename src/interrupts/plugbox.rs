use x86_64::structures::idt::{HandlerFunc, InterruptDescriptorTable};

use super::traps;
use super::{handlers::INTERRUPT_HANDLERS, pic::PICS};
use crate::println;
use crate::{gdt, interrupts::pic::PICLine};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

type Gate = HandlerFunc;

pub fn guardian(slot: usize) {
    println!("Handle interrupt {}", slot);

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}

pub struct Plugbox {
    handlers: [Option<Gate>; 256],
}

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

        // Set handlers which all refer to the guardian function.
        for i in 32..64 {
            unsafe {
                IDT[i].set_handler_fn(INTERRUPT_HANDLERS[i - 32]);
            }
        }

        Plugbox {
            handlers: [None; 256],
        }
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
        let index = 32 + offset;

        self.handlers[index] = Some(gate);

        self.allow_pic_line(&line);
    }
}
