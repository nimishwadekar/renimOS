#[macro_export]
macro_rules! init_guard {
    () => {{
        use core::sync::atomic::{AtomicBool, Ordering};
        static mut INIT_GUARD: AtomicBool = AtomicBool::new(false);
        if let Err(..) = unsafe { INIT_GUARD.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) } {
            panic!("FATAL: Double init()");
        }
    }};
}

pub fn serial_panic(info: &str) -> ! {
    crate::arch::disable_interrupts();
    crate::serial_println!("{}ABORT{}: {}", crate::serial::RED, crate::serial::RESET_COLOUR, info);
    loop {}
}

#[inline]
pub fn halt_cpu_and_loop() -> ! {
    loop { crate::arch::halt_cpu(); }
}