# Pass "bios" or "uefi" as argument for firmware option.

if [ "$#" -ne 1 ]; then
    echo "Exactly one argument expected."
    exit 1
fi

firmware=$1
if [ "$firmware" = "bios" ] || [ "$firmware" = "uefi" ]; then
    cargo test --no-run --target=x86_64-renimOS.json -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem && \
    cargo run -p boot -- test && \

    cmd=( qemu-system-x86_64 )
    if [ "$firmware" = "bios" ]; then
        cmd+=( -drive format=raw,file=target/x86_64-renimOS/debug/deps/boot-bios-renimOS-462ec008b5ddf143.img )
    else
        cmd+=( -bios boot/OVMF.fd -drive format=raw,file=target/x86_64-renimOS/debug/deps/boot-uefi-renimOS-462ec008b5ddf143.img )
    fi

    # Rest of the (common) flags
    cmd+=( -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none )

    "${cmd[@]}"

    if [ "$?" -eq 33 ]; then
        exit 0
    else
        exit 1
    fi
fi

echo "Argument can only be 'bios' or 'uefi'."
exit 1