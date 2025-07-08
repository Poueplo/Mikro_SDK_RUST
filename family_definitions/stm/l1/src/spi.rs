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

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::gpio::*;


#[cfg(feature = "spi1")]
pub const HAL_LL_SPI1_MASTER_GPIO_AF5 : u8 = 5;
#[cfg(feature = "spi2")]
pub const HAL_LL_SPI2_MASTER_GPIO_AF5 : u8 = 5;

#[cfg(feature = "spi3")]
pub const HAL_LL_SPI3_MASTER_GPIO_AF5 : u8 = 5;
#[cfg(feature = "spi3")]
pub const HAL_LL_SPI3_MASTER_GPIO_AF6 : u8 = 6;
#[cfg(feature = "spi3")]
pub const HAL_LL_SPI3_MASTER_GPIO_AF7 : u8 = 7;


#[cfg(feature = "spi1")]
pub const SPI1_MASTER_BASE_ADDR : u32 = 0x4001_3000;
#[cfg(feature = "spi2")]
pub const SPI2_MASTER_BASE_ADDR : u32 = 0x4000_3800;
#[cfg(feature = "spi3")]
pub const SPI3_MASTER_BASE_ADDR : u32 = 0x4000_3C00;


#[repr(u8)]
#[derive(Clone, Copy)]
pub enum spi_modules {
    NULL_MODULE, //is present to ensure the enum is never empty
    #[cfg(feature = "spi1")]
    SPI_MODULE_1,
    #[cfg(feature = "spi2")]
    SPI_MODULE_2,
    #[cfg(feature = "spi3")]
    SPI_MODULE_3
}

pub const SPI_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "spi1") { 1 } else { 0 }
    + if cfg!(feature = "spi2") { 1 } else { 0 }
    + if cfg!(feature = "spi3") { 1 } else { 0 };


pub struct hal_ll_spi_master_pin_map_t {
    pub pin : hal_ll_pin_name_t,
    pub base : hal_ll_base_addr_t,
    pub module_index : u8,
    pub af : u32
}

pub const _spi_sck_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    #[cfg(all(feature = "spi1", feature = "spi1_sck_a5_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A5, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_sck_b3_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B3, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_sck_e13_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E13, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},



    // #[cfg(all(feature = "spi2", feature = "spi2_sck_b10_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_B10, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi2", feature = "spi2_sck_b13_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B13, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi2", feature = "spi2_sck_d1_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_D1, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    // #[cfg(all(feature = "spi2", feature = "spi2_sck_i1_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_I1, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    #[cfg(all(feature = "spi3", feature = "spi3_sck_b3_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B3, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    #[cfg(all(feature = "spi3", feature = "spi3_sck_c10_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C10, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    

    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}

];

pub const _spi_miso_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    #[cfg(all(feature = "spi1", feature = "spi1_miso_a6_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A6, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_miso_a11_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A11, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_miso_b4_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B4, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_miso_e14_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E14, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},

    #[cfg(all(feature = "spi2", feature = "spi2_miso_b14_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B14, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    // #[cfg(all(feature = "spi2", feature = "spi2_miso_c2_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_C2, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi2", feature = "spi2_miso_d3_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_D3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    // #[cfg(all(feature = "spi2", feature = "spi2_miso_i2_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_I2, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    #[cfg(all(feature = "spi3", feature = "spi3_miso_b4_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B4, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    #[cfg(all(feature = "spi3", feature = "spi3_miso_c11_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C11, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    
    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}
];

pub const _spi_mosi_map: &[hal_ll_spi_master_pin_map_t] = 
&[
    #[cfg(all(feature = "spi1", feature = "spi1_mosi_a7_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A7, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_mosi_a12_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_A12, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_mosi_b5_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B5, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi1", feature = "spi1_mosi_e15_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_E15, base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), af: HAL_LL_SPI1_MASTER_GPIO_AF5 as u32},

    #[cfg(all(feature = "spi2", feature = "spi2_mosi_b15_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B15, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    // #[cfg(all(feature = "spi2", feature = "spi2_mosi_c3_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_C3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    #[cfg(all(feature = "spi2", feature = "spi2_mosi_d4_af5"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_D4, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},
    // #[cfg(all(feature = "spi2", feature = "spi2_mosi_i3_af5"))]
    // hal_ll_spi_master_pin_map_t{ pin: GPIO_I3, base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), af: HAL_LL_SPI2_MASTER_GPIO_AF5 as u32},

    #[cfg(all(feature = "spi3", feature = "spi3_mosi_b5_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_B5, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    #[cfg(all(feature = "spi3", feature = "spi3_mosi_c12_af6"))]
    hal_ll_spi_master_pin_map_t{ pin: GPIO_C12, base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), af: HAL_LL_SPI3_MASTER_GPIO_AF6 as u32},
    

    hal_ll_spi_master_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32}
];

pub const fn hal_ll_spi_master_module_num(module: u8) -> u8 {
    module - 1
}