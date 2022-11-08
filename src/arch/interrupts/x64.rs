use crate::arch::InterruptFunctions;

pub struct X64;

impl InterruptFunctions for X64 {
    fn disable() {
        x86_64::instructions::interrupts::disable();
    }

    fn enable() {
        x86_64::instructions::interrupts::enable();
    }

    fn are_enabled() -> bool {
        x86_64::instructions::interrupts::are_enabled()
    }
}