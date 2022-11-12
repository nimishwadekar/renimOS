use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{kprintln, init_guard};

pub fn disable() {
    x86_64::instructions::interrupts::disable();
}

pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn are_enabled() -> bool {
    x86_64::instructions::interrupts::are_enabled()
}

/// Initialises the IDT.
pub fn init_idt() {
    init_guard!();

    static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault.set_handler_fn(double_fault_handler).set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        IDT.load();
    }
}

// Interrupt Handlers

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

//================================================
//  UNIT TESTS
//================================================

mod tests {
    #[test_case]
    fn breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
}