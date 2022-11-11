use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{arch::InterruptFunctions, kprintln, init_guard};

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

    /// Initialises the IDT.
    fn init() {
        init_guard!();

        static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
        unsafe {
            IDT.breakpoint.set_handler_fn(breakpoint_handler);
            IDT.load();
        }
    }
}

// Interrupt Handlers

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

//================================================
//  UNIT TESTS
//================================================

#[test_case]
fn breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}