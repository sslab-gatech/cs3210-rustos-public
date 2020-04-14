ROOT := $(shell git rev-parse --show-toplevel)

KERN := kernel
TARGET := target/aarch64-unknown-none/release/${KERN}
# SDCARD ?= $(ROOT)/ext/fat32-imgs/mock1.fat32.img
SDCARD ?= $(ROOT)/user/fs.img
OBJCPY := cargo objcopy -- --strip-all -O binary
TTY_PATH := /dev/ttyUSB0
QEMU_ARGS ?=

.PHONY: all build qemu transmit objdump nm check clean install test

all: build

build:
	@echo "+ Building build/$(KERN).elf [xbuild/$@]"
	@cargo xbuild --release
	@mkdir -p build
	@cp -f $(TARGET) build/$(KERN).elf

	@echo "+ Building build/$(KERN).bin [objcopy]"
	@$(OBJCPY) $(TARGET) build/$(KERN).bin

build-all:
	@(cd ../ext/uspi/lib; make)
	cp -f ../ext/uspi/lib/libuspi.a ./.cargo/
	@(cd ../ext/uspi/env/lib; make)
	cp -f ../ext/uspi/env/lib/libuspienv.a ./.cargo/
	touch src/main.rs
	make build

check:
	@cargo xcheck

qemu: build
	./qemu.sh build/$(KERN).bin -drive file=$(SDCARD),format=raw,if=sd $(QEMU_ARGS)

qemu-gdb: build
	./qemu.sh build/$(KERN).bin -drive file=$(SDCARD),format=raw,if=sd -s -S

qemu-asm: build
	./qemu.sh build/$(KERN).bin -drive file=$(SDCARD),format=raw,if=sd -d in_asm

transmit: build
	@echo "+ Transmitting build/$(KERN).bin to $(TTY_PATH)"
	ttywrite -i build/$(KERN).bin $(TTY_PATH)
	screen $(TTY_PATH) 115200

objdump: build
	cargo objdump -- -disassemble -no-show-raw-insn -print-imm-hex build/$(KERN).elf

nm: build
	cargo nm build/$(KERN).elf

clean:
	cargo clean
	rm -rf build

install: build
	@echo "+ Installing build/$(KERN).elf [install-kernel.py]"
	@$(ROOT)/bin/install-kernel.py build/$(KERN).elf

test:
	cargo test --target=$(shell $(ROOT)/bin/get-host-target.sh)
