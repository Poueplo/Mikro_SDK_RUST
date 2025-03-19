# Mikro SDK in Rust

**Supported MCU:** STM32F429x  

## Overview  
This initial version implements only GPIO functionality.  
The `main.rs` file calls functions from the following modules:  

- `drv_digital_in`  
- `drv_digital_out`  
- `drv_port`  

For some functions, it is recommended to use a debugger to check the return value, especially for the variable `value1`.  

## Compiling and Flashing the Project  

### Prerequisites  
Ensure you have the following tools installed:  

- **Rustup**  
  - Installer for Windows, and installation procedure for other platform can be found here: [Rust Installation Guide](https://www.rust-lang.org/tools/install)  

- **probe-rs**  
  - Windows PowerShell command:  
    ```powershell
    powershell -ExecutionPolicy Bypass -c "irm https://github.com/probe-rs/probe-rs/releases/download/v0.27.0/probe-rs-tools-installer.ps1 | iex"
    ```
  - Alternatively, you can download it manually from: [probe-rs Releases](https://github.com/probe-rs/probe-rs/releases)
  - There You will also find further informations for other platform.

You must have a ST-Link V2, Firmware version 2.26 or higher

### Compiling and Flashing  

Run the following command to flash your MCU:  

```sh
cargo-flash --chip STM32F429ZI --connect-under-reset
```

After executing this command, you may be prompted to select a detected programmer (if multiple are connected). Choose the appropriate ST-Link V2.

This project is configured to target the thumbv7m-none-eabi architecture by default. You can find this setting in .cargo/config.toml.

## Debugging on Windows

### Prerequisites

- **OpenOCD-xpack**
 - [OpenOCD-xpack download](https://sourceforge.net/projects/openocd-xpack/)

- **arm-none-eabi**
 - [arm-none-eabi download](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)

### Setting Up  

In the `xpack-openocd-X.XX.X-X\bin` folder, create a new configuration file (`.cfg`).  
This file must contain the following:  

```cfg
source [find interface/stlink.cfg]
transport select dapdirect_swd
source [find target/stm32f4x.cfg]
```

### Debugging  

1. Start OpenOCD:  
   ```sh
   <Your path to>\xpack-openocd-X.XX.X-X\bin\openocd.exe -f <Your path to>\xpack-openocd-X.XX.X-X\bin\<Your config name>.cfg
   ```
   
2. Launch GDB:  
   ```sh
   <Your path to>\arm-gnu-toolchain-XX.X.rel1-mingw-w64-x86_64-arm-none-eabi\bin\arm-none-eabi-gdb.exe <Your path to>\<project root folder>\target\thumbv7m-none-eabi\debug\mikrosdk
   ```
3. In GDB:
    ```sh
    target extended-remote localhost:3333
    ```
    or
    ```sh
    tar ext :3333
    ```