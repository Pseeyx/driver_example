# Driver Example

This project is a Kernel-Mode Driver Framework (KMDF) driver written in Rust.

## Project Structure

    - `src/`: Contains the Rust source code for the driver.
    - `build.rs`: Build script for configuring the WDK binary build.
    - `Cargo.toml`: Cargo configuration file with dependencies.
    - `.cargo/config.toml`: Cargo configuration file with build settings.
    - `kernel_driver.inx`: INX file for driver installation.

## Dependencies

    - Rust
    - Cargo
    - LLVM (for `LIBCLANG_PATH`)

## Building the Project

    1. Ensure you have Rust and Cargo installed.
    2. Set the `LIBCLANG_PATH` environment variable to the path of LLVM's bin directory.
    3. Run the following command to build the project:

```sh
  cargo build
```

## Installing the Driver

    1. Use the `.inx` file to create an installation package.
    2. Follow the standard procedures to install the driver on your system.

## Usage

    - The driver logs messages to the kernel debugger.
    - Modify the `KMDFDriver.DeviceDesc` and `KMDFDriver.SVCDESC` in the `.inx` file to change the driver name.

## Getting Started

### Build Requirements

 * Binding generation via `bindgen` requires `libclang`. The easiest way to acquire this is via `winget`
    * `winget install -i LLVM.LLVM --version 17.0.6 --force`
        * Ensure you select the GUI option to add LLVM to the PATH
        * LLVM 18 has a bug that causes bindings to fail to generate for ARM64. Continue using LLVM 17 until LLVM 19
          comes out with [the fix](https://github.com/llvm/llvm-project/pull/93235).
          See [this](https://github.com/rust-lang/rust-bindgen/issues/2842) for more details.
 * To execute post-build tasks (ie. `inf2cat`, `infverif`, etc.), `cargo make` is used
    * `cargo install --locked cargo-make --no-default-features --features tls-native`
    * Building programs with the WDK also requires being in a valid WDK environment. The recommended way to do this is
      to [enter an eWDK developer prompt](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk#getting-started)