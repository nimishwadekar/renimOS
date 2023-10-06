# Pass "bios" or "uefi" as argument for firmware option.

if [ "$#" -ne 1 ]; then
    echo "Exactly one argument expected."
    exit 1
fi

firmware=$1
if [ "$firmware" = "bios" ] || [ "$firmware" = "uefi" ]; then
    cargo build --release --target=x86_64-renimOS.json -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem && \
    cargo run -p boot && \

    cmd=( qemu-system-x86_64 )
    if [ "$firmware" = "bios" ]; then
        cmd+=( -drive format=raw,file=target/x86_64-renimOS/debug/boot-bios-renimOS.img )
    else
        cmd+=( -bios boot/OVMF.fd -drive format=raw,file=target/x86_64-renimOS/debug/boot-uefi-renimOS.img )
    fi

    # Rest of the (common) flags
    cmd+=( -serial stdio )

    "${cmd[@]}"

    exit "$?"
fi

echo "Argument can only be 'bios' or 'uefi'."
exit 1