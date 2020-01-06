ROOT := $(shell git rev-parse --show-toplevel)

KERN := blinky
KERN_DEBUG := target/aarch64-unknown-none/debug/${KERN}
KERN_RELEASE := target/aarch64-unknown-none/release/${KERN}

OBJCPY := cargo objcopy -- --strip-all -O binary

.PHONY: all debug release qemu objdump nm check clean install test

all: release

# e.g., xbuild bin opt
define xbuild
	@echo "+ Building build/$(KERN).elf [xbuild/$@]"
	@cargo xbuild $(2)
	@mkdir -p build
	@cp -f $(1) build/$(KERN).elf

	@echo "+ Building build/$(KERN).bin [objcopy]"
	@$(OBJCPY) $(1) build/$(KERN).bin
endef

debug:
	$(call xbuild,$(KERN_DEBUG))

release:
	$(call xbuild,$(KERN_RELEASE),--release)

check:
	@cargo xcheck

qemu:
	./qemu.sh build/$(KERN).elf -d in_asm

objdump:
	cargo objdump -- -disassemble -no-show-raw-insn -print-imm-hex build/$(KERN).elf

nm:
	cargo nm build/$(KERN).elf

clean:
	cargo clean
	rm -rf build

install:
	@echo "+ Installing build/$(KERN).elf [install-kernel.py]"
	@$(ROOT)/bin/install-kernel.py build/$(KERN).elf

test:
	cargo test --target=$(shell $(ROOT)/bin/get-host-target.sh)