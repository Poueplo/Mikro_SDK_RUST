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

pub const HAL_LL_I2C_AFIO_MAPR_REMAP : u32 = 0x2;
pub const HAL_LL_I2C_REMAP_DISABLE : u32 = 0;
pub const HAL_LL_I2C_REMAP_ENABLE : u32 = 0x0800_0000;

pub const HAL_LL_I2C1_GPIO_AF4 : u8 = 4;
pub const HAL_LL_I2C2_GPIO_AF4 : u8 = 4;
pub const HAL_LL_I2C2_GPIO_AF9 : u8 = 9;
pub const HAL_LL_I2C3_GPIO_AF4 : u8 = 4;
pub const HAL_LL_I2C3_GPIO_AF9 : u8 = 9;


#[repr(u8)]
#[derive(Clone, Copy)]
pub enum i2c_modules {
    NULL_MODULE, //is present to ensure the enum is never empty
    #[cfg(feature = "i2c1")]
    I2C_MODULE_1,
    #[cfg(feature = "i2c2")]
    I2C_MODULE_2,
    #[cfg(feature = "i2c3")]
    I2C_MODULE_3,
}

pub const I2C_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "i2c1") { 1 } else { 0 }
    + if cfg!(feature = "i2c2") { 1 } else { 0 }
    + if cfg!(feature = "i2c3") { 1 } else { 0 };

#[cfg(feature = "i2c1")]
pub const I2C1_BASE_ADDR: u32 = 0x4000_5400;
#[cfg(feature = "i2c2")]
pub const I2C2_BASE_ADDR: u32 = 0x4000_5800;
#[cfg(feature = "i2c3")]
pub const I2C3_BASE_ADDR: u32 = 0x4000_5C00;

pub struct hal_ll_i2c_pin_map_t
{
    pub pin : hal_ll_pin_name_t,
    pub base : hal_ll_base_addr_t,
    pub module_index : hal_ll_pin_name_t,
    pub af : u32
}

pub const hal_ll_i2c_scl_map: &[hal_ll_i2c_pin_map_t] =
&[
    // hal_ll_i2c_pin_map_t{ pin: GPIO_B6, base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), af: HAL_LL_I2C_REMAP_DISABLE },
    #[cfg(all(feature = "i2c1", feature = "i2c1_scl_b6_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B6, base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), af: HAL_LL_I2C1_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c1", feature = "i2c1_scl_b8_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B8, base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), af: HAL_LL_I2C1_GPIO_AF4 as u32},
    
    // hal_ll_i2c_pin_map_t{ pin: GPIO_B10, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C_REMAP_DISABLE },
    #[cfg(all(feature = "i2c2", feature = "i2c2_scl_b10_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B10, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c2", feature = "i2c2_scl_f1_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_F1, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c2", feature = "i2c2_scl_h4_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_H4, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    
    #[cfg(all(feature = "i2c3", feature = "i2c3_scl_a8_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_A8, base: I2C3_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8), af: HAL_LL_I2C3_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c3", feature = "i2c3_scl_h7_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_H7, base: I2C3_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8), af: HAL_LL_I2C3_GPIO_AF4 as u32},
    
    hal_ll_i2c_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32},
];

pub const hal_ll_i2c_sda_map: &[hal_ll_i2c_pin_map_t] =
&[
    #[cfg(all(feature = "i2c1", feature = "i2c1_sda_b7_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B7, base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), af: HAL_LL_I2C1_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c1", feature = "i2c1_sda_b9_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B9, base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), af: HAL_LL_I2C1_GPIO_AF4 as u32},
    
    #[cfg(all(feature = "i2c2", feature = "i2c2_sda_b11_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_B11, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c2", feature = "i2c2_sda_f0_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_F0, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c2", feature = "i2c2_sda_h5_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_H5, base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), af: HAL_LL_I2C2_GPIO_AF4 as u32},
    
    #[cfg(all(feature = "i2c3", feature = "i2c3_sda_c9_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_C9, base: I2C3_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8), af: HAL_LL_I2C3_GPIO_AF4 as u32},
    #[cfg(all(feature = "i2c3", feature = "i2c3_sda_h8_af4"))]
    hal_ll_i2c_pin_map_t{ pin: GPIO_H8, base: I2C3_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8), af: HAL_LL_I2C3_GPIO_AF4 as u32},
    
    hal_ll_i2c_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, af: HAL_LL_PIN_NC as u32},
];

pub const fn hal_ll_i2c_module_num(module: u8) -> u8 {
    module - 1
}