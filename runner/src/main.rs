fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    
    // choose whether to start the UEFI or BIOS image
    let mut args = std::env::args();
    args.next();
    let uefi = args.next().filter(|e| e.as_str() == "--uefi").is_some();

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    cmd.arg("-serial").arg("stdio");

    #[cfg(feature = "test")]
    {
        cmd.arg("-device").arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
        cmd.arg("-display").arg("none");
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}