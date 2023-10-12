#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, BootInfo};

use crate::prelude::*;

mod prelude;
mod spinlock;
mod serial;
mod display;
mod arch;

#[cfg(testf)]
mod test_framework;


#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    serial_println!("Serial init");

    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| serial_panic("No framebuffer")); // hang if no framebuffer.
    display::init(fb);
    display::clear_screen();
    kprintln!("Display init");

    arch::init();

    kprintln_with_colour!(display::Colour::OK, "renim initialisation completed");
    
    halt_cpu_and_loop();
}

/// This function is called by the test main function to initialise the kernel for testing.
#[cfg(testf)]
fn kernel_test_init(boot_info: &'static BootInfo) {
    serial::init();
    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| serial_panic("No framebuffer")); // hang if no framebuffer.
    display::init(fb);
    arch::init();
}

#[cfg(not(testf))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    arch::disable_interrupts();
    serial_println!("{}", info);
    kprintln!("{}", info);
    halt_cpu_and_loop();
}

use bootloader_api::{BootloaderConfig, config::Mapping};

pub const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.framebuffer = Mapping::FixedAddress(0xFFFFFFFFB0000000);
    config.mappings.kernel_stack = Mapping::FixedAddress(0xFFFFFFFFFF000000);
    config.kernel_stack_size = 0x20000;
    config.mappings.boot_info = Mapping::FixedAddress(0xFFFFFFFFFFFFF000);
    config.mappings.dynamic_range_start = Some(0xFFFF_8000_0000_0000);
    config
};

#[cfg(not(testf))]
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);
#[cfg(testf)]
entry_point!(test_framework::test_kernel_main, config = &BOOTLOADER_CONFIG);