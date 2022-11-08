#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

mod spinlock;
mod serial;
mod display;
mod arch;
mod interrupts;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| loop {}); // hang if no framebuffer.

    display::init(fb);
    kprintln!("Hello Eden!");

    serial::init();
    serial_println!("Hello Eden!");
    serial_println!("{:#?}", boot_info);

    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}