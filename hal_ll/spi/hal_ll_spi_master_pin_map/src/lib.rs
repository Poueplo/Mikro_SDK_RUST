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
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use hal_ll_pin_names::*;

pub const HAL_LL_SPI1_MASTER_GPIO_AF0 : u8 = 0;
pub const HAL_LL_SPI1_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI1_MASTER_GPIO_AF6 : u8 = 6;
pub const HAL_LL_SPI1_MASTER_GPIO_AF : u32 = 0x04000001;

pub const HAL_LL_SPI2_MASTER_GPIO_AF0 : u8 = 0;
pub const HAL_LL_SPI2_MASTER_GPIO_AF1 : u8 = 1;
pub const HAL_LL_SPI2_MASTER_GPIO_AF3 : u8 = 3;
pub const HAL_LL_SPI2_MASTER_GPIO_AF4 : u8 = 4;
pub const HAL_LL_SPI2_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI2_MASTER_GPIO_AF7 : u8 = 7;

pub const HAL_LL_SPI3_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI3_MASTER_GPIO_AF6 : u8 = 6;
pub const HAL_LL_SPI3_MASTER_GPIO_AF7 : u8 = 7;
pub const HAL_LL_SPI3_MASTER_GPIO_AF : u32 = 0x10000000;

pub const HAL_LL_SPI4_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI4_MASTER_GPIO_AF6 : u8 = 6;

pub const HAL_LL_SPI5_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI5_MASTER_GPIO_AF6 : u8 = 6;

pub const HAL_LL_SPI6_MASTER_GPIO_AF5 : u8 = 5;
pub const HAL_LL_SPI6_MASTER_GPIO_AF8 : u8 = 8;

pub struct hal_ll_spi_master_pin_map_t {
    pub pin : hal_ll_pin_name_t,
    pub base : hal_ll_base_addr_t,
    pub module_index : u8,
    pub af : u32
}

pub const _spi_sck_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A5, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B3, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B10, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B13, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_D3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_I1, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B3, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C10, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_E2, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E12, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_F7, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_H6, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_G13, base: SPI6_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_6), af: HAL_LL_SPI6_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}

];

pub const _spi_miso_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A6, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B4, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B14, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C2, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_I2, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B4, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C11, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E5, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E13, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_F8, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_H7, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_G12, base: SPI6_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_6), af: HAL_LL_SPI6_MASTER_GPIO_AF5 as u32},
    
    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}
];

pub const _spi_mosi_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A7, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B5, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_1), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B15, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_I3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_2), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_B5, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C12, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_D6, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_3), af: HAL_LL_SPI3_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_E6, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E14, base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_4), af: HAL_LL_SPI4_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_F9, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},
    hal_ll_spi_master_pin_map_t{ pin: GPIO_F11, base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_5), af: HAL_LL_SPI5_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: GPIO_G14, base: SPI6_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(SPI_MODULE_6), af: HAL_LL_SPI6_MASTER_GPIO_AF5 as u32},

    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}
];

pub const fn hal_ll_spi_master_module_num(module: u8) -> u8 {
    module - 1
}