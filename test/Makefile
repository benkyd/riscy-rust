TARGET=test
GCC_PREFIX=riscv32-unknown-elf

CFLAGS:=-fno-stack-protector
CFLAGS+=-static-libgcc -fdata-sections -ffunction-sections
CFLAGS+=-g -O0 -march=rv32ia -mabi=ilp32 -static
LDFLAGS:= -T flatfile.lds -nostdlib

all: build

assemble:
	$(GCC_PREFIX)-gcc -o $(TARGET) $(TARGET).c $(CFLAGS) $(LDFLAGS)
	$(GCC_PREFIX)-objdump -t $(TARGET) > $(TARGET).map
	$(GCC_PREFIX)-objdump -d -M no-aliases $(TARGET) > $(TARGET).lst
	$(GCC_PREFIX)-objcopy -O binary $(TARGET) $(TARGET).bin
	$(GCC_PREFIX)-objcopy -O ihex $(TARGET) $(TARGET).hex

build:
	$(GCC_PREFIX)-gcc -o $(TARGET) $(TARGET).S $(CFLAGS) $(LDFLAGS)
	$(GCC_PREFIX)-objdump -t $(TARGET) > $(TARGET).map
	$(GCC_PREFIX)-objdump -d -M no-aliases $(TARGET) > $(TARGET).lst
	$(GCC_PREFIX)-objcopy -O binary $(TARGET) $(TARGET).bin
	$(GCC_PREFIX)-objcopy -O ihex $(TARGET) $(TARGET).hex

clean:
	rm -f $(TARGET) $(TARGET).bin

