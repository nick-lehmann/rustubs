use x86_64::structures::idt::{HandlerFunc, InterruptStackFrame};

use super::guardian;

pub static INTERRUPT_HANDLERS: [HandlerFunc; 32] = [
    handle_interrupt_32,
    handle_interrupt_33,
    handle_interrupt_34,
    handle_interrupt_35,
    handle_interrupt_36,
    handle_interrupt_37,
    handle_interrupt_38,
    handle_interrupt_39,
    handle_interrupt_40,
    handle_interrupt_41,
    handle_interrupt_42,
    handle_interrupt_43,
    handle_interrupt_44,
    handle_interrupt_45,
    handle_interrupt_46,
    handle_interrupt_47,
    handle_interrupt_48,
    handle_interrupt_49,
    handle_interrupt_50,
    handle_interrupt_51,
    handle_interrupt_52,
    handle_interrupt_53,
    handle_interrupt_54,
    handle_interrupt_55,
    handle_interrupt_56,
    handle_interrupt_57,
    handle_interrupt_58,
    handle_interrupt_59,
    handle_interrupt_60,
    handle_interrupt_61,
    handle_interrupt_62,
    handle_interrupt_63,
];

extern "x86-interrupt" fn handle_interrupt_32(_stack: InterruptStackFrame) {
    guardian(32);
}

extern "x86-interrupt" fn handle_interrupt_33(_stack: InterruptStackFrame) {
    guardian(33);
}

extern "x86-interrupt" fn handle_interrupt_34(_stack: InterruptStackFrame) {
    guardian(34);
}

extern "x86-interrupt" fn handle_interrupt_35(_stack: InterruptStackFrame) {
    guardian(35);
}

extern "x86-interrupt" fn handle_interrupt_36(_stack: InterruptStackFrame) {
    guardian(36);
}

extern "x86-interrupt" fn handle_interrupt_37(_stack: InterruptStackFrame) {
    guardian(37);
}

extern "x86-interrupt" fn handle_interrupt_38(_stack: InterruptStackFrame) {
    guardian(38);
}

extern "x86-interrupt" fn handle_interrupt_39(_stack: InterruptStackFrame) {
    guardian(39);
}

extern "x86-interrupt" fn handle_interrupt_40(_stack: InterruptStackFrame) {
    guardian(40);
}

extern "x86-interrupt" fn handle_interrupt_41(_stack: InterruptStackFrame) {
    guardian(41);
}

extern "x86-interrupt" fn handle_interrupt_42(_stack: InterruptStackFrame) {
    guardian(42);
}

extern "x86-interrupt" fn handle_interrupt_43(_stack: InterruptStackFrame) {
    guardian(43);
}

extern "x86-interrupt" fn handle_interrupt_44(_stack: InterruptStackFrame) {
    guardian(44);
}

extern "x86-interrupt" fn handle_interrupt_45(_stack: InterruptStackFrame) {
    guardian(45);
}

extern "x86-interrupt" fn handle_interrupt_46(_stack: InterruptStackFrame) {
    guardian(46);
}

extern "x86-interrupt" fn handle_interrupt_47(_stack: InterruptStackFrame) {
    guardian(47);
}

extern "x86-interrupt" fn handle_interrupt_48(_stack: InterruptStackFrame) {
    guardian(48);
}

extern "x86-interrupt" fn handle_interrupt_49(_stack: InterruptStackFrame) {
    guardian(49);
}

extern "x86-interrupt" fn handle_interrupt_50(_stack: InterruptStackFrame) {
    guardian(50);
}

extern "x86-interrupt" fn handle_interrupt_51(_stack: InterruptStackFrame) {
    guardian(51);
}

extern "x86-interrupt" fn handle_interrupt_52(_stack: InterruptStackFrame) {
    guardian(52);
}

extern "x86-interrupt" fn handle_interrupt_53(_stack: InterruptStackFrame) {
    guardian(53);
}

extern "x86-interrupt" fn handle_interrupt_54(_stack: InterruptStackFrame) {
    guardian(54);
}

extern "x86-interrupt" fn handle_interrupt_55(_stack: InterruptStackFrame) {
    guardian(55);
}

extern "x86-interrupt" fn handle_interrupt_56(_stack: InterruptStackFrame) {
    guardian(56);
}

extern "x86-interrupt" fn handle_interrupt_57(_stack: InterruptStackFrame) {
    guardian(57);
}

extern "x86-interrupt" fn handle_interrupt_58(_stack: InterruptStackFrame) {
    guardian(58);
}

extern "x86-interrupt" fn handle_interrupt_59(_stack: InterruptStackFrame) {
    guardian(59);
}

extern "x86-interrupt" fn handle_interrupt_60(_stack: InterruptStackFrame) {
    guardian(60);
}

extern "x86-interrupt" fn handle_interrupt_61(_stack: InterruptStackFrame) {
    guardian(61);
}

extern "x86-interrupt" fn handle_interrupt_62(_stack: InterruptStackFrame) {
    guardian(62);
}

extern "x86-interrupt" fn handle_interrupt_63(_stack: InterruptStackFrame) {
    guardian(63);
}
