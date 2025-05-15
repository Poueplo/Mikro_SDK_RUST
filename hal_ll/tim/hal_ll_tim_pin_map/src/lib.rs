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


pub use hal_ll_pin_names::*;

pub const HAL_LL_TIM1_GPIO_AF1 : u8 = 1;
pub const HAL_LL_TIM2_GPIO_AF1 : u8 = 1;
pub const HAL_LL_TIM3_GPIO_AF2 : u8 = 2;
pub const HAL_LL_TIM4_GPIO_AF2 : u8 = 2;
pub const HAL_LL_TIM5_GPIO_AF2 : u8 = 2;
pub const HAL_LL_TIM8_GPIO_AF3 : u8 = 3;
pub const HAL_LL_TIM9_GPIO_AF3 : u8 = 3;
pub const HAL_LL_TIM10_GPIO_AF3 : u8 = 3;
pub const HAL_LL_TIM11_GPIO_AF3 : u8 = 3;
pub const HAL_LL_TIM12_GPIO_AF2 : u8 = 2;
pub const HAL_LL_TIM12_GPIO_AF9 : u8 = 9;
pub const HAL_LL_TIM13_GPIO_AF9 : u8 = 9;
pub const HAL_LL_TIM14_GPIO_AF9 : u8 = 9;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_tim_channel_t {
    HAL_LL_TIM_CHANNEL_1,
    HAL_LL_TIM_CHANNEL_1N,
    HAL_LL_TIM_CHANNEL_2,
    HAL_LL_TIM_CHANNEL_2N,
    HAL_LL_TIM_CHANNEL_3,
    HAL_LL_TIM_CHANNEL_3N,
    HAL_LL_TIM_CHANNEL_4,
    HAL_LL_TIM_CHANNEL_4N,
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
    //TIM1
    hal_ll_tim_pin_map_t{ pin: GPIO_A8, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E9, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A10, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E11, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A9, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E13, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A11, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E14, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },

    //TIM2
    hal_ll_tim_pin_map_t{ pin: GPIO_A0, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A5, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A15, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A1, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B3, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B10, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B11, base: TIM2_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM2_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },

    //TIM3
    hal_ll_tim_pin_map_t{ pin: GPIO_A6, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B4, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C6, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B5, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C7, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B0, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C8, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B1, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C9, base: TIM3_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM3_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },

    //TIM4
    hal_ll_tim_pin_map_t{ pin: GPIO_B6, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_D12, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B7, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_D13, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B8, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_D14, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B9, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_pin_map_t{ pin: GPIO_D15, base: TIM4_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM4_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },

    //TIM5
    hal_ll_tim_pin_map_t{ pin: GPIO_A0, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H10, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A1, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H11, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H12, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_pin_map_t{ pin: GPIO_I0, base: TIM5_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM5_GPIO_AF2 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },

    //TIM8
    hal_ll_tim_pin_map_t{ pin: GPIO_C6, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_I5, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C7, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_I6, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C8, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_I7, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_C9, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_pin_map_t{ pin: GPIO_I2, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_4, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },

    //TIM9
    hal_ll_tim_pin_map_t{ pin: GPIO_A2, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_9) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E5, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_9) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A3, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_9) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E6, base: TIM9_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM9_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_9) },

    //TIM10
    hal_ll_tim_pin_map_t{ pin: GPIO_B8, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_10) },
    hal_ll_tim_pin_map_t{ pin: GPIO_F6, base: TIM10_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM10_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_10) },

    //TIM11
    hal_ll_tim_pin_map_t{ pin: GPIO_B9, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_11) },
    hal_ll_tim_pin_map_t{ pin: GPIO_F7, base: TIM11_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM11_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_11) },

    //TIM12
    hal_ll_tim_pin_map_t{ pin: GPIO_B14, base: TIM12_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM12_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_12) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H6, base: TIM12_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM12_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_12) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B15, base: TIM12_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM12_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_12) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H9, base: TIM12_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2, af: HAL_LL_TIM12_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_12) },

    //TIM13
    hal_ll_tim_pin_map_t{ pin: GPIO_A6, base: TIM13_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM13_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_13) },
    hal_ll_tim_pin_map_t{ pin: GPIO_F8, base: TIM13_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM13_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_13) },

    //TIM14
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM14_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM14_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_14) },
    hal_ll_tim_pin_map_t{ pin: GPIO_F9, base: TIM14_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1, af: HAL_LL_TIM14_GPIO_AF9 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_14) },

    //COMPLEMENTARY
    //TIM1
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B13, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E8, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B0, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B14, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E10, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B1, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B15, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_pin_map_t{ pin: GPIO_E12, base: TIM1_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM1_GPIO_AF1 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },

    //TIM8
    hal_ll_tim_pin_map_t{ pin: GPIO_A5, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_A7, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H13, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_1N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B0, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B14, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H14, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_2N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B1, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_B15, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_pin_map_t{ pin: GPIO_H15, base: TIM8_BASE_ADDR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3N, af: HAL_LL_TIM8_GPIO_AF3 as u32, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },

    
    hal_ll_tim_pin_map_t{ pin: HAL_LL_PIN_NC, base: HAL_LL_MODULE_ERROR, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: HAL_LL_PIN_NC as u32 as u32, module_index: HAL_LL_PIN_NC },
];

pub const fn hal_ll_tim_module_num(module: u8) -> u8 {
    module - 1
}