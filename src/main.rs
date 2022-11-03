#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

mod serial;
mod fonts;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    framebuffer.buffer_mut()[0] = 0xFF;
    framebuffer.buffer_mut()[1] = 0xFF;
    framebuffer.buffer_mut()[2] = 0xFF;
    framebuffer.buffer_mut()[3] = 0xFF;

    serial_println!("Hello Eden!");
    //serial_println!("{:#?}", boot_info);

    let psf = fonts::PSF::parse(include_bytes!("fonts/files/zap-light16.psf")).unwrap();
    serial_println!("psf loaded");

    loop {}
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}