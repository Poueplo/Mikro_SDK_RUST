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

#![no_std]

pub use hal_ll_pin_names::*;

pub const HAL_LL_UART1_GPIO_AF7: u8 = 7;
pub const HAL_LL_UART1_GPIO_AF: u8 = 4;

pub const HAL_LL_UART2_GPIO_AF7: u8 = 7;
pub const HAL_LL_UART2_GPIO_AF: u8 = 4;

pub const HAL_LL_UART3_GPIO_AF7: u8 = 7;
pub const HAL_LL_UART3_GPIO_AF_1: u32 = 0x10;
pub const HAL_LL_UART3_GPIO_AF_2: u32 = 0x30;

pub const HAL_LL_UART4_GPIO_AF8: u8 = 8;
pub const HAL_LL_UART4_GPIO_AF11: u8 = 11;
pub const HAL_LL_UART4_GPIO_AF: u8 = 0;

pub const HAL_LL_UART5_GPIO_AF8: u8 = 8;
pub const HAL_LL_UART5_GPIO_AF11: u8 = 11;
pub const HAL_LL_UART5_GPIO_AF: u8 = 0;

pub const HAL_LL_UART6_GPIO_AF8: u8 = 8;

pub const HAL_LL_UART7_GPIO_AF8: u8 = 8;

pub const HAL_LL_UART8_GPIO_AF8: u8 = 8;


pub struct hal_ll_uart_pin_map_t {
    pub pin: hal_ll_pin_name_t,
    pub base: hal_ll_base_addr_t,
    pub af: u32,
    pub module_index: u8
}

pub static hal_ll_uart_tx_map : &[hal_ll_uart_pin_map_t] = & [
    hal_ll_uart_pin_map_t { pin: GPIO_A9, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_1) },
    hal_ll_uart_pin_map_t { pin: GPIO_B6, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_1) },

    hal_ll_uart_pin_map_t { pin: GPIO_A2, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_2) },
    hal_ll_uart_pin_map_t { pin: GPIO_D5, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_2) },

    hal_ll_uart_pin_map_t { pin: GPIO_B10, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },
    hal_ll_uart_pin_map_t { pin: GPIO_C10, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },
    hal_ll_uart_pin_map_t { pin: GPIO_D8, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },

    hal_ll_uart_pin_map_t { pin: GPIO_A0, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_4) },
    hal_ll_uart_pin_map_t { pin: GPIO_C10, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_4) },

    hal_ll_uart_pin_map_t { pin: GPIO_C12, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_5) },

    hal_ll_uart_pin_map_t { pin: GPIO_C6, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_6) },
    hal_ll_uart_pin_map_t { pin: GPIO_G14, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_6) },

    hal_ll_uart_pin_map_t { pin: GPIO_E8, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_7) },
    hal_ll_uart_pin_map_t { pin: GPIO_F7, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_7) },

    hal_ll_uart_pin_map_t { pin: GPIO_E1, base: UART8_BASE_ADDR, af: HAL_LL_UART8_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];

pub static hal_ll_uart_rx_map : &[hal_ll_uart_pin_map_t] = & [
    hal_ll_uart_pin_map_t { pin: GPIO_A10, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_1) },
    hal_ll_uart_pin_map_t { pin: GPIO_B7, base: UART1_BASE_ADDR, af: HAL_LL_UART1_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_1) },

    hal_ll_uart_pin_map_t { pin: GPIO_A3, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_2) },
    hal_ll_uart_pin_map_t { pin: GPIO_D6, base: UART2_BASE_ADDR, af: HAL_LL_UART2_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_2) },

    hal_ll_uart_pin_map_t { pin: GPIO_B11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },
    hal_ll_uart_pin_map_t { pin: GPIO_D9, base: UART3_BASE_ADDR, af: HAL_LL_UART3_GPIO_AF7 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_3) },

    hal_ll_uart_pin_map_t { pin: GPIO_A1, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_4) },
    hal_ll_uart_pin_map_t { pin: GPIO_C11, base: UART4_BASE_ADDR, af: HAL_LL_UART4_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_4) },

    hal_ll_uart_pin_map_t { pin: GPIO_D2, base: UART5_BASE_ADDR, af: HAL_LL_UART5_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_5) },

    hal_ll_uart_pin_map_t { pin: GPIO_C7, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_6) },
    hal_ll_uart_pin_map_t { pin: GPIO_G9, base: UART6_BASE_ADDR, af: HAL_LL_UART6_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_6) },

    hal_ll_uart_pin_map_t { pin: GPIO_E7, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_7) },
    hal_ll_uart_pin_map_t { pin: GPIO_F6, base: UART7_BASE_ADDR, af: HAL_LL_UART7_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_7) },

    hal_ll_uart_pin_map_t { pin: GPIO_E0, base: UART8_BASE_ADDR, af: HAL_LL_UART8_GPIO_AF8 as u32, module_index: hal_ll_uart_module_num(UART_MODULE_8) },


    hal_ll_uart_pin_map_t { pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, af: HAL_LL_PIN_NC as u32, module_index: HAL_LL_PIN_NC },
];


pub const fn hal_ll_uart_module_num(module: u8) -> u8 {
    module - 1
}