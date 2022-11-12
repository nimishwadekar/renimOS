#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

mod prelude;
mod spinlock;
mod serial;
mod display;
mod arch;
mod interrupts;

#[cfg(test)]
mod test;

#[cfg(not(test))]
entry_point!(kernel_main);
#[cfg(test)]
entry_point!(test::test_kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    serial_println!("Serial init");

    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| crate::prelude::serial_panic("No framebuffer")); // hang if no framebuffer.
    display::init(fb);
    display::clear_screen();
    kprintln!("Display init");

    arch::init();

    kprintln_with_colour!(display::Colour::OK, "Eden initialisation completed");
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    kprintln!("{}", info);
    loop {}
}