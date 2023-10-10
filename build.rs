use std::path::PathBuf;

fn main() {
    dbg!(std::env::vars_os());
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    #[cfg(not(feature = "test"))]
    let key = "CARGO_BIN_FILE_RENIM_KERNEL";
    #[cfg(feature = "test")]
    let key = "CARGO_BIN_FILE_RENIM_KERNEL_TEST_renim-kernel";

    let kernel = PathBuf::from(std::env::var_os(key).unwrap());

    // create an UEFI disk image
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel).create_disk_image(&uefi_path).unwrap();

    // create a BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel).create_disk_image(&bios_path).unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}