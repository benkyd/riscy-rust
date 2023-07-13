TARGET=test
GCC_PREFIX=riscv32-unknown-elf

all: build

assemble:
	$(GCC_PREFIX)-gcc -S $(TARGET).c

build:
	$(GCC_PREFIX)-gcc -Wl,-Ttext=0x0 -nostdlib -march=rv64i -mabi=lp64 -o $(TARGET) $(TARGET).S
	$(GCC_PREFIX)-objdump -t $(TARGET) > $(TARGET).map
	$(GCC_PREFIX)-objdump -d -M no-aliases $(TARGET) > $(TARGET).lst
	$(GCC_PREFIX)-objcopy -O binary $(TARGET) $(TARGET).bin
	$(GCC_PREFIX)-objcopy -O ihex $(TARGET) $(TARGET).hex

clean:
	rm -f $(TARGET) $(TARGET).bin

