#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, BootInfo};

use crate::prelude::halt_cpu_and_loop;

mod prelude;
mod spinlock;
mod serial;
mod display;
mod arch;

#[cfg(test)]
mod test;

#[link_section = ".text.startup"]
fn init() -> i32 {
    1234
}

#[link_section = ".init_array"]
static INIT: fn() -> i32 = init;

#[link_section = ".init_array"]
static INIT2: u64 = 5;

#[cfg(not(ftest))]
#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    serial_println!("Serial init");
    //serial_println!("{:x?} -> {:x?} = {}", &INIT as *const fn() -> i32, INIT as usize, INIT());
    serial_println!("{:x?}", boot_info.ramdisk_len);
    let inits = unsafe { core::slice::from_raw_parts(boot_info.ramdisk_len as usize as *const usize, 1) };
    serial_println!("{:x?}", inits[0]);
    let func = inits[0] as *const ();
    serial_println!("{:x?}", func);
    let func: fn()->i32 = unsafe { core::mem::transmute(func) };
    serial_println!("{:x?}", func);
    serial_println!("{}", (func)());

    //serial_println!("{}", );

    halt_cpu_and_loop();

    let fb = boot_info.framebuffer.as_ref().unwrap_or_else(|| crate::prelude::serial_panic("No framebuffer")); // hang if no framebuffer.
    display::init(fb);
    display::clear_screen();
    kprintln!("Display init");

    arch::init();

    kprintln_with_colour!(display::Colour::OK, "renim initialisation completed");
    
    halt_cpu_and_loop();
}

#[cfg(ftest)]
#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    serial_println!("[TEST] Serial init");
    halt_cpu_and_loop();
}


#[cfg(not(test))]
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

#[cfg(not(test))]
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);
#[cfg(test)]
entry_point!(test::test_kernel_main, config = &BOOTLOADER_CONFIG);