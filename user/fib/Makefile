ROOT := $(shell git rev-parse --show-toplevel)

BIN := $(shell basename $(shell realpath .))
TARGET := target/aarch64-unknown-none/release/$(BIN)
OBJCPY := cargo objcopy -- --strip-all -O binary

.PHONY: all build qemu objdump nm clean

all: build

build:
	@echo "+ Building build/$(BIN).elf [xbuild/$@]"
	@cargo xbuild --release
	@mkdir -p build
	@cp -f $(TARGET) build/$(BIN).elf

	@echo "+ Building build/$(BIN).bin [objcopy]"
	@$(OBJCPY) $(TARGET) build/$(BIN).bin

check:
	@cargo xcheck

objdump: build
	cargo objdump -- -disassemble -no-show-raw-insn -print-imm-hex build/$(BIN).elf

nm: build
	cargo nm build/$(BIN).elf

clean:
	cargo clean
	rm -rf build
