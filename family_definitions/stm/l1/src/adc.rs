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
    ADC_MODULE_1
}


pub const ADC_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "adc1") { 1 } else { 0 };


pub const ADC1_BASE_ADDR: u32 = 0x4001_2400;

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

    HAL_LL_ADC_CHANNEL_B,
    
    HAL_LL_ADC_CHANNEL_0B,
    HAL_LL_ADC_CHANNEL_1B,
    HAL_LL_ADC_CHANNEL_2B,
    HAL_LL_ADC_CHANNEL_3B,
    HAL_LL_ADC_CHANNEL_4B,
    HAL_LL_ADC_CHANNEL_5B,
    HAL_LL_ADC_CHANNEL_6B,
    HAL_LL_ADC_CHANNEL_7B,
    HAL_LL_ADC_CHANNEL_8B,
    HAL_LL_ADC_CHANNEL_9B,
    HAL_LL_ADC_CHANNEL_10B,
    HAL_LL_ADC_CHANNEL_11B,
    HAL_LL_ADC_CHANNEL_12B,
    HAL_LL_ADC_CHANNEL_13B,
    HAL_LL_ADC_CHANNEL_14B,
    HAL_LL_ADC_CHANNEL_15B,
    HAL_LL_ADC_CHANNEL_16B,
    HAL_LL_ADC_CHANNEL_17B,
    HAL_LL_ADC_CHANNEL_18B,
    HAL_LL_ADC_CHANNEL_19B,
    HAL_LL_ADC_CHANNEL_20B,
    HAL_LL_ADC_CHANNEL_21B,
    HAL_LL_ADC_CHANNEL_22B,
    HAL_LL_ADC_CHANNEL_23B,
    HAL_LL_ADC_CHANNEL_24B,
    HAL_LL_ADC_CHANNEL_25B,
    HAL_LL_ADC_CHANNEL_26B,
    HAL_LL_ADC_CHANNEL_27B,
    HAL_LL_ADC_CHANNEL_28B,
    HAL_LL_ADC_CHANNEL_29B,
    HAL_LL_ADC_CHANNEL_30B,
    HAL_LL_ADC_CHANNEL_31B,

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
    
    #[cfg(all(feature = "adc1", feature = "adc1_b12_c18"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B12,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_18},
    #[cfg(all(feature = "adc1", feature = "adc1_b13_c19"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B13,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_19},
    #[cfg(all(feature = "adc1", feature = "adc1_b14_c20"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B14,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_20},
    #[cfg(all(feature = "adc1", feature = "adc1_b15_c21"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B15,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_21},
    #[cfg(all(feature = "adc1", feature = "adc1_e7_c22"))]
    hal_ll_adc_pin_map_t{pin: GPIO_E7,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_22},
    #[cfg(all(feature = "adc1", feature = "adc1_e8_c23"))]
    hal_ll_adc_pin_map_t{pin: GPIO_E8,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_23},
    #[cfg(all(feature = "adc1", feature = "adc1_e9_c24"))]
    hal_ll_adc_pin_map_t{pin: GPIO_E9,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_24},
    #[cfg(all(feature = "adc1", feature = "adc1_e10_c25"))]
    hal_ll_adc_pin_map_t{pin: GPIO_E10,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_25},

    #[cfg(all(feature = "adc1", feature = "adc1_b2_c0b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_B2,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_0B},
    #[cfg(all(feature = "adc1", feature = "adc1_f11_c1b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F11,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_1B},
    #[cfg(all(feature = "adc1", feature = "adc1_f12_c2b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F12,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_2B},
    #[cfg(all(feature = "adc1", feature = "adc1_f13_c3b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F13,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_3B},
    #[cfg(all(feature = "adc1", feature = "adc1_f14_c6b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F14,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_6B},
    #[cfg(all(feature = "adc1", feature = "adc1_f15_c7b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_F15,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_7B},
    #[cfg(all(feature = "adc1", feature = "adc1_g0_c8b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_G0,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_8B},
    #[cfg(all(feature = "adc1", feature = "adc1_g1_c9b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_G1,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_9B},
    #[cfg(all(feature = "adc1", feature = "adc1_g3_c11b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_G3,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_11B},
    #[cfg(all(feature = "adc1", feature = "adc1_g4_c12b"))]
    hal_ll_adc_pin_map_t{pin: GPIO_G4,  base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(adc_modules::ADC_MODULE_1 as u8), channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_12B},


    hal_ll_adc_pin_map_t{pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC}
];

#[inline(always)]
pub const fn hal_ll_adc_module_num(module: u8) -> u8 {
    module - 1
}