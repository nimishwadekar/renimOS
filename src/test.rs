// This module is only compiled for tests.

use crate::{serial, serial_println, serial_print, display, interrupts};

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...  ", core::any::type_name::<T>());
        self();
        serial_println!("{}[ok]{}", serial::GREEN, serial::RESET_COLOUR);
    }
}

#[no_mangle]
pub fn test_kernel_main(boot_info: &'static mut bootloader::BootInfo) -> ! {
    serial::init();
    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| serial_panic("No framebuffer")); // hang if no framebuffer.
    display::init(fb);
    interrupts::init();

    crate::test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\n*************** TESTS ***************\n");
    serial_println!("Running {} test{}", tests.len(), if tests.len() == 1 {""} else {"s"});
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn serial_panic(info: &str) -> ! {
    crate::arch::disable_interrupts();
    serial_println!("{}[failed]{}\n", serial::RED, serial::RESET_COLOUR);
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::arch::disable_interrupts();
    serial_println!("{}[failed]{}\n", serial::RED, serial::RESET_COLOUR);
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Qemu-specific

#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}