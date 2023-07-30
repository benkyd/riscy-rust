#!/bin/sh
(
BINARIES_DIR="${0%/*}/"
cd ${BINARIES_DIR}

if [ "${1}" = "serial-only" ]; then
    EXTRA_ARGS='-nographic'
else
    EXTRA_ARGS=''
fi

export PATH="/home/benk/dprog/misc/mini-rv32ima/buildroot/output/host/bin:${PATH}"
exec qemu-system-riscv32 -M virt -bios none -kernel Image -append "rootwait root=/dev/vda ro" -drive file=rootfs.ext2,format=raw,id=hd0 -device virtio-blk-device,drive=hd0 -nographic -cpu rv32,mmu=off  ${EXTRA_ARGS}
)
