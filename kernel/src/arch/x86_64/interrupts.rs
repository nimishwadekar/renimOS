use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{kprintln, init_guard, kprint};
use super::pic;

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
        IDT[pic::InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        IDT[pic::InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        IDT.load();
    }
}

//================================================
//  INTERRUPT HANDLERS
//================================================

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    kprint!(".");
    pic::end_of_interrupt(pic::InterruptIndex::Timer);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use crate::spinlock::SpinLock;
    use x86_64::instructions::port::Port;

    static KEYBOARD: SpinLock<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        SpinLock::new(Keyboard::new(HandleControl::Ignore));

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => kprint!("{}", character),
                DecodedKey::RawKey(key) => kprint!("{:?}", key),
            }
        }
    }

    pic::end_of_interrupt(pic::InterruptIndex::Keyboard);
}

//================================================
//  UNIT TESTS
//================================================

/* mod tests {
    #[test_case]
    fn breakpoint_exception() {
        // invoke a breakpoint exception
        x86_64::instructions::interrupts::int3();
    }
} */