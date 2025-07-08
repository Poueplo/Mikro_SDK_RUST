/****************************************************************************
**
** Copyright (C) ${COPYRIGHT_YEAR} MikroElektronika d.o.o.
** Contact: https://www.mikroe.com/contact
**
** This file is part of the mikroSDK package
**
** Commercial License Usage
**
** Licensees holding valid commercial NECTO compilers AI licenses may use this
** file in accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The MikroElektronika Company.
** For licensing terms and conditions see
** https://www.mikroe.com/legal/software-license-agreement.
** For further information use the contact form at
** https://www.mikroe.com/contact.
**
**
** GNU Lesser General Public License Usage
**
** Alternatively, this file may be used for
** non-commercial projects under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** The above copyright notice and this permission notice shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
** OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
** DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
** OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
** OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**
****************************************************************************/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub use crate::gpio::*;

#[cfg(feature = "uart1")]
pub const HAL_LL_UART1_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart2")]
pub const HAL_LL_UART2_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart3")]
pub const HAL_LL_UART3_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart4")]
pub const HAL_LL_UART4_GPIO_AF8: u8 = 8;
#[cfg(feature = "uart5")]
pub const HAL_LL_UART5_GPIO_AF8: u8 = 8;


#[cfg(feature = "uart1")]
pub const UART1_BASE_ADDR : u32 = 0x4001_1000;
#[cfg(feature = "uart2")]
pub const UART2_BASE_ADDR : u32 = 0x4000_4400;
#[cfg(feature = "uart3")]
pub const UART3_BASE_ADDR : u32 = 0x4000_4800;
#[cfg(feature = "uart4")]
pub const UART4_BASE_ADDR : u32 = 0x4000_4C00;
#[cfg(feature = "uart5")]
pub const UART5_BASE_ADDR : u32 = 0x4000_5000;


#[cfg(feature = "uart1")]
pub const UART1_NVIC: u8 = 37;
#[cfg(feature = "uart2")]
pub const UART2_NVIC: u8 = 38;
#[cfg(feature = "uart3")]
pub const UART3_NVIC: u8 = 39;
#[cfg(feature = "uart4")]
pub const UART4_NVIC: u8 = 52;
#[cfg(feature = "uart5")]
pub const UART5_NVIC: u8 = 53;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum uart_modules {
    NULL_MODULE, //is present to ensure the enum is never empty
    #[cfg(feature = "uart1")]
    UART_MODULE_1,
    #[cfg(feature = "uart2")]
    UART_MODULE_2,
    #[cfg(feature = "uart3")]
    UART_MODULE_3,
    #[cfg(feature = "uart4")]
    UART_MODULE_4,
    #[cfg(feature = "uart5")]
    UART_MODULE_5,
}

pub const UART_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 }
    + if cfg!(feature = "uart5") { 1 } else { 0 };


pub struct hal_ll_uart_pin_map_t {
    pub pin: hal_ll_pin_name_t,
    pub base: hal_ll_base_addr_t,
    pub af: u32,
    pub module_index: u8
}

pub static hal_ll_uart_tx_map : &[hal_ll_uart_pin_map_t] = & [
    #[cfg(all(feature = "uart1", feature = "uart1_tx_a9_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A9, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_tx_b6_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B6, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },

    #[cfg(all(feature = "uart2", feature = "uart2_tx_a2_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A2, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },
    #[cfg(all(feature = "uart2", feature = "uart2_tx_d5_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D5, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },

    #[cfg(all(feature = "uart3", feature = "uart3_tx_b10_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B10, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_tx_c10_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C10, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_tx_d8_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D8, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },

    #[cfg(all(feature = "uart4", feature = "uart4_tx_a0_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A0, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_tx_c10_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C10, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },

    #[cfg(all(feature = "uart5", feature = "uart5_tx_c12_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C12, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];

pub static hal_ll_uart_rx_map : &[hal_ll_uart_pin_map_t] = & [
    #[cfg(all(feature = "uart1", feature = "uart1_rx_a10_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A10, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_rx_b7_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B7, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },

    #[cfg(all(feature = "uart2", feature = "uart2_rx_a3_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A3, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },
    #[cfg(all(feature = "uart2", feature = "uart2_rx_d6_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D6, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },

    #[cfg(all(feature = "uart3", feature = "uart3_rx_b11_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_rx_c11_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_rx_d9_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D9, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },

    #[cfg(all(feature = "uart4", feature = "uart4_rx_a1_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A1, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_c11_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },

    #[cfg(all(feature = "uart5", feature = "uart5_rx_d2_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D2, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];


pub const fn hal_ll_uart_module_num(module: u8) -> u8 {
    module - 1
}