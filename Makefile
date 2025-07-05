# Makefile for Rust project with nightly builds and Raspberry Pi BCM2837 entry

# Configuration
CARGO = cargo +nightly
RUST_TARGET = aarch64-unknown-none
AS = aarch64-none-elf-as
LD = aarch64-none-elf-ld
QEMU = qemu-system-aarch64
BUILD_DIR = output

# Assembly #
ASM_DIR = asm
ASM_SRC = $(wildcard $(ASM_DIR)/*.S)
ASM_OBJ = $(patsubst $(ASM_DIR)/%.S,$(BUILD_DIR)/%.o,$(ASM_SRC))

# Cargo Outputs directories #
OUTPUT_DIR_DEBUG = target/$(RUST_TARGET)/debug
OUTPUT_DIR_RELEASE = target/$(RUST_TARGET)/release

# Cargo Outputs files names
KERNEL_LIB_NAME = libkernel.rlib

# Cargo Outputs files names #
KERNEL_LIB_FULL_PATH_DEBUG = $(OUTPUT_DIR_DEBUG)/deps/kernel-*.o
KERNEL_LIB_FULL_PATH_RELEASE = $(OUTPUT_DIR_RELEASE)/deps/kernel-*.o

# Output images #
KERNEL = $(BUILD_DIR)/kernel.elf
KERNEL_RELEASE = $(BUILD_DIR)/kernel-release.elf
KERNEL_IMG = $(BUILD_DIR)/kernel8.img

# Linking #
LINKER_SCRIPT = linker.ld
LINKING_FLAGS = -nostdlib -g# -L/usr/local/lib -l:libgcc.a # -ffreestanding -shared
# In case linking against libgcc, flag is provided above, add in .cargo/config.toml too (rustflags) 
# The libgcc is in: /usr/local/lib/libgcc.a (from the arm-gnu toolchain)

# Make sure output dir exists
$(shell mkdir -p $(BUILD_DIR))

# Rule for assembling .S files for build/build-release
$(BUILD_DIR)/%.o: $(ASM_DIR)/%.S
	$(AS) -g $< -o $@

# Default target
.PHONY: all
all: build

.PHONY: build
build: $(ASM_OBJ)
	$(CARGO) rustc --target $(RUST_TARGET) --manifest-path kernel/Cargo.toml -- --emit=obj
	$(LD) -T $(LINKER_SCRIPT) $(LINKING_FLAGS) $(ASM_OBJ) $(KERNEL_LIB_FULL_PATH_DEBUG) -o $(KERNEL)
	aarch64-none-elf-objcopy -O binary $(KERNEL) $(KERNEL_IMG)

.PHONY: build-release
build-release: $(ASM_OBJ)
	$(CARGO) rustc --release --target $(RUST_TARGET) --manifest-path kernel/Cargo.toml -- --emit=obj
	$(LD) -T $(LINKER_SCRIPT) $(LINKING_FLAGS) $(ASM_OBJ) $(KERNEL_LIB_FULL_PATH_RELEASE) -o $(KERNEL_RELEASE)
	aarch64-none-elf-objcopy -O binary $(KERNEL_RELEASE) $(KERNEL_IMG)

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