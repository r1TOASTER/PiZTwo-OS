
# PiZTwo-OS

An experimental operating system written from scratch in Rust for the Raspberry Pi Zero 2W, targeting the aarch64 architecture.

## Overview

PiZTwo-OS is a lightweight, experimental OS designed to explore low-level system programming on the Raspberry Pi Zero 2W. Built entirely in Rust, it leverages the language's safety and performance features to create a minimal, reliable, and secure environment for the aarch64 architecture. This project is intended for hobbyists, researchers, and developers interested in OS development and embedded systems.

## Features

- **Minimal Kernel**: A bare-bones kernel with memory management, interrupt handling, drivers and etc.
- **Rust-Based**: Utilizes Rust's memory safety guarantees to reduce common low-level bugs.
- **aarch64 Support**: Optimized for the Raspberry Pi Zero 2W's 64-bit ARM Cortex-A53 processor.
- **UART Output**: Basic serial console output for debugging and interaction.
- **Modular Design**: Structured for easy experimentation and extension.

## Hardware Requirements

- **Raspberry Pi Zero 2W**
- MicroSD card (size determained by release builds)
- USB-to-serial adapter: for console interaction (upcoming HDMI support)
- Power supply: micro USB

## Getting Started

### Prerequisites

- **Rust Toolchain**: Install the nightly Rust toolchain with `rustup`.
  ```bash
  rustup default nightly
  rustup target add aarch64-unknown-none
  ```
- **QEMU**: Optional, for emulation.
  ```bash
  sudo apt install qemu-system-arm
  ```
- **Minicom** or similar: For serial communication.

### Building the OS

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/PiZTwo-OS.git
   cd PiZTwo-OS
   ```

2. Build the kernel:
   ```bash
   make release
   ```

### Flashing the SD Card
TODO: check the flashing SD card instructions
1. Insert the MicroSD card into your computer.
2. Copy the generated image to the SD card:
   ```bash
   sudo dd if=target/aarch64-unknown-none/release/pizeroos.img of=/dev/sdX bs=4M status=progress
   ```
   Replace `/dev/sdX` with your SD card's device path.

3. Eject the SD card and insert it into the Raspberry Pi Zero 2W.

### Running on Hardware
TODO: check the terminal emulator settings instructions (baud rate)
1. Connect a USB-to-serial adapter to the Raspberry Pi Zero 2W's UART pins (GPIO x and x).
2. Open a terminal emulator (e.g., Minicom):
   ```bash
   minicom -b 115200 -D /dev/ttyUSB0
   ```
3. Power on the Raspberry Pi Zero 2W and observe the boot output.

## Structure

- **Root**:
  - `aarch64-unknown-none.json`: `aarch64` target spec.
  - `Cargo.toml`, `Cargo.lock`: Rust project config.
  - `Makefile`: Build automation.
  - `LICENSE`: MIT License.

- **`src`**:
  - `buses/{i2c,spi,uart,usb}`: Communication bus drivers.
  - `common`: Shared utilities.
  - `cpu`: CPU functionality.
  - `graphics`: Display drivers.
  - `interrupt`: Interrupt handling.
  - `ipc`: Inter-process communication.
  - `main.rs`: Entry point.
  - `memory/{alloc,dma}`: Memory management.
  - `net/{bluetooth/{ble,classic},wlan}`: Networking protocols.
  - `panic`: Custom panic handler.
  - `peripherals/{emmc,gpio,i2s,pwm}`: Peripheral drivers.
  - `process`: Task management.
  - `timer`: Timer support.

- **`tests`**: Integration tests (`buses.rs`, `cpu.rs`, etc.).
- **`target`**: Build artifacts.

## Contributing

Contributions are welcome! Please fork the repository, create a feature branch, and submit a pull request. Ensure your code follows Rust's style guidelines and includes appropriate documentation.

### Guidelines

- Use `cargo fmt` for code formatting.
- Write clear commit messages.
- Add tests for new features where possible.
- Report issues or suggest improvements via the GitHub Issues page.

## Roadmap

- [ ] Interrupt handling
- [ ] Debug support using SWD
- [ ] Peripherals and Buses support
- [ ] USB driver for peripherals / boot
- [ ] Graphics / User interface via UART / HDMI 
- [ ] Basic virtual memory support
- [ ] Simple task scheduling
- [ ] Filesystem support (e.g., FAT32)
- [ ] Network stack (WLAN, Bluetooth classic / BLE)
- [ ] RTC from NTP
- [ ] User-space application support

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for excellent documentation and tools.
- Raspberry Pi Foundation for the Zero 2W platform.

## Contact

For questions or feedback, open an issue on GitHub.
