# Makefile for Rust project with nightly builds and Raspberry Pi BCM2837 entry

# Configuration
CARGO = cargo +nightly
RUST_TARGET = aarch64-unknown-none
AS = aarch64-none-elf-as
LD = aarch64-none-elf-ld
QEMU = qemu-system-aarch64
BUILD_DIR = output

# Cargo Outputs directories #
OUTPUT_DIR_DEBUG = target/$(RUST_TARGET)/debug
OUTPUT_DIR_RELEASE = target/$(RUST_TARGET)/release

# Cargo Outputs files names
KERNEL_LIB_NAME = libkernel.rlib

# Cargo Outputs files names #
KERNEL_FULL_PATH_DEBUG = $(OUTPUT_DIR_DEBUG)/kernel
KERNEL_FULL_PATH_RELEASE = $(OUTPUT_DIR_RELEASE)/kernel
KERNEL_DEBUG_IMG = $(BUILD_DIR)/kernel8.img
KERNEL_RELEASE_IMG = $(BUILD_DIR)/kernel-release8.img

# Make sure output dir exists
$(shell mkdir -p $(BUILD_DIR))

# Default target
.PHONY: all
all: build

.PHONY: build
build:
	$(CARGO) rustc --target $(RUST_TARGET) --manifest-path kernel/Cargo.toml -- --emit=obj
	aarch64-none-elf-objcopy -O binary $(KERNEL_FULL_PATH_DEBUG) $(KERNEL_DEBUG_IMG)
	cp $(KERNEL_FULL_PATH_DEBUG) $(BUILD_DIR)/kernel.elf

.PHONY: build-release
build-release:
	$(CARGO) rustc --release --target $(RUST_TARGET) --manifest-path kernel/Cargo.toml -- --emit=obj
	aarch64-none-elf-objcopy -O binary $(KERNEL_FULL_PATH_RELEASE) $(KERNEL_RELEASE_IMG)
	cp $(KERNEL_FULL_PATH_RELEASE) $(BUILD_DIR)/kernel-release.elf

.PHONY: run
run: build
	$(QEMU) -M raspi3b \
	-kernel $(KERNEL_IMG) \
	-semihosting-config enable=on,target=native \
	-serial none -serial mon:stdio \
	-display none \
	-cpu cortex-a53 \
	-S -gdb tcp::9999 \

.PHONY: run-release
run-release: build-release
	$(QEMU) -M raspi3b \
	-kernel $(KERNEL_IMG) \
	-serial none -serial mon:stdio \
	-display none \
	-cpu cortex-a53 \

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(BUILD_DIR)/*