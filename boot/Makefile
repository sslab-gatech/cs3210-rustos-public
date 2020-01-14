ROOT := $(shell git rev-parse --show-toplevel)

KERN := boot
TARGET := target/aarch64-unknown-none/release/${KERN}
OBJCPY := cargo objcopy -- --strip-all -O binary

.PHONY: all build qemu objdump nm check clean install test

all: build

build:
	@echo "+ Building build/$(KERN).elf [xbuild/$@]"
	@cargo xbuild --release
	@mkdir -p build
	@cp -f $(TARGET) build/$(KERN).elf

	@echo "+ Building build/$(KERN).bin [objcopy]"
	@$(OBJCPY) $(TARGET) build/$(KERN).bin

check:
	@cargo xcheck

qemu: build
	./qemu.sh build/$(KERN).elf

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
