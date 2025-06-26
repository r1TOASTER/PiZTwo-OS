# Makefile for Rust project with nightly builds and Raspberry Pi BCM2837 entry

# Configuration
BUILD_DIR = output
CARGO = cargo +nightly
AS = aarch64-none-elf-as
LD = aarch64-none-elf-ld
QEMU = qemu-system-aarch64
RUSTFLAGS = -C link-arg=-Tlinker.ld
ENTRY_SRC = entry/entry.S
ENTRY_OBJ = $(BUILD_DIR)/entry.o
LINKER_SCRIPT = linker.ld
OUTPUT_DIR = target/aarch64-unknown-none/debug
OUTPUT_DIR_RELEASE = target/aarch64-unknown-none/release
KERNEL = $(BUILD_DIR)/kernel.elf
KERNEL_RELEASE = $(BUILD_DIR)/kernel-release.elf
TEST_KERNEL = $(BUILD_DIR)/test-kernel.elf

# Default target
.PHONY: all
all: build

# Assemble entry.S file into object file
$(ENTRY_OBJ): $(ENTRY_SRC)
	$(AS) $(ENTRY_SRC) -o $(ENTRY_OBJ)

# Build debug version and link with entry.S
.PHONY: build
build: $(ENTRY_OBJ)
	$(CARGO) build --target aarch64-unknown-none
	$(LD) -T $(LINKER_SCRIPT) $(OUTPUT_DIR)/piztwo-os $(ENTRY_OBJ) -o $(KERNEL)

# Build release version and link with entry.S
.PHONY: build-release
build-release: $(ENTRY_OBJ)
	$(CARGO) build --release --target aarch64-unknown-none
	$(LD) -T $(LINKER_SCRIPT) $(OUTPUT_DIR_RELEASE)/piztwo-os $(ENTRY_OBJ) -o $(KERNEL_RELEASE)

# Run debug version
.PHONY: run
run: build
	$(QEMU) -M raspi3b -kernel $(KERNEL) -serial stdio -display none

# Run release version
.PHONY: run-release
run-release: build-release
	$(QEMU) -M raspi3b -kernel $(KERNEL_RELEASE) -serial stdio -display none

# Build and run integration test, then link with entry.S
.PHONY: test-integration
test-integration: $(ENTRY_OBJ)
	$(CARGO) test --target aarch64-unknown-none --test integration
	$(LD) -T $(LINKER_SCRIPT) $(OUTPUT_DIR)/piztwo-os $(ENTRY_OBJ) -o $(TEST_KERNEL)

# Run QEMU test
.PHONY: qemu-test
qemu-test: test-integration
	$(QEMU) -M raspi3b -kernel $(TEST_KERNEL) -serial stdio -display none

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean
	rm -f $(BUILD_DIR)/*