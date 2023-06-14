GCC_COMPILER=riscv32-unknown-elf-gcc
GCC_OBJCOPY=riscv32-unknown-elf-objcopy

all: bin

assemble:
	$(GCC_COMPILER) -S test.c

build:
	$(GCC_COMPILER) -Wl,-Ttext=0x0 -nostdlib -march=rv64i -mabi=lp64 -o test test.S

bin: build
	$(GCC_OBJCOPY) -O binary test test.bin

clean:
	rm -f test test.bin

