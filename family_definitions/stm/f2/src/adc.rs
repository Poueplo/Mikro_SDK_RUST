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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum adc_modules {
    NULL_MODULE, //is present to ensure the enum is never empty
    #[cfg(feature = "adc1")]
    ADC_MODULE_1,
    #[cfg(feature = "adc2")]
    ADC_MODULE_2,
    #[cfg(feature = "adc3")]
    ADC_MODULE_3,
}


pub const ADC_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "adc1") { 1 } else { 0 }
    + if cfg!(feature = "adc2") { 1 } else { 0 }
    + if cfg!(feature = "adc3") { 1 } else { 0 };


pub const ADC1_BASE_ADDR: u32 = 0x4001_2000;
#[cfg(feature = "adc2")]
pub const ADC2_BASE_ADDR: u32 = 0x4001_2100;
#[cfg(feature = "adc3")]
pub const ADC3_BASE_ADDR: u32 = 0x4001_2200;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum hal_ll_adc_channel_t
{
    HAL_LL_ADC_CHANNEL_0 = 0,
    HAL_LL_ADC_CHANNEL_1,
    HAL_LL_ADC_CHANNEL_2,
    HAL_LL_ADC_CHANNEL_3,
    HAL_LL_ADC_CHANNEL_4,
    HAL_LL_ADC_CHANNEL_5,
    HAL_LL_ADC_CHANNEL_6,
    HAL_LL_ADC_CHANNEL_7,
    HAL_LL_ADC_CHANNEL_8,
    HAL_LL_ADC_CHANNEL_9,
    HAL_LL_ADC_CHANNEL_10,
    HAL_LL_ADC_CHANNEL_11,
    HAL_LL_ADC_CHANNEL_12,
    HAL_LL_ADC_CHANNEL_13,
    HAL_LL_ADC_CHANNEL_14,
    HAL_LL_ADC_CHANNEL_15,
    HAL_LL_ADC_CHANNEL_16,
    HAL_LL_ADC_CHANNEL_17,
    HAL_LL_ADC_CHANNEL_18,
    HAL_LL_ADC_CHANNEL_19,
    HAL_LL_ADC_CHANNEL_20,
    HAL_LL_ADC_CHANNEL_21,
    HAL_LL_ADC_CHANNEL_22,
    HAL_LL_ADC_CHANNEL_23,
    HAL_LL_ADC_CHANNEL_24,
    HAL_LL_ADC_CHANNEL_25,
    HAL_LL_ADC_CHANNEL_26,
    HAL_LL_ADC_CHANNEL_27,
    HAL_LL_ADC_CHANNEL_28,
    HAL_LL_ADC_CHANNEL_29,
    HAL_LL_ADC_CHANNEL_30,
    HAL_LL_ADC_CHANNEL_31,

    HAL_LL_ADC_CHANNEL_NC = 0xFFFF
}

pub struct hal_ll_adc_pin_map_t
{
    pub pin: hal_ll_pin_name_t,
    pub base: hal_ll_base_addr_t,
    pub module_index: hal_ll_pin_name_t,
    pub channel: hal_ll_adc_channel_t,
}

pub const _adc_map: &[hal_ll_adc_pin_map_t] =
&[
    #[cfg(all(feature = "adc1", feature = "adc1_a0_c0"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A0,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_0},
    #[cfg(all(feature = "adc1", feature = "adc1_a1_c1"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A1,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_1},
    #[cfg(all(feature = "adc1", feature = "adc1_a2_c2"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A2,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_2},
    #[cfg(all(feature = "adc1", feature = "adc1_a3_c3"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A3,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_3},
    #[cfg(all(feature = "adc1", feature = "adc1_a4_c4"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A4,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_4},
    #[cfg(all(feature = "adc1", feature = "adc1_a5_c5"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A5,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_5},
    #[cfg(all(feature = "adc1", feature = "adc1_a6_c6"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A6,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_6},
    #[cfg(all(feature = "adc1", feature = "adc1_a7_c7"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A7,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_7},
    #[cfg(all(feature = "adc1", feature = "adc1_b0_c8"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B0,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_8},
    #[cfg(all(feature = "adc1", feature = "adc1_b1_c9"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B1,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_9},
    #[cfg(all(feature = "adc1", feature = "adc1_c0_c10"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C0,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_10},
    #[cfg(all(feature = "adc1", feature = "adc1_c1_c11"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C1,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_11},
    #[cfg(all(feature = "adc1", feature = "adc1_c2_c12"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C2,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_12},
    #[cfg(all(feature = "adc1", feature = "adc1_c3_c13"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C3,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_13},
    #[cfg(all(feature = "adc1", feature = "adc1_c4_c14"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C4,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_14},    
    #[cfg(all(feature = "adc1", feature = "adc1_c5_c15"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C5,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_15},
    
    #[cfg(all(feature = "adc2", feature = "adc2_a0_c0"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A0,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_0},
    #[cfg(all(feature = "adc2", feature = "adc2_a1_c1"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A1,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_1},
    #[cfg(all(feature = "adc2", feature = "adc2_a2_c2"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A2,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_2},
    #[cfg(all(feature = "adc2", feature = "adc2_a3_c3"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A3,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_3},
    #[cfg(all(feature = "adc2", feature = "adc2_a4_c4"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A4,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_4},
    #[cfg(all(feature = "adc2", feature = "adc2_a5_c5"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A5,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_5},
    #[cfg(all(feature = "adc2", feature = "adc2_a6_c6"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A6,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_6},
    #[cfg(all(feature = "adc2", feature = "adc2_a7_c7"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A7,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_7},
    #[cfg(all(feature = "adc2", feature = "adc2_b0_c8"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B0,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_8},
    #[cfg(all(feature = "adc2", feature = "adc2_b1_c9"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B1,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_9},
    #[cfg(all(feature = "adc2", feature = "adc2_c0_c10"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C0,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_10},
    #[cfg(all(feature = "adc2", feature = "adc2_c1_c11"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C1,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_11},
    #[cfg(all(feature = "adc2", feature = "adc2_c2_c12"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C2,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_12},
    #[cfg(all(feature = "adc2", feature = "adc2_c3_c13"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C3,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_13},
    #[cfg(all(feature = "adc2", feature = "adc2_c4_c14"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C4,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_14},
    #[cfg(all(feature = "adc2", feature = "adc2_c5_c15"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C5,  base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_2 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_15},

    #[cfg(all(feature = "adc3", feature = "adc3_a0_c0"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A0,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_0},
    #[cfg(all(feature = "adc3", feature = "adc3_a1_c1"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A1,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_1},
    #[cfg(all(feature = "adc3", feature = "adc3_a2_c2"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A2,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_2},
    #[cfg(all(feature = "adc3", feature = "adc3_a3_c3"))]
    hal_ll_adc_pin_map_t{pin: GPIO_A3,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_3},
    #[cfg(all(feature = "adc3", feature = "adc3_f6_c4"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F6,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_4},
    #[cfg(all(feature = "adc3", feature = "adc3_f7_c5"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F7,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_5},
    #[cfg(all(feature = "adc3", feature = "adc3_f8_c6"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F8,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_6},
    #[cfg(all(feature = "adc3", feature = "adc3_f9_c7"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F9,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_7},
    #[cfg(all(feature = "adc3", feature = "adc3_f10_c8"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F10,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_8},
    #[cfg(all(feature = "adc3", feature = "adc3_f3_c9"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F3,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_9},
    #[cfg(all(feature = "adc3", feature = "adc3_c0_c10"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C0,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_10},
    #[cfg(all(feature = "adc3", feature = "adc3_c1_c11"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C1,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_11},
    #[cfg(all(feature = "adc3", feature = "adc3_c2_c12"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C2,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_12},
    #[cfg(all(feature = "adc3", feature = "adc3_c3_c13"))]
    hal_ll_adc_pin_map_t{pin: GPIO_C3,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_13},
    #[cfg(all(feature = "adc3", feature = "adc3_f4_c14"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F4,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_14},
    #[cfg(all(feature = "adc3", feature = "adc3_f5_c15"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F5,  base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_3 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_15},
    
    hal_ll_adc_pin_map_t{pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC}
];

#[inline(always)]
pub const fn hal_ll_adc_module_num(module: u8) -> u8 {
    module - 1
}