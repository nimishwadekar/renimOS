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

pub fn serial_panic(msg: &str) -> ! {
    crate::serial_println!("{}ABORT{}: {}", crate::serial::RED, crate::serial::RESET_COLOUR, msg);
    loop {}
}