#[macro_export]
macro_rules! init_guard {
    () => {
        static mut INIT_GUARD: bool = false;
        if unsafe{ INIT_GUARD } { panic!("FATAL: Double init()"); }
        else { unsafe { INIT_GUARD = true; } }
    };
}

pub fn serial_panic(msg: &str) -> ! {
    crate::serial_println!("{}ABORT{}: {}", crate::serial::RED, crate::serial::RESET_COLOUR, msg);
    loop {}
}