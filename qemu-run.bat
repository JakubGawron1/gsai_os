qemu-system-x86_64^
    -m 512M^
    -nographic^
    -bios ./ovmf/OVMF-pure-efi.fd^
    -drive format=raw,file=fat:rw:./image/