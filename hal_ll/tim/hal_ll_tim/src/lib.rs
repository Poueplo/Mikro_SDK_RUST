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

use hal_ll_target::*;
use hal_ll_tim_pin_map::*;
use hal_ll_gpio::*;
use hal_ll_gpio::gpio_constants::*;
use system::{rcc_get_clocks_frequency, RCC_ClocksTypeDef};
use core::fmt;

const HAL_LL_TIM_ENABLE_COUNTER_BIT: u32 = 0;
const HAL_LL_TIM_BDTR_MOE_BIT: u32 = 15;

const HAL_LL_TIM_CCMR_CCS_OUTPUT: u32 = 0x3;
const HAL_LL_TIM_CCMR_OCP: u32 = 0x8;
const HAL_LL_TIM_CCMR_OCM: u32 = 0x70;
const HAL_LL_TIM_CCMR_OCM_PWM_MODE_1: u32 = 0x60;

const HAL_LL_TIM_CR1_DIR_BIT: u32 = 4;

//APB1
const HAL_LL_TIM_ENABLE_2: u8 = 0;
const HAL_LL_TIM_ENABLE_3: u8 = 1;
const HAL_LL_TIM_ENABLE_4: u8 = 2;
const HAL_LL_TIM_ENABLE_5: u8 = 3;
const HAL_LL_TIM_ENABLE_12: u8 = 6;
const HAL_LL_TIM_ENABLE_13: u8 = 7;
const HAL_LL_TIM_ENABLE_14: u8 = 8;

//APB2
const HAL_LL_TIM_ENABLE_1: u8 = 0;
const HAL_LL_TIM_ENABLE_8: u8 = 1;
const HAL_LL_TIM_ENABLE_9: u8 = 16;
const HAL_LL_TIM_ENABLE_10: u8 = 17;
const HAL_LL_TIM_ENABLE_11: u8 = 18;

const HAL_LL_APB1_TIMER_CLOCK: u8 = 1;
const HAL_LL_APB2_TIMER_CLOCK: u8 = 2;


#[derive(Debug)]
pub enum HAL_LL_TIM_ERROR {
    TIM_WRONG_PIN,
    ACQUIRE_FAIL,
    TIM_MODULE_ERROR,
}

impl fmt::Display for HAL_LL_TIM_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TIM_WRONG_PIN => write!(f, "TIM_WRONG_PIN occurred"),                  
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),                  
            Self::TIM_MODULE_ERROR => write!(f, "TIM_MODULE_ERROR occurred"),                  
        }
    }
}

type Result<T> = core::result::Result<T, HAL_LL_TIM_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_ll_tim_handle_register_t
{
    pub tim_handle : handle_t,
    pub init_ll_state: bool
}

impl Default for hal_ll_tim_handle_register_t {
    fn default() -> Self {
        Self {
            tim_handle : 0,
            init_ll_state : false
        }
    }
}

#[repr(C)]
struct hal_ll_tim_base_handle_t
{
    pub cr1: hal_ll_base_addr_t,            /* Address offset 0x00 */
    pub cr2: hal_ll_base_addr_t,            /* Address offset 0x04 */
    pub smcr: hal_ll_base_addr_t,           /* Address offset 0x08 */
    pub dier: hal_ll_base_addr_t,           /* Address offset 0x0C */
    pub sr: hal_ll_base_addr_t,             /* Address offset 0x10 */
    pub egr: hal_ll_base_addr_t,            /* Address offset 0x14 */
    pub ccmr1: hal_ll_base_addr_t,          /* Address offset 0x18 */
    pub ccmr2: hal_ll_base_addr_t,          /* Address offset 0x1C */
    pub ccer: hal_ll_base_addr_t,           /* Address offset 0x20 */
    pub cnt: hal_ll_base_addr_t,            /* Address offset 0x24 */
    pub psc: hal_ll_base_addr_t,            /* Address offset 0x28 */
    pub arr: hal_ll_base_addr_t,            /* Address offset 0x2C */
    pub rcr: hal_ll_base_addr_t,            /* Address offset 0x30 */
    pub ccr1: hal_ll_base_addr_t,           /* Address offset 0x34 */
    pub ccr2: hal_ll_base_addr_t,           /* Address offset 0x38 */
    pub ccr3: hal_ll_base_addr_t,           /* Address offset 0x3C */
    pub ccr4: hal_ll_base_addr_t,           /* Address offset 0x40 */
    pub bdtr: hal_ll_base_addr_t,           /* Address offset 0x44 */
    pub dcr: hal_ll_base_addr_t,            /* Address offset 0x48 */
    pub dmar: hal_ll_base_addr_t,           /* Address offset 0x4C */
    pub tim2_5_or: hal_ll_base_addr_t,      /* Address offset 0x50 */
}

#[derive(Clone, Copy)]
struct hal_ll_tim_t {
    pub pin : hal_ll_pin_name_t,
    pub channel : hal_ll_tim_channel_t,
    pub af : u32,
}

struct hal_ll_tim_hw_specifics_map_t {
    pub base : hal_ll_base_addr_t,
    pub config : [hal_ll_tim_t; 8],
    pub max_period : u16,
    pub freq_hz : u32,
    pub module_index : hal_ll_pin_name_t,
}

static mut hal_ll_module_state: [hal_ll_tim_handle_register_t; TIM_MODULE_COUNT as usize]  = [ 
    hal_ll_tim_handle_register_t{
        tim_handle : 0, 
        init_ll_state : false
        };
        TIM_MODULE_COUNT as usize];

static mut hal_ll_tim_hw_specifics_map: [hal_ll_tim_hw_specifics_map_t; (TIM_MODULE_COUNT + 1) as usize] = [
    hal_ll_tim_hw_specifics_map_t{ base: TIM1_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_1) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM2_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_2) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM3_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_3) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM4_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_4) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM5_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_5) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM8_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_8) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM9_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_9) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM10_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_10) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM11_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_11) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM12_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_12) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM13_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_13) },
    hal_ll_tim_hw_specifics_map_t{ base: TIM14_BASE_ADDR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: hal_ll_tim_module_num(TIM_MODULE_14) },
    hal_ll_tim_hw_specifics_map_t{ base: HAL_LL_MODULE_ERROR, config: [hal_ll_tim_t{ pin: HAL_LL_PIN_NC, channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, af: 0 }; 8], max_period: 0, freq_hz: 0, module_index: HAL_LL_MODULE_ERROR as u8},
];

///////// public functions
pub fn hal_ll_tim_register_handle(pin: hal_ll_pin_name_t, hal_module_id: &mut u8) -> Result<hal_ll_tim_handle_register_t> {
    let pin_check_result: u8;
    let mut index_list: u8 = 0;
    let mut config_index: u8 = 8;

    pin_check_result = hal_ll_tim_check_pins(pin, &mut index_list, &mut config_index);
    if pin_check_result == HAL_LL_PIN_NC {
        return Err(HAL_LL_TIM_ERROR::TIM_WRONG_PIN);
    }
    
    unsafe{
        if hal_ll_tim_hw_specifics_map[pin_check_result as usize].config[config_index as usize].pin != pin {
            hal_ll_tim_alternate_functions_set_state(&mut hal_ll_tim_hw_specifics_map[pin_check_result as usize], false );
            hal_ll_tim_map_pins( pin_check_result as usize, config_index as usize, index_list);
            hal_ll_tim_alternate_functions_set_state(&mut hal_ll_tim_hw_specifics_map[pin_check_result as usize], true );
        
            hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        }

        *hal_module_id = pin_check_result;

        hal_ll_module_state[pin_check_result as usize].tim_handle = hal_ll_tim_hw_specifics_map[pin_check_result as usize].base;
        
        Ok(hal_ll_module_state[pin_check_result as usize])
    }
}

pub fn hal_ll_tim_set_freq(handle: &mut hal_ll_tim_handle_register_t, freq_hz: u32) {
    let hal_handle : &mut hal_ll_tim_handle_register_t = handle;
    let hal_ll_tim_hw_specifics_map_local: &mut hal_ll_tim_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let tim_ptr : *mut hal_ll_tim_base_handle_t = hal_ll_tim_hw_specifics_map_local.base as *mut hal_ll_tim_base_handle_t;

    let tmp_freq: u32;
    let local_base_addr: u32;
    let ck_psc: u32;

    hal_handle.init_ll_state = false;

    unsafe{
        for channel in 0x00 .. (hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 / 2) {
            *((&(*tim_ptr).ccr1 as *const u32 as u32 + channel as u32 * 0x4) as *mut u32) = 0;
        }

        local_base_addr = hal_ll_tim_hw_specifics_map_local.base;

        ck_psc = hal_ll_tim_get_clock_speed( local_base_addr ) / freq_hz;

        if ck_psc > 0xFFFF {
            tmp_freq =  hal_ll_tim_get_clock_speed( local_base_addr ) / 0xFFFF;
            hal_ll_tim_hw_specifics_map_local.freq_hz = tmp_freq;
        } else {
            hal_ll_tim_hw_specifics_map_local.freq_hz = freq_hz;
        }

    }

    hal_ll_tim_init(hal_ll_tim_hw_specifics_map_local);

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_tim_set_duty(handle: &mut hal_ll_tim_handle_register_t, pin: hal_ll_pin_name_t, duty_ratio: f32) {
    let hal_handle : &mut hal_ll_tim_handle_register_t = handle;
    let hal_ll_tim_hw_specifics_map_local: &mut hal_ll_tim_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let tim_ptr : *mut hal_ll_tim_base_handle_t = hal_ll_tim_hw_specifics_map_local.base as *mut hal_ll_tim_base_handle_t;

    let tmp_duty: f32;
    let max_duty: u32;
    let max_period: u16;
    let mut channel: hal_ll_tim_channel_t = hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE;

    tmp_duty = duty_ratio * 100.0;
    max_period = hal_ll_tim_hw_specifics_map_local.max_period;
    max_duty = ((max_period as f32 / 100.0) * tmp_duty) as u32;

    for config_index in 0x00 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
        if hal_ll_tim_hw_specifics_map_local.config[config_index as usize].pin == pin {
            channel = hal_ll_tim_hw_specifics_map_local.config[config_index as usize].channel;
        }
    }

    if channel == hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE {
        return; //error pin not used
    }

    unsafe{
        *((&(*tim_ptr).ccr1 as *const u32 as u32 + (channel as u32 / 2) * 0x4) as *mut u32) = max_duty;
    }

}

pub fn hal_ll_tim_start(handle: &mut hal_ll_tim_handle_register_t, pin: hal_ll_pin_name_t) {
    let hal_handle : &mut hal_ll_tim_handle_register_t = handle;
    let hal_ll_tim_hw_specifics_map_local: &mut hal_ll_tim_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let tim_ptr : *mut hal_ll_tim_base_handle_t = hal_ll_tim_hw_specifics_map_local.base as *mut hal_ll_tim_base_handle_t;
    let mut channel: hal_ll_tim_channel_t = hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE;
    let tmp_ptr: *mut u32;
    let offset: u8;

    for config_index in 0x00 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
        if hal_ll_tim_hw_specifics_map_local.config[config_index as usize].pin == pin {
            channel = hal_ll_tim_hw_specifics_map_local.config[config_index as usize].channel;
        }
    }

    unsafe{
        //disable channel
        clear_reg_bit(&(*tim_ptr).ccer as *const u32 as u32, (channel as u32) * 2);

        set_reg_bit(&(*tim_ptr).bdtr as *const u32 as u32, HAL_LL_TIM_BDTR_MOE_BIT);

        if (channel as u8) < (hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_3 as u8) {
            tmp_ptr = &(*tim_ptr).ccmr1 as *const u32 as *mut u32;
            offset = ((channel as u8) / 2) * 8;
        } else {
            tmp_ptr = &(*tim_ptr).ccmr2 as *const u32 as *mut u32;
            offset = (((channel as u8) / 2) - 2) * 8;
        }

        *tmp_ptr &= !((HAL_LL_TIM_CCMR_CCS_OUTPUT | HAL_LL_TIM_CCMR_OCP | HAL_LL_TIM_CCMR_OCM) << offset);
        *tmp_ptr |= (HAL_LL_TIM_CCMR_OCM_PWM_MODE_1) << offset;

        //enable channel
        set_reg_bit(&(*tim_ptr).ccer as *const u32 as u32, (channel as u32) * 2);
        clear_reg_bit(&(*tim_ptr).ccer as *const u32 as u32, ((channel as u32) * 2) + 1);

        //enable timer
        set_reg_bit(&(*tim_ptr).cr1 as *const u32 as u32, HAL_LL_TIM_ENABLE_COUNTER_BIT);
    }

}

pub fn hal_ll_tim_stop(handle: &mut hal_ll_tim_handle_register_t, pin: hal_ll_pin_name_t) {
    let hal_handle : &mut hal_ll_tim_handle_register_t = handle;
    let hal_ll_tim_hw_specifics_map_local: &mut hal_ll_tim_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let tim_ptr : *mut hal_ll_tim_base_handle_t = hal_ll_tim_hw_specifics_map_local.base as *mut hal_ll_tim_base_handle_t;
    let mut channel: hal_ll_tim_channel_t = hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE;
    
    for config_index in 0x00 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
        if hal_ll_tim_hw_specifics_map_local.config[config_index as usize].pin == pin {
            channel = hal_ll_tim_hw_specifics_map_local.config[config_index as usize].channel;
        }
    }
    unsafe {
        clear_reg_bit(&(*tim_ptr).ccer as *const u32 as u32, (channel as u32) * 2);
    }
}

pub fn hal_ll_tim_close(handle: &mut hal_ll_tim_handle_register_t) {
    let hal_handle : &mut hal_ll_tim_handle_register_t = handle;
    let mut hal_ll_tim_hw_specifics_map_local: &mut hal_ll_tim_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_tim_hw_specifics_map_local.module_index as usize;

    if hal_handle.tim_handle != 0 {
        *hal_handle = hal_ll_tim_handle_register_t::default();

        hal_ll_tim_hw_specifics_map_local.max_period = 0;
        hal_ll_tim_hw_specifics_map_local.freq_hz = 0;

        hal_ll_tim_set_clock(hal_ll_tim_hw_specifics_map_local, true);
        hal_ll_tim_alternate_functions_set_state(&mut hal_ll_tim_hw_specifics_map_local, false );
        hal_ll_tim_set_clock(hal_ll_tim_hw_specifics_map_local, false);

        hal_ll_tim_hw_specifics_map_local.config = [
            hal_ll_tim_t{   pin: HAL_LL_PIN_NC, 
                            channel: hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE, 
                            af: 0 }; 
                8];
        
        unsafe{hal_ll_module_state[pin_check_result as usize] = *hal_handle;}
    }
}

///////// private functions
#[allow(unused)]
fn hal_ll_tim_check_pins(pin: hal_ll_pin_name_t, index: &mut u8, config_index: &mut u8) -> u8
{
    let tim_map_size: u8 = _tim_map.len() as u8 ;
    let mut index_counter: u8 = 0;
    let mut hal_ll_module_id: u8 = 0;
    let mut hal_ll_channel: hal_ll_tim_channel_t = hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE;
    let mut is_channel_taken: bool = false;
    let mut tmp_config_index: u8 = 0;

    if HAL_LL_PIN_NC == pin {
        return HAL_LL_PIN_NC;
    }

    //check if pin is already taken
    for module_index in 0x00 .. TIM_MODULE_COUNT {
        for config_index in 0x00 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
            if  pin == unsafe{hal_ll_tim_hw_specifics_map[module_index as usize].config[config_index as usize].pin}  {               
                return HAL_LL_PIN_NC;
            }
        }
    }

    for  pin_index in 0x00 .. tim_map_size
    {
        if pin == _tim_map[pin_index as usize].pin
        {
            // Get module number
            hal_ll_module_id = _tim_map[pin_index as usize].module_index;
            // get pin channel
            hal_ll_channel = _tim_map[pin_index as usize].channel;
            // Map pin name
            *index = pin_index;

            // Check if module is not taken
            if  hal_ll_tim_handle_register_t::default().tim_handle == unsafe{hal_ll_module_state[hal_ll_module_id as usize].tim_handle}  {               
                *config_index = hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 - 1;
                return hal_ll_module_id;
            }
            
            //check if channel is taken
            is_channel_taken = false;
            for config_counter in 0x00 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
                if  hal_ll_channel == unsafe{hal_ll_tim_hw_specifics_map[hal_ll_module_id as usize].config[config_counter as usize].channel}  {               
                    is_channel_taken = true;
                }

                if  unsafe{hal_ll_tim_hw_specifics_map[hal_ll_module_id as usize].config[config_counter as usize].channel} == hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE {               
                    tmp_config_index = config_counter;
                }
            }
            
            if !is_channel_taken {
                *config_index = tmp_config_index;
                return hal_ll_module_id;
            } else {
                index_counter = index_counter + 1;
                if TIM_MODULE_COUNT == index_counter {
                    return index_counter - 1;
                }
            }
        }
    }

    if  index_counter > 0 {
        return hal_ll_module_id;
    } else {
        return HAL_LL_PIN_NC;
    }
}

#[allow(unused_variables, unused_assignments)]
fn hal_ll_get_specifics<'a>(handle: hal_ll_tim_handle_register_t) -> &'a mut hal_ll_tim_hw_specifics_map_t {
    let mut hal_ll_module_count: usize = SPI_MODULE_COUNT as usize;
    let mut hal_ll_module_error : usize = 0;
    hal_ll_module_error = hal_ll_module_count;
    
    unsafe{
        while hal_ll_module_count > 0 {
            hal_ll_module_count -= 1;

            let base: u32 = handle.tim_handle;

            if base == hal_ll_tim_hw_specifics_map[hal_ll_module_count].base {
                return &mut hal_ll_tim_hw_specifics_map[hal_ll_module_count];
            }
        }

        return &mut hal_ll_tim_hw_specifics_map[hal_ll_module_error];
    }
}

fn hal_ll_tim_clock_source(selector : u8) -> u32 {
    let mut rcc_clocks : RCC_ClocksTypeDef = RCC_ClocksTypeDef{
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0
        };

    rcc_get_clocks_frequency( &mut rcc_clocks );

    if selector == HAL_LL_APB1_TIMER_CLOCK {
        if rcc_clocks.HCLK_Frequency / rcc_clocks.PCLK1_Frequency > 1 {
            return rcc_clocks.PCLK1_Frequency * 2;
        } else {
            return rcc_clocks.PCLK1_Frequency;
        }
    } else if selector == HAL_LL_APB2_TIMER_CLOCK {
        if rcc_clocks.HCLK_Frequency / rcc_clocks.PCLK2_Frequency > 1 {
            return rcc_clocks.PCLK2_Frequency * 2;
        } else {
            return rcc_clocks.PCLK2_Frequency;
        }
    }

    0
}

fn hal_ll_tim_get_clock_speed(base : hal_ll_base_addr_t) -> u32 {
    match base {
        TIM1_BASE_ADDR => hal_ll_tim_clock_source(TIM1_BUS),
        TIM2_BASE_ADDR => hal_ll_tim_clock_source(TIM2_BUS),
        TIM3_BASE_ADDR => hal_ll_tim_clock_source(TIM3_BUS),
        TIM4_BASE_ADDR => hal_ll_tim_clock_source(TIM4_BUS),
        TIM5_BASE_ADDR => hal_ll_tim_clock_source(TIM5_BUS),
        TIM8_BASE_ADDR => hal_ll_tim_clock_source(TIM8_BUS),
        TIM9_BASE_ADDR => hal_ll_tim_clock_source(TIM9_BUS),
        TIM10_BASE_ADDR => hal_ll_tim_clock_source(TIM10_BUS),
        TIM11_BASE_ADDR => hal_ll_tim_clock_source(TIM11_BUS),
        TIM12_BASE_ADDR => hal_ll_tim_clock_source(TIM12_BUS),
        TIM13_BASE_ADDR => hal_ll_tim_clock_source(TIM13_BUS),
        TIM14_BASE_ADDR => hal_ll_tim_clock_source(TIM14_BUS),
        _ => 0,
    }
}

fn hal_ll_tim_map_pins(module_index: usize, config_index: usize, index: u8) {
    unsafe{
        // Map new pins
        hal_ll_tim_hw_specifics_map[module_index as usize].config[config_index].pin = _tim_map[ index as usize].pin;
        hal_ll_tim_hw_specifics_map[module_index as usize].config[config_index].af = _tim_map[ index as usize].af;
        hal_ll_tim_hw_specifics_map[module_index as usize].config[config_index].channel = _tim_map[ index as usize].channel;
    }
}

fn hal_ll_tim_set_clock(map: &mut hal_ll_tim_hw_specifics_map_t, hal_ll_state: bool) {
    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_1) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_1 as u32);
        } else {
            clear_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_1 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_2) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_2 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_2 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_3) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_3 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_3 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_4) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_4 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_4 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_5) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_5 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_5 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_8) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_8 as u32);
        } else {
            clear_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_8 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_9) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_9 as u32);
        } else {
            clear_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_9 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_10) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_10 as u32);
        } else {
            clear_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_10 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_11) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_11 as u32);
        } else {
            clear_reg_bit( RCC_APB2ENR, HAL_LL_TIM_ENABLE_11 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_12) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_12 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_12 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_13) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_13 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_13 as u32);
        }
    }

    if map.module_index == hal_ll_tim_module_num(TIM_MODULE_14) {
        if hal_ll_state  {
            set_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_14 as u32);
        } else {
            clear_reg_bit( RCC_APB1ENR, HAL_LL_TIM_ENABLE_14 as u32);
        }
    }


}

fn hal_ll_tim_alternate_functions_set_state(map: &mut hal_ll_tim_hw_specifics_map_t, hal_ll_state: bool) {
    let mut module: module_struct = module_struct::default();
    let mut module_index: u8 = 0;
    let tim_config : u32;

    if hal_ll_state {
        tim_config = GPIO_CFG_MODE_ALT_FUNCTION |
                     GPIO_CFG_SPEED_HIGH |
                     GPIO_CFG_OTYPE_PP;
    } else {
        tim_config = GPIO_CFG_MODE_ALT_FUNCTION |
                     GPIO_CFG_SPEED_LOW |
                     GPIO_CFG_OTYPE_PP;
    }

    for config_index in 0x0 .. hal_ll_tim_channel_t::HAL_LL_TIM_CHANNEL_NONE as u8 {
        if (*map).config[config_index as usize].pin != HAL_LL_PIN_NC  {
            module.pins[0 + module_index as usize] = VALUE( (*map).config[config_index as usize].pin, (*map).config[config_index as usize].af );
            module.configs[0 + module_index as usize] = tim_config;
            
            module_index += 1;
        }
    }

    if module.pins[0] != GPIO_MODULE_STRUCT_END {
        hal_ll_gpio_module_struct_init( &mut module, hal_ll_state );
    }
}

fn hal_ll_tim_hw_init(map: &mut hal_ll_tim_hw_specifics_map_t) {
    let tim_ptr : *mut hal_ll_tim_base_handle_t = map.base as *mut hal_ll_tim_base_handle_t;
    let mut ck_psc: u32;

    unsafe{
        clear_reg_bit(&(*tim_ptr).cr1 as *const u32 as u32, HAL_LL_TIM_CR1_DIR_BIT);

        ck_psc = hal_ll_tim_get_clock_speed(map.base) / map.freq_hz;

        if ck_psc > 0xFFFF {
            ck_psc = 0xFFFF - 1;
        } else if ck_psc < 1 {
            ck_psc = 1;
        }

        (*tim_ptr).psc = 0;
        (*tim_ptr).arr = ck_psc & 0xFFFF;
        
        map.max_period = (ck_psc & 0xFFFF) as u16;
    }
}

fn hal_ll_tim_init(map: &mut hal_ll_tim_hw_specifics_map_t) {
    hal_ll_tim_set_clock(map, true);
    hal_ll_tim_alternate_functions_set_state(map, true);
    hal_ll_tim_hw_init(map);
}