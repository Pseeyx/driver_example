# Driver Example

A Kernel-Mode Driver Framework (KMDF) example written in Rust. This project demonstrates how to build a Windows kernel driver using Rust alongside the Windows Driver Kit (WDK) and appropriate tooling.

## Project Overview

This example leverages the safety, reliability, and performance of Rust to build a KMDF driver. It is structured to provide seamless integration with WDK tools and utilities, enabling easy development and deployment of kernel drivers.

### Project Structure
- **`src/`**: Contains the Rust source code for the driver implementation.
- **`build.rs`**: Build script responsible for setting up WDK-related build artifacts.
- **`Cargo.toml`**: Defines project dependencies and metadata.
- **`.cargo/config.toml`**: Configures Cargo-specific settings and environment variables.
- **`kernel_driver.inx`**: INX file used for creating `.inf` installer packages for the driver.

### Key Features
- Written in Rust to take advantage of its memory safety guarantees and performance.
- Full integration with the Windows Driver Kit (WDK) build tools using Rust.
- Support for automated build tasks like `.cat` and `.inf` creation through optional tooling like `cargo-make`.

---

## Prerequisites

Before building or running this example, ensure that you have the following dependencies installed and properly configured:

1. **Rust and Cargo**: Install the latest Rust compiler and Cargo using [rustup](https://rustup.rs/).
2. **LLVM (Version 17.0.6)**: Required for `bindgen`, which generates bindings. For simplicity, install via `winget` or directly download it:
   ```bash
   winget install -i LLVM.LLVM --version 17.0.6 --force
   ```
   - ⚠️ **Note**: LLVM 18 has an ARM64 bindings bug. Use LLVM 17 until a fix is released in LLVM 19. For details, see [this issue](https://github.com/rust-lang/rust-bindgen/issues/2842).

3. **Windows Driver Kit (WDK)**: Download and configure the [Enterprise WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk).
4. **Cargo Make** (Optional but Recommended): Automates additional build tasks. Install it with:
   ```bash
   cargo install --locked cargo-make --no-default-features --features tls-native
   ```

---

## Installation and Setup

### Step 1: Set Up Rust and LLVM
Install Rust using [rustup](https://rustup.rs/) and ensure LLVM tools are available in your system's `PATH`. Configure the `LIBCLANG_PATH` so that Rust bindings generation works correctly:
```bash
export LIBCLANG_PATH=<path_to_llvm_bin>
```

### Step 2: Configure the WDK Environment
Set up a valid WDK environment by launching the [eWDK developer prompt](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk#getting-started).

---

## Building the Driver

Follow these steps to build the driver example:

1. **Ensure Build Dependencies are Correctly Installed**
   - Confirm Rust, LLVM, and WDK are properly installed and configured.
   - Double-check the `LIBCLANG_PATH` environment variable.

2. **Build the Project**
   Run the following command to build the project through Cargo:
   ```bash
   cargo build
   ```

3. **Optional Post-Build Steps**
   Run additional build tasks supported by `cargo-make` to finalize the driver (e.g., `.inf` and `.cat` creation):
   ```bash
   cargo make
   ```

---

## Getting Help

For additional guidance or reference, check out the following resources:

- Rust Documentation: [https://doc.rust-lang.org/](https://doc.rust-lang.org/)
- Windows Driver Kit (WDK) Guidance: [https://learn.microsoft.com/en-us/windows-hardware/drivers/](https://learn.microsoft.com/en-us/windows-hardware/drivers/)
- Open an issue in this repository for project-related discussions or questions.