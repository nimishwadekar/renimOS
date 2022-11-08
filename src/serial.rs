use uart_16550::SerialPort;
use crate::spinlock::SpinLock;

static mut SERIAL1: Option<SpinLock<SerialPort>> = None;

fn serial1() -> &'static SpinLock<SerialPort> {
    // Can't print an error as serial port not initialised. So loop forever.
    unsafe { SERIAL1.as_ref() }.expect("Serial port not initialised.")
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    serial1().lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

pub fn init() {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    unsafe { SERIAL1 = Some(SpinLock::new(serial_port)); }
}