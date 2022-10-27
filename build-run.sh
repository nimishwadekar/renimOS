# Pass "bios" or "uefi" as argument for firmware option.

if [ "$#" -ne 1 ]; then
    echo "Exactly one argument expected."
    exit 1
fi

firmware=$1
if [ "$firmware" = "bios" ]; then
    cargo build --target=x86_64-eden.json -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
    cargo run -p boot
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-eden/debug/boot-bios-eden.img
    exit 0
fi

if [ "$firmware" = "uefi" ]; then
    cargo build --target=x86_64-eden.json -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
    cargo run -p boot
    qemu-system-x86_64 -bios boot/OVMF.fd -drive format=raw,file=target/x86_64-eden/debug/boot-uefi-eden.img
    exit 0
fi

echo "Argument can only be 'bios' or 'uefi'."
exit 1