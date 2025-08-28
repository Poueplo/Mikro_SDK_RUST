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
pub const HAL_LL_UART1_GPIO_AF4: u8 = 4;
#[cfg(feature = "uart1")]
pub const HAL_LL_UART1_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart2")]
pub const HAL_LL_UART2_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart3")]
pub const HAL_LL_UART3_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart4")]
pub const HAL_LL_UART4_GPIO_AF6: u8 = 6;
#[cfg(feature = "uart4")]
pub const HAL_LL_UART4_GPIO_AF8: u8 = 8;
#[cfg(feature = "uart5")]
pub const HAL_LL_UART5_GPIO_AF1: u8 = 1;
#[cfg(feature = "uart5")]
pub const HAL_LL_UART5_GPIO_AF7: u8 = 7;
#[cfg(feature = "uart5")]
pub const HAL_LL_UART5_GPIO_AF8: u8 = 8;
#[cfg(feature = "uart6")]
pub const HAL_LL_UART6_GPIO_AF8: u8 = 8;
#[cfg(feature = "uart7")]
pub const HAL_LL_UART7_GPIO_AF8: u8 = 8;
#[cfg(feature = "uart7")]
pub const HAL_LL_UART7_GPIO_AF12: u8 = 12;
#[cfg(feature = "uart8")]
pub const HAL_LL_UART8_GPIO_AF8: u8 = 8;


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
#[cfg(feature = "uart6")]
pub const UART6_BASE_ADDR : u32 = 0x4001_1400;
#[cfg(feature = "uart7")]
pub const UART7_BASE_ADDR : u32 = 0x4000_7800;
#[cfg(feature = "uart8")]
pub const UART8_BASE_ADDR : u32 = 0x4000_7C00;


#[cfg(feature = "uart1")]
pub const UART1_NVIC: u8 = 53;
#[cfg(feature = "uart2")]
pub const UART2_NVIC: u8 = 54;
#[cfg(feature = "uart3")]
pub const UART3_NVIC: u8 = 55;
#[cfg(feature = "uart4")]
pub const UART4_NVIC: u8 = 68;
#[cfg(feature = "uart5")]
pub const UART5_NVIC: u8 = 69;
#[cfg(feature = "uart6")]
pub const UART6_NVIC: u8 = 87;
#[cfg(feature = "uart7")]
pub const UART7_NVIC: u8 = 98;
#[cfg(feature = "uart8")]
pub const UART8_NVIC: u8 = 99;

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
    #[cfg(feature = "uart6")]
    UART_MODULE_6,
    #[cfg(feature = "uart7")]
    UART_MODULE_7,
    #[cfg(feature = "uart8")]
    UART_MODULE_8,
}

pub const UART_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 }
    + if cfg!(feature = "uart5") { 1 } else { 0 }
    + if cfg!(feature = "uart6") { 1 } else { 0 }
    + if cfg!(feature = "uart7") { 1 } else { 0 }
    + if cfg!(feature = "uart8") { 1 } else { 0 };


pub struct hal_ll_uart_pin_map_t {
    pub pin: hal_ll_pin_name_t,
    pub base: hal_ll_base_addr_t,
    pub af: u32,
    pub module_index: u8
}

pub static hal_ll_uart_tx_map : &[hal_ll_uart_pin_map_t] = & [
    #[cfg(all(feature = "uart1", feature = "uart1_tx_a9_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A9, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    // #[cfg(all(feature = "uart1", feature = "uart1_tx_a15_af7"))]
    // hal_ll_uart_pin_map_t { pin: GPIO_A15, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_tx_b6_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B6, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_tx_b14_af4"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B14, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF4 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },

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
    #[cfg(all(feature = "uart4", feature = "uart4_tx_a12_af6"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A12, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF6 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_tx_c10_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C10, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_tx_d1_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D1, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_tx_h13_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_H13, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },

    #[cfg(all(feature = "uart5", feature = "uart5_tx_b6_af1"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B6, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF1 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_tx_b9_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B9, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_tx_b13_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B13, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_tx_c12_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C12, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    
    // #[cfg(all(feature = "uart6", feature = "uart6_tx_a11_af8"))]
    // hal_ll_uart_pin_map_t { pin: GPIO_A11, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },
    #[cfg(all(feature = "uart6", feature = "uart6_tx_c6_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C6, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },
    #[cfg(all(feature = "uart6", feature = "uart6_tx_g14_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_G14, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },

    #[cfg(all(feature = "uart7", feature = "uart7_tx_a15_af12"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A15, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF12 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_tx_b4_af12"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B4, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF12 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_tx_e8_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_E8, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_tx_f7_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_F7, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },

    #[cfg(all(feature = "uart8", feature = "uart8_tx_e1_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_E1, base: UART8_BASE_ADDR, af: HAL_LL_UART8_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];

pub static hal_ll_uart_rx_map : &[hal_ll_uart_pin_map_t] = & [
    #[cfg(all(feature = "uart1", feature = "uart1_rx_a10_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A10, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    // #[cfg(all(feature = "uart1", feature = "uart1_rx_b3_af7"))]
    // hal_ll_uart_pin_map_t { pin: GPIO_B3, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_rx_b7_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B7, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    #[cfg(all(feature = "uart1", feature = "uart1_rx_b15_af4"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B15, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF4 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8) },
    
    #[cfg(all(feature = "uart2", feature = "uart2_rx_a3_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A3, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },
    #[cfg(all(feature = "uart2", feature = "uart2_rx_d6_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D6, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8) },

    #[cfg(all(feature = "uart3", feature = "uart3_rx_b11_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    // #[cfg(all(feature = "uart3", feature = "uart3_rx_c5_af7"))]
    // hal_ll_uart_pin_map_t { pin: GPIO_C5, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_rx_c11_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },
    #[cfg(all(feature = "uart3", feature = "uart3_rx_d9_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D9, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8) },

    #[cfg(all(feature = "uart4", feature = "uart4_rx_a1_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A1, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_a11_af6"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A11, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF6 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_c11_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_d0_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D0, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_h14_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_H14, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },
    #[cfg(all(feature = "uart4", feature = "uart4_rx_i9_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_I9, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8) },

    #[cfg(all(feature = "uart5", feature = "uart5_rx_b5_af1"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B5, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF1 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_rx_b8_af7"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B8, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_rx_b12_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B12, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    #[cfg(all(feature = "uart5", feature = "uart5_rx_d2_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_D2, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8) },
    
    // #[cfg(all(feature = "uart6", feature = "uart6_rx_a12_af8"))]
    // hal_ll_uart_pin_map_t { pin: GPIO_A12, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },
    #[cfg(all(feature = "uart6", feature = "uart6_rx_c7_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_C7, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },
    #[cfg(all(feature = "uart6", feature = "uart6_rx_g9_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_G9, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8) },

    #[cfg(all(feature = "uart7", feature = "uart7_rx_a8_af12"))]
    hal_ll_uart_pin_map_t { pin: GPIO_A8, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF12 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_rx_b3_af12"))]
    hal_ll_uart_pin_map_t { pin: GPIO_B3, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF12 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_rx_e7_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_E7, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    #[cfg(all(feature = "uart7", feature = "uart7_rx_f6_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_F6, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8) },
    
    #[cfg(all(feature = "uart8", feature = "uart8_rx_e0_af8"))]
    hal_ll_uart_pin_map_t { pin: GPIO_E0, base: UART8_BASE_ADDR, af: HAL_LL_UART8_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];


pub const fn hal_ll_uart_module_num(module: u8) -> u8 {
    module - 1
}