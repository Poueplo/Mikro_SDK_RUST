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


pub use crate::gpio::*;

#[cfg(feature = "tim2")]
pub const HAL_LL_TIM2_GPIO_AF1 : u8 = 1;
#[cfg(feature = "tim3")]
pub const HAL_LL_TIM3_GPIO_AF2 : u8 = 2;
#[cfg(feature = "tim4")]
pub const HAL_LL_TIM4_GPIO_AF2 : u8 = 2;
#[cfg(feature = "tim5")]
pub const HAL_LL_TIM5_GPIO_AF2 : u8 = 2;
#[cfg(feature = "tim9")]
pub const HAL_LL_TIM9_GPIO_AF3 : u8 = 3;
#[cfg(feature = "tim10")]
pub const HAL_LL_TIM10_GPIO_AF3 : u8 = 3;
#[cfg(feature = "tim11")]
pub const HAL_LL_TIM11_GPIO_AF3 : u8 = 3;


#[cfg(feature = "tim2")]
pub const TIM2_BASE_ADDR : u32 = 0x4000_0000;
#[cfg(feature = "tim3")]
pub const TIM3_BASE_ADDR : u32 = 0x4000_0400;
#[cfg(feature = "tim4")]
pub const TIM4_BASE_ADDR : u32 = 0x4000_0800;
#[cfg(feature = "tim5")]
pub const TIM5_BASE_ADDR : u32 = 0x4000_0C00;
#[cfg(feature = "tim9")]
pub const TIM9_BASE_ADDR : u32 = 0x4001_4000;
#[cfg(feature = "tim10")]
pub const TIM10_BASE_ADDR : u32 = 0x4001_4400;
#[cfg(feature = "tim11")]
pub const TIM11_BASE_ADDR : u32 = 0x4001_4800;


#[cfg(feature = "tim2")]
pub const TIM2_BUS: u8 = 1;
#[cfg(feature = "tim3")]
pub const TIM3_BUS: u8 = 1;
#[cfg(feature = "tim4")]
pub const TIM4_BUS: u8 = 1;
#[cfg(feature = "tim5")]
pub const TIM5_BUS: u8 = 1;
#[cfg(feature = "tim9")]
pub const TIM9_BUS: u8 = 2;
#[cfg(feature = "tim10")]
pub const TIM10_BUS: u8 = 2;
#[cfg(feature = "tim11")]
pub const TIM11_BUS: u8 = 2;


#[repr(u8)]
#[derive(Clone, Copy)]
pub enum tim_modules {
    NULL_MODULE, //is present to ensure the enum is never empty
    #[cfg(feature = "tim2")]
    TIM_MODULE_2,
    #[cfg(feature = "tim3")]
    TIM_MODULE_3,
    #[cfg(feature = "tim4")]
    TIM_MODULE_4,
    #[cfg(feature = "tim5")]
    TIM_MODULE_5,
    #[cfg(feature = "tim9")]
    TIM_MODULE_9,
    #[cfg(feature = "tim10")]
    TIM_MODULE_10,
    #[cfg(feature = "tim11")]
    TIM_MODULE_11,
}

pub const TIM_MODULE_COUNT: u8 = 0
    + if cfg!(feature = "tim2") { 1 } else { 0 }
    + if cfg!(feature = "tim3") { 1 } else { 0 }
    + if cfg!(feature = "tim4") { 1 } else { 0 }
    + if cfg!(feature = "tim5") { 1 } else { 0 }
    + if cfg!(feature = "tim9") { 1 } else { 0 }
    + if cfg!(feature = "tim10") { 1 } else { 0 }
    + if cfg!(feature = "tim11") { 1 } else { 0 };

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_tim_channel_t {
    HAL_LL_TIM_CHANNEL_1,
    HAL_LL_TIM_CHANNEL_2,
    HAL_LL_TIM_CHANNEL_3,
    HAL_LL_TIM_CHANNEL_4,
    HAL_LL_TIM_CHANNEL_NONE,
}

pub struct hal_ll_tim_pin_map_t {
    pub pin: hal_ll_pin_name_t,
    pub base: hal_ll_base_addr_t,
    pub channel: hal_ll_tim_channel_t,
    pub af: u32,
    pub module_index: u8
}

pub static _tim_map : &[hal_ll_tim_pin_map_t] = & [
    //TIM2
    #[cfg(all(feature = "tim2", feature = "tim2_ch1_a0_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A0, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch1_a5_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A5, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch1_a15_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A15, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch1_e9_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E9, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch2_a1_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A1, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch2_b3_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B3, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch2_e10_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E10, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch3_a2_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch3_b10_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B10, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch3_e11_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E11, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch4_a3_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch4_b11_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B11, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },
    #[cfg(all(feature = "tim2", feature = "tim2_ch4_e12_af1"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E12, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_2 as u8) },

    //TIM3
    #[cfg(all(feature = "tim3", feature = "tim3_ch1_a6_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A6, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch1_b4_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B4, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch1_c6_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_C6, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch1_e3_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E3, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch2_a7_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch2_b5_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B5, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch2_c7_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_C7, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch2_e4_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E4, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch3_b0_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B0, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch3_c8_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_C8, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch4_b1_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B1, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },
    #[cfg(all(feature = "tim3", feature = "tim3_ch4_c9_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_C9, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_3 as u8) },

    //TIM4
    #[cfg(all(feature = "tim4", feature = "tim4_ch1_b6_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B6, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch1_d12_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D12, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch2_b7_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B7, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch2_d13_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D13, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch3_b8_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B8, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch3_d14_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D14, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch4_b9_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B9, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },
    #[cfg(all(feature = "tim4", feature = "tim4_ch4_d15_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D15, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_4 as u8) },


    //TIM5
    #[cfg(all(feature = "tim5", feature = "tim5_ch1_a0_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A0, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch1_f6_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_F6, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch2_a1_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A1, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch2_f7_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_F7, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch3_a2_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch3_f8_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_F8, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch4_a3_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },
    #[cfg(all(feature = "tim5", feature = "tim5_ch4_f9_af2"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_F9, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_5 as u8) },

    //TIM9
    #[cfg(all(feature = "tim9", feature = "tim9_ch1_a2_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch1_b13_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B13, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch1_d0_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D0, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch1_e5_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E5, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch2_a3_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch2_b14_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B14, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch2_d7_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_D7, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },
    #[cfg(all(feature = "tim9", feature = "tim9_ch2_e6_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E6, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_9 as u8) },

    //TIM10
    #[cfg(all(feature = "tim10", feature = "tim10_ch1_a6_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A6, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_10 as u8) },
    #[cfg(all(feature = "tim10", feature = "tim10_ch1_b12_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B12, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_10 as u8) },
    #[cfg(all(feature = "tim10", feature = "tim10_ch1_b8_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B8, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_10 as u8) },
    #[cfg(all(feature = "tim10", feature = "tim10_ch1_e0_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E0, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_10 as u8) },

    //TIM11
    #[cfg(all(feature = "tim11", feature = "tim11_ch1_a7_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_11 as u8) },
    #[cfg(all(feature = "tim11", feature = "tim11_ch1_b15_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B15, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_11 as u8) },
    #[cfg(all(feature = "tim11", feature = "tim11_ch1_b9_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_B9, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_11 as u8) },
    #[cfg(all(feature = "tim11", feature = "tim11_ch1_e1_af3"))]
    hal_ll_tim_pin_map_t{ pin: GPIO_E1, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(tim_modules::TIM_MODULE_11 as u8) },

    hal_ll_tim_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: HAL_LL_PIN_NC as u32 as u32, module_index: HAL_LL_PIN_NC },
];

pub const fn hal_ll_tim_module_num(module: u8) -> u8 {
    module - 1
}