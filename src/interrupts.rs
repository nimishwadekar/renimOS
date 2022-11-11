use crate::arch::InterruptFunctions;

pub fn disable() {
    crate::arch::interrupts::x86_64::disable();
}

pub fn enable() {
    crate::arch::interrupts::x86_64::enable();
}

pub fn are_enabled() -> bool {
    crate::arch::interrupts::x86_64::are_enabled()
}

pub fn init() {
    crate::arch::interrupts::x86_64::init();
}