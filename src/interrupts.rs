pub fn disable() {
    #[cfg(feature = "x86_64")]
    crate::arch::x86_64::interrupts::disable();
}

pub fn enable() {
    #[cfg(feature = "x86_64")]
    crate::arch::x86_64::interrupts::enable();
}

pub fn are_enabled() -> bool {
    #[cfg(feature = "x86_64")]
    crate::arch::x86_64::interrupts::are_enabled()
}