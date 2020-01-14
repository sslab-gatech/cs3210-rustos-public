ROOT := $(shell git rev-parse --show-toplevel)

export PATH := /usr/lib/llvm-8/bin:$(PATH)

CC := clang
AS := clang

TARGET := aarch64-unknown-none
CFLAGS := --target=$(TARGET) -Wall -O2 -mcpu=cortex-a53
ASFLAGS := $(CFLAGS)

OBJS := blinky.o init.o

.PHONY: all clean install objdump nm

all: blinky.bin

blinky.bin: blinky.elf
	llvm-objcopy -O binary $< $@

blinky.elf: $(OBJS) layout.ld
	ld.lld -m aarch64elf $(OBJS) -o $@ -Tlayout.ld

clean:
	rm -f *.o *.bin *.elf

install: blinky.elf
	$(ROOT)/bin/install-kernel.py $<

objdump:
	llvm-objdump -d -C blinky.elf

nm:
	llvm-nm blinky.elf

qemu:
	./qemu.sh blinky.elf -d in_asm
