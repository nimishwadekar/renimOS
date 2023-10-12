// This module is only compiled for tests.

use crate::{serial, serial_println, serial_print, prelude::halt_cpu_and_loop};

use crate::test_framework::__UnitTest;

//====================================================================
// STRUCTURES
//====================================================================

struct TestRunner {
    tests: &'static [&'static __UnitTest],
}

//====================================================================
// IMPLEMENTATIONS
//====================================================================

impl TestRunner {
    /// Loads the array of test locations from the boot info.
    fn new(boot_info: &bootloader_api::BootInfo) -> Self {
        let tests: &[&__UnitTest] = unsafe { core::slice::from_raw_parts(
            boot_info.init_array_addr.into_option().unwrap() as usize as *const &__UnitTest,
            boot_info.init_array_len as usize / core::mem::size_of::<*const __UnitTest>()
        ) };
        Self { tests }
    }

    fn run_and_exit_qemu(&self) -> ! {
        let tests = self.tests;
        serial_println!("\n*************** TESTS ***************\n");
        serial_println!("Running {} test{}", tests.len(), if tests.len() == 1 {""} else {"s"});
        for test in tests {
            serial_print!("{}...  ", test.name);
            (test.func)();
            serial_println!("{}[ok]{}", serial::GREEN, serial::RESET_COLOUR);
        }

        exit_qemu(QemuExitCode::Success)
    }
}

//====================================================================
// FUNCTIONS
//====================================================================

#[no_mangle]
pub fn test_kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    crate::kernel_test_init(boot_info);
    let test_runner = TestRunner::new(boot_info);
    test_runner.run_and_exit_qemu();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::arch::disable_interrupts();
    serial_println!("{}[failed]{}\n", serial::RED, serial::RESET_COLOUR);
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    #[allow(unreachable_code)]
    {
        halt_cpu_and_loop();
    }
}

//====================================================================
// QEMU-SPECIFIC
//====================================================================

#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    panic!("Should have exited QEMU")
}