pub fn disable() {
    crate::arch::x86_64::interrupts::disable();
}

pub fn enable() {
    crate::arch::x86_64::interrupts::enable();
}

pub fn are_enabled() -> bool {
    crate::arch::x86_64::interrupts::are_enabled()
}

pub fn init() {
    crate::arch::x86_64::interrupts::init();
}