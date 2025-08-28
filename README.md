# Mikro SDK in Rust

**Supported MCU:** 
- STM32F207ZG
- STM32F217ZG
- STM32F405xx, STM32F407xx, STM32F415xx and STM32F417xx tested with STM32F405ZG
- STM32F412RE
- STM32F429ZI
- STM32F479II
- STM32L152RE
- STM32F723IE and STM32F723ZE tested with STM32F723ZE
- STM32F756NG and STM32F756ZG tested with STM32F756ZG
- STM32F777NI

## Overview  
This version implements every module required by Mikrobuses.  
The folder `test` provides a set of `main.rs` files demonstrating the capabilities of the SDK using function from the libraries below:  

- `drv_digital_in`
- `drv_digital_out`
- `drv_port`
- `drv_analog_in`
- `drv_i2c_master`
- `drv_spi_master`
- `drv_pwm`
- `drv_uart`
- `drv_one_wire`

For some functions, it is recommended to use a debugger to check their contents.
The setting of pins and port should be adapted to the MCU being used.

## Compiling and Flashing the Project  

### Prerequisites  
Ensure you have the following tools installed:  

- **Rustup and python 3**
  - Installer for Windows, and installation procedure for other platform can be found here: 
    - [Rust Installation Guide](https://www.rust-lang.org/tools/install)
    - [Python](https://www.python.org/downloads/)

- **probe-rs**  
  - Windows PowerShell command:  
    ```powershell
    powershell -ExecutionPolicy Bypass -c "irm https://github.com/probe-rs/probe-rs/releases/download/v0.27.0/probe-rs-tools-installer.ps1 | iex"
    ```
  - Alternatively, you can download it manually from: [probe-rs Releases](https://github.com/probe-rs/probe-rs/releases)
  - There You will also find further informations for other platform.

You must have a ST-Link V2, Firmware version 2.26 or higher

### Project Setup

To set your project up according to your chosen MCU You must launch the python app `project_setup_application.py`.
It will open a window in which you will have the possibility to search for your MCU. It will load a default set of settings for the clock that can be adjusted to your liking. Then all you have to do is press the button "Save System Parameters" at the bottom of the windows and you project will be set to the chosen MCU with you clock settings.

You can now develope your project !

### Compiling and Flashing  

Run the following command to build and flash your MCU:  

```sh
cargo-flash --chip STM32F429ZI --connect-under-reset
```
This command should be adapted to the MCU used for your project

After executing this command, you may be prompted to select a detected programmer (if multiple are connected). Choose the appropriate ST-Link V2.

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
The target should be adapted again to the family of the MCU in use for your project. Here is a list for the family currently being supported:
  - stm32f4x.cfg
  - stm32f2x.cfg
  - stm32l1.cfg
  - stm32f7x.cfg


### Debugging  

1. Start OpenOCD:  
   ```sh
   <Your path to>\xpack-openocd-X.XX.X-X\bin\openocd.exe -f <Your path to>\xpack-openocd-X.XX.X-X\bin\<Your config name>.cfg
   ```
   
2. Launch GDB:  
   ```sh
   <Your path to>\arm-gnu-toolchain-XX.X.rel1-mingw-w64-x86_64-arm-none-eabi\bin\arm-none-eabi-gdb.exe <Your path to>\<project root folder>\target\<target of the chosen  MCU>\debug\mikrosdk
   ```
   If you do not know the name of the target for your specific MCU, you can find it in the file `.cargo/config.toml`. It is automatically set at project setup by the python application.
   <br>
   
3. In GDB:
    ```sh
    target extended-remote localhost:3333
    ```
    or
    ```sh
    tar ext :3333
    ```
