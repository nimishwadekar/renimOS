#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use display::init_display;

mod spinlock;
mod serial;
mod display;
mod arch;
mod interrupts;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("Hello Eden!");
    serial_println!("{:#?}", boot_info);

    if !init_display(boot_info.framebuffer.as_ref()) {
        serial_println!("Framebuffer not initialised");
    }

    println!("Hello kernel");

    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}