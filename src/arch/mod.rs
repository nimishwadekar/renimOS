#[cfg(feature = "x86_64")]
pub mod x86_64;

#[cfg(feature = "x86_64")]
pub fn init() {
    x86_64::init()
}

#[inline]
pub fn disable_interrupts() {
    #[cfg(feature = "x86_64")]
    x86_64::interrupts::disable();
}

#[inline]
pub fn enable_interrupts() {
    #[cfg(feature = "x86_64")]
    x86_64::interrupts::enable();
}

#[inline]
pub fn are_interrupts_enabled() -> bool {
    #[cfg(feature = "x86_64")]
    x86_64::interrupts::are_enabled()
}

#[inline]
pub fn halt_cpu() {
    #[cfg(feature = "x86_64")]
    x86_64::hlt();
}