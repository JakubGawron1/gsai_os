qemu-system-x86_64^
    -m 4G^
    -serial stdio^
    -machine q35^
    -cpu qemu64^
    -smp 2^
    -bios ./ovmf/OVMF-pure-efi.fd^
    -drive format=raw,file=fat:rw:./hdd/image/^
    -drive if=none,format=raw,id=disk,file=./hdd/rootfs.img^
    -device ahci,id=ahci^
    -device ide-hd,drive=disk,bus=ahci.0^
    -drive if=none,format=raw,id=nvm,file=./hdd/nvme.img^
    -device nvme,drive=nvm,serial=deadbeef^
    -net none^
    -no-shutdown^
    -no-reboot^
    -monitor stdio^