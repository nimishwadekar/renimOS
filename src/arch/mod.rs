pub mod interrupts;

pub trait InterruptFunctions {
    fn disable();
    fn enable();
    fn are_enabled() -> bool;
}