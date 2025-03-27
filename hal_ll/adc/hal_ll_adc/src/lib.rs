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

use core::ptr;

use hal_ll_target::*;
use hal_ll_gpio_port::{hal_ll_gpio_analog_input,hal_ll_gpio_port_base,hal_ll_gpio_port_index,hal_ll_gpio_pin_mask};
use system::{rcc_get_clocks_frequency, delay_1us, RCC_ClocksTypeDef};
use hal_ll_adc_pin_map::*;
use core::fmt;

pub const HAL_LL_ADC_CCR_OFFSET: u32 = 0x304;

pub const HAL_LL_ADC_ADON_BIT: u32 = 0;
pub const HAL_LL_ADC_SR_EOC_BIT: u32 = 1;
pub const HAL_LL_ADC_CONT_BIT: u32 = 1;
pub const HAL_LL_ADC_SCAN_BIT: u32 = 8;
pub const HAL_LL_ADC_ALIGN_BIT: u32 = 11;
pub const HAL_LL_ADC_RESOLUTION_BIT0: u32 = 24;
pub const HAL_LL_ADC_RESOLUTION_BIT1: u32 = 25;
pub const HAL_LL_ADC_SWSTART_BIT: u32 = 30;

pub const HAL_LL_ADC1_ENABLE_CLOCK: u32 = 8;
pub const HAL_LL_ADC2_ENABLE_CLOCK: u32 = 9;
pub const HAL_LL_ADC3_ENABLE_CLOCK: u32 = 10;

pub const HAL_LL_ADC_CONT: u32 = 0x2;
pub const HAL_LL_ADC_ALIGN: u32 = 0x800;
pub const HAL_LL_ADC_JEXTSEL: u32 = 0xE0000;

pub const HAL_LL_ADC_60MHZ: u32 = 6000_0000;
pub const HAL_LL_ADC_120MHZ: u32 = 12000_0000;
pub const HAL_LL_ADC_180MHZ: u32 = 18000_0000;

pub const HAL_LL_ADC_PRESCALER_2: u32 = 0x0000_0000;
pub const HAL_LL_ADC_PRESCALER_4: u32 = 0x0001_0000;
pub const HAL_LL_ADC_PRESCALER_6: u32 = 0x0002_0000;
pub const HAL_LL_ADC_PRESCALER_8: u32 = 0x0003_0000;

pub const HAL_LL_ADC_PRESCALER_MASK: u32 = 0x00F0_0000;
pub const HAL_LL_ADC_SEQUENCE_LENGTH_MASK: u32 = 0x00F0_0000;
pub const HAL_LL_ADC_CONVERSION_ONE: u32 = 0x0000_001F;

pub const HAL_LL_ADC_12BIT_RES: u32 = 0x0000_0000;
pub const HAL_LL_ADC_10BIT_RES: u32 = 0x0100_0000;
pub const HAL_LL_ADC_8BIT_RES: u32 = 0x0200_0000;
pub const HAL_LL_ADC_6BIT_RES: u32 = 0x0300_0000;

pub const HAL_LL_RESOLUTION_MASK: u32 = 0xFCFFFFFF;

#[derive(Debug)]
pub enum HAL_LL_ADC_ERROR {
    HAL_LL_ADC_WRONG_PIN,
    HAL_LL_ADC_UNSUPPORTED_VREF,
    HAL_LL_ADC_UNSUPPORTED_RESOLUTION,
    HAL_LL_ADC_UNITIALISED_MODULE
}

impl fmt::Display for HAL_LL_ADC_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HAL_LL_ADC_WRONG_PIN => write!(f, "HAL_LL_ADC_WRONG_PIN occurred"),
            Self::HAL_LL_ADC_UNSUPPORTED_VREF => write!(f, "HAL_LL_ADC_UNSUPPORTED_VREF occurred"),
            Self::HAL_LL_ADC_UNSUPPORTED_RESOLUTION => write!(f, "HAL_LL_ADC_UNSUPPORTED_RESOLUTION occurred"),
            Self::HAL_LL_ADC_UNITIALISED_MODULE => write!(f, "HAL_LL_ADC_UNITIALISED_MODULE occurred"),
        }
    }
}

type Result<T> = core::result::Result<T, HAL_LL_ADC_ERROR>;

#[repr(C)]
pub struct hal_ll_adc_base_handle_t
{
    pub sr      : u32,
    pub cr1     : u32,
    pub cr2     : u32,
    pub smpr1   : u32,
    pub smpr2   : u32,
    pub jofr1   : u32,
    pub jofr2   : u32,
    pub jofr3   : u32,
    pub jofr4   : u32,
    pub htr     : u32,
    pub ltr     : u32,
    pub sqr1    : u32,
    pub sqr2    : u32,
    pub sqr3    : u32,
    pub jsqr    : u32,
    pub jdr1    : u32,
    pub jdr2    : u32,
    pub jdr3    : u32,
    pub jdr4    : u32,
    pub dr      : u32,
}

struct hal_ll_adc_pin_id
{
    pub pin_an : [u8; ADC_MODULE_COUNT as usize],
}

#[derive(Clone, Copy)]
pub enum hal_ll_adc_resolution_t
{
    HAL_LL_ADC_RESOLUTION_NOT_SET = 0,
    HAL_LL_ADC_RESOLUTION_6_BIT,     /* 6  bit resolution */
    HAL_LL_ADC_RESOLUTION_8_BIT,     /* 8  bit resolution */
    HAL_LL_ADC_RESOLUTION_10_BIT,    /* 10 bit resolution */
    HAL_LL_ADC_RESOLUTION_12_BIT,    /* 12 bit resolution */
    HAL_LL_ADC_RESOLUTION_14_BIT,    /* 14 bit resolution */
    HAL_LL_ADC_RESOLUTION_16_BIT,    /* 16 bit resolution */
}

pub const HAL_LL_ADC_RESOLUTION_DEFAULT: hal_ll_adc_resolution_t = hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_12_BIT;

#[derive(Clone, Copy)]
pub enum hal_ll_adc_voltage_reference_t {
    HAL_LL_ADC_VREF_EXTERNAL = 0,
}

pub const HAL_LL_ADC_VREF_DEFAULT: hal_ll_adc_voltage_reference_t = hal_ll_adc_voltage_reference_t::HAL_LL_ADC_VREF_EXTERNAL;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_ll_adc_handle_register_t
{
    pub hal_ll_adc_handle : handle_t,
    pub init_ll_state : bool
}

impl Default for hal_ll_adc_handle_register_t {
    fn default() -> Self {
        Self {
            hal_ll_adc_handle : 0,
            init_ll_state : false
        }
    }
}


#[derive(Clone, Copy)]
pub struct hal_ll_adc_hw_specifics_map_t {
    pub base : hal_ll_base_addr_t,
    pub module_index : u8,
    pub pin : hal_ll_pin_name_t,
    pub vref_input : hal_ll_adc_voltage_reference_t,
    pub vref_value : f32,
    pub resolution : u32,
    pub channel : hal_ll_adc_channel_t
}

impl Default for hal_ll_adc_hw_specifics_map_t {
    fn default() -> Self {
        Self { base: 0, module_index: 0, pin: 0, vref_input: HAL_LL_ADC_VREF_DEFAULT, vref_value: 0.0, resolution: HAL_LL_ADC_12BIT_RES, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC}
    }
}

static mut hal_ll_module_state: [hal_ll_adc_handle_register_t; ADC_MODULE_COUNT as usize]  = [ 
    hal_ll_adc_handle_register_t{
        hal_ll_adc_handle : 0, 
        init_ll_state : false
        };
    ADC_MODULE_COUNT as usize];

static mut hal_ll_adc_hw_specifics_map:[hal_ll_adc_hw_specifics_map_t; (ADC_MODULE_COUNT+1) as usize] = [
    hal_ll_adc_hw_specifics_map_t{base: ADC1_BASE_ADDR, module_index: hal_ll_adc_module_num(ADC_MODULE_1), pin: HAL_LL_PIN_NC, vref_input: HAL_LL_ADC_VREF_DEFAULT, vref_value: 0.0, resolution: HAL_LL_ADC_10BIT_RES, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC},
    hal_ll_adc_hw_specifics_map_t{base: ADC2_BASE_ADDR, module_index: hal_ll_adc_module_num(ADC_MODULE_2), pin: HAL_LL_PIN_NC, vref_input: HAL_LL_ADC_VREF_DEFAULT, vref_value: 0.0, resolution: HAL_LL_ADC_12BIT_RES, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC},
    hal_ll_adc_hw_specifics_map_t{base: ADC3_BASE_ADDR, module_index: hal_ll_adc_module_num(ADC_MODULE_3), pin: HAL_LL_PIN_NC, vref_input: HAL_LL_ADC_VREF_DEFAULT, vref_value: 0.0, resolution: HAL_LL_ADC_12BIT_RES, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC},
    hal_ll_adc_hw_specifics_map_t{base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_MODULE_ERROR as u8, pin: HAL_LL_PIN_NC, vref_input: HAL_LL_ADC_VREF_DEFAULT, vref_value: 0.0, resolution: HAL_LL_ADC_12BIT_RES, channel: hal_ll_adc_channel_t::HAL_LL_ADC_CHANNEL_NC}
];

static mut hal_ll_module_error : usize = 0;


////////////// PUBLIC function //////////////////

pub fn hal_ll_adc_register_handle(pin: hal_ll_pin_name_t, vref_input: hal_ll_adc_voltage_reference_t, resolution : hal_ll_adc_resolution_t, hal_module_id: &mut u8) -> Result<hal_ll_adc_handle_register_t> {
    let pin_check_result: u8;
    let mut index: hal_ll_adc_pin_id = hal_ll_adc_pin_id{pin_an: [HAL_LL_PIN_NC; ADC_MODULE_COUNT as usize]};

    pin_check_result = hal_ll_adc_check_pins(pin, &mut index);
    if pin_check_result == HAL_LL_PIN_NC {
        return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_WRONG_PIN);
    }

    unsafe{
        match resolution {
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_6_BIT => 
                hal_ll_adc_hw_specifics_map[pin_check_result as usize].resolution = HAL_LL_ADC_6BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_8_BIT => 
                hal_ll_adc_hw_specifics_map[pin_check_result as usize].resolution = HAL_LL_ADC_8BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_10_BIT => 
                hal_ll_adc_hw_specifics_map[pin_check_result as usize].resolution = HAL_LL_ADC_10BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_12_BIT => 
                hal_ll_adc_hw_specifics_map[pin_check_result as usize].resolution = HAL_LL_ADC_12BIT_RES,
            _ => return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_UNSUPPORTED_RESOLUTION)
        }
    }

    match vref_input {
        hal_ll_adc_voltage_reference_t::HAL_LL_ADC_VREF_EXTERNAL => (),
        _ => return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_UNSUPPORTED_VREF)
    }

    unsafe{
        if hal_ll_adc_hw_specifics_map[pin_check_result as usize].pin != pin {
            hal_ll_adc_map_pin(pin_check_result, &mut index);
            hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        }

    *hal_module_id = pin_check_result;

    hal_ll_module_state[pin_check_result as usize].hal_ll_adc_handle = hal_ll_adc_hw_specifics_map[pin_check_result as usize].base;
    
    Ok(hal_ll_module_state[pin_check_result as usize])
    }
}

//not thread safe
pub fn hal_ll_module_configure_adc (handle: &mut hal_ll_adc_handle_register_t) {
    
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*handle);
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let pin_check_result : u8 = (*hal_ll_adc_hw_specifics_map_local).module_index;

    hal_ll_adc_init(hal_ll_adc_hw_specifics_map_local);
    unsafe{
        hal_ll_module_state[pin_check_result as usize].hal_ll_adc_handle = hal_ll_adc_hw_specifics_map[pin_check_result as usize].base;
        hal_ll_module_state[pin_check_result as usize].init_ll_state = true;
    }
    hal_handle.init_ll_state = true;
}

pub fn hal_ll_adc_set_resolution(handle: &mut hal_ll_adc_handle_register_t, resolution : hal_ll_adc_resolution_t) -> Result<()> {
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result : u8 = (*hal_ll_adc_hw_specifics_map_local).module_index;
    hal_handle.init_ll_state = false;

    unsafe{
        hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        match resolution {
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_6_BIT => 
                hal_ll_adc_hw_specifics_map_local.resolution = HAL_LL_ADC_6BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_8_BIT => 
                hal_ll_adc_hw_specifics_map_local.resolution = HAL_LL_ADC_8BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_10_BIT => 
                hal_ll_adc_hw_specifics_map_local.resolution = HAL_LL_ADC_10BIT_RES,
            hal_ll_adc_resolution_t::HAL_LL_ADC_RESOLUTION_12_BIT => 
                hal_ll_adc_hw_specifics_map_local.resolution = HAL_LL_ADC_12BIT_RES,
            _ => return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_UNSUPPORTED_RESOLUTION)
        }

        hal_ll_adc_init(hal_ll_adc_hw_specifics_map_local);

        hal_handle.init_ll_state = true;
        hal_ll_module_state[pin_check_result as usize].init_ll_state = true;
        Ok(())
    }
}

pub fn hal_ll_adc_set_vref_input(handle: &mut hal_ll_adc_handle_register_t, vref_input: hal_ll_adc_voltage_reference_t) -> Result<()> {
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result : u8 = (*hal_ll_adc_hw_specifics_map_local).module_index;
    hal_handle.init_ll_state = false;

    unsafe{
        hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        
        match vref_input {
            hal_ll_adc_voltage_reference_t::HAL_LL_ADC_VREF_EXTERNAL => (),
            _ => return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_UNSUPPORTED_VREF)
        }

        hal_ll_adc_init(hal_ll_adc_hw_specifics_map_local);

        hal_handle.init_ll_state = true;
        hal_ll_module_state[pin_check_result as usize].init_ll_state = true;
        Ok(())
    }
}

pub fn hal_ll_adc_set_vref_value(handle: &mut hal_ll_adc_handle_register_t, vref_value: f32)
{
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    
    hal_ll_adc_hw_specifics_map_local.vref_value = vref_value;
}

pub fn hal_ll_adc_read(handle: &mut hal_ll_adc_handle_register_t) -> Result<(u16)> {
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    if hal_ll_adc_hw_specifics_map_local.base == HAL_LL_MODULE_ERROR {
        return Err(HAL_LL_ADC_ERROR::HAL_LL_ADC_UNITIALISED_MODULE);
    }

    let adc_ptr : *mut hal_ll_adc_base_handle_t = hal_ll_adc_hw_specifics_map_local.base as *mut hal_ll_adc_base_handle_t;

    unsafe{
        // (*adc_ptr).sqr3 &= !HAL_LL_ADC_CONVERSION_ONE;
        // (*adc_ptr).sqr3 |= hal_ll_adc_hw_specifics_map_local.channel as u32 & HAL_LL_ADC_CONVERSION_ONE;

        set_reg_bit( &(*adc_ptr).cr2 as *const u32 as u32, HAL_LL_ADC_SWSTART_BIT );

        delay_1us();

        while check_reg_bit( &(*adc_ptr).sr as *const u32 as u32, HAL_LL_ADC_SR_EOC_BIT) == 0 {}


        Ok(((*adc_ptr).dr & 0xFFFF) as u16)

    }
}

pub fn hal_ll_adc_close(handle: &mut hal_ll_adc_handle_register_t) {
    let hal_handle : &mut hal_ll_adc_handle_register_t = handle;
    let hal_ll_adc_hw_specifics_map_local: &mut hal_ll_adc_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result : u8 = (*hal_ll_adc_hw_specifics_map_local).module_index;

    if hal_ll_adc_hw_specifics_map_local.base != HAL_LL_MODULE_ERROR  {

        hal_ll_adc_hw_specifics_map_local.vref_input = HAL_LL_ADC_VREF_DEFAULT;
        hal_ll_adc_hw_specifics_map_local.vref_value = 0.0;
        hal_ll_adc_hw_specifics_map_local.resolution = HAL_LL_ADC_12BIT_RES;
        hal_ll_adc_hw_specifics_map_local.pin = HAL_LL_PIN_NC;

        unsafe{
            hal_ll_module_state[pin_check_result as usize] = hal_ll_adc_handle_register_t::default();
        
            *handle = hal_ll_module_state[pin_check_result as usize];
        }
    }

}

///////////// PRIVATE function /////////////////
fn hal_ll_adc_check_pins(pin: hal_ll_pin_name_t, index: &mut hal_ll_adc_pin_id) -> u8
{
    let adc_map_size: u8 = _adc_map.len() as u8 ;
    let mut index_counter: u8 = 0;
    let mut hal_ll_module_id: u8 = 0;

    if HAL_LL_PIN_NC == pin {
        return HAL_LL_PIN_NC;
    }

    //check if pin is already used by another adc
    for module_index in 0x00 .. ADC_MODULE_COUNT {
        if  pin == unsafe{hal_ll_adc_hw_specifics_map[module_index as usize].pin}  {               
            return HAL_LL_PIN_NC;
        }
    }

    for  pin_index in 0x00 .. adc_map_size
    {
        if pin == _adc_map[pin_index as usize].pin
        {
            // Get module number
            hal_ll_module_id = _adc_map[pin_index as usize].module_index;
            // Map pin name
            index.pin_an[hal_ll_module_id as usize] = pin_index;

            // Check if module is taken
            if  hal_ll_adc_handle_register_t::default().hal_ll_adc_handle == unsafe{hal_ll_module_state[hal_ll_module_id as usize].hal_ll_adc_handle}  {               
                return hal_ll_module_id;
            } else {
                index_counter = index_counter + 1;
                if ADC_MODULE_COUNT == index_counter {
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

fn hal_ll_adc_map_pin(module_index: u8, index: &mut hal_ll_adc_pin_id) {
    unsafe{
        hal_ll_adc_hw_specifics_map[module_index as usize].pin = _adc_map[ index.pin_an[module_index as usize] as usize].pin;
        hal_ll_adc_hw_specifics_map[module_index as usize].channel = _adc_map[ index.pin_an[module_index as usize] as usize ].channel;
    }
}

fn hal_ll_get_specifics<'a>( handle: hal_ll_adc_handle_register_t ) -> &'a mut hal_ll_adc_hw_specifics_map_t{
    unsafe{
        let mut hal_ll_module_count : usize = ptr::read(ptr::addr_of!(hal_ll_module_state)).len();
        hal_ll_module_error = hal_ll_module_count;

    while hal_ll_module_count > 0 {
        hal_ll_module_count -= 1;

        let base = handle.hal_ll_adc_handle;

        if base == hal_ll_adc_hw_specifics_map[hal_ll_module_count].base {
            return &mut hal_ll_adc_hw_specifics_map[hal_ll_module_count];
        }
    }
    return &mut hal_ll_adc_hw_specifics_map[hal_ll_module_error]
    }
}

fn adc1_enable_clock(){
    set_reg_bit( RCC_APB2ENR, HAL_LL_ADC1_ENABLE_CLOCK);
}

fn adc2_enable_clock(){
    set_reg_bit(RCC_APB2ENR, HAL_LL_ADC2_ENABLE_CLOCK);
}

fn adc3_enable_clock(){
    set_reg_bit(RCC_APB2ENR, HAL_LL_ADC3_ENABLE_CLOCK);
}

fn _hal_ll_adc_enable_clock(base :  u8) {
    if base == hal_ll_adc_module_num(ADC_MODULE_1) 
    {adc1_enable_clock();}

    if base == hal_ll_adc_module_num(ADC_MODULE_2) 
    {adc2_enable_clock();}

    if base == hal_ll_adc_module_num(ADC_MODULE_3) 
    {adc3_enable_clock();}    
}

fn _hal_ll_adc_hw_init(base : u32, resolution : u32,  channel : u16) {

    let adc_ptr : *mut hal_ll_adc_base_handle_t = base as *mut hal_ll_adc_base_handle_t;

    let mut rcc_clocks : RCC_ClocksTypeDef = RCC_ClocksTypeDef{
        SYSCLK_Frequency    : 0,
        HCLK_Frequency      : 0,
        PCLK1_Frequency     : 0,
        PCLK2_Frequency     : 0
        };
    let reg : *mut u32 = (ADC1_BASE_ADDR + HAL_LL_ADC_CCR_OFFSET) as *mut u32;

    rcc_get_clocks_frequency( &mut rcc_clocks );
    unsafe{
        *reg &= !HAL_LL_ADC_PRESCALER_MASK;
        if rcc_clocks.PCLK2_Frequency > HAL_LL_ADC_180MHZ 
        {
            *reg |= HAL_LL_ADC_PRESCALER_8;
        }
        else if  rcc_clocks.PCLK2_Frequency > HAL_LL_ADC_120MHZ 
        {
            *reg |= HAL_LL_ADC_PRESCALER_6;
        }
        else if  rcc_clocks.PCLK2_Frequency > HAL_LL_ADC_60MHZ 
        {
            *reg |= HAL_LL_ADC_PRESCALER_4;
        }
        else
        {
            *reg |= HAL_LL_ADC_PRESCALER_2;
        }

        (*adc_ptr).cr1 &= 0xFFF0FEFF;
        (*adc_ptr).cr2 &= !(HAL_LL_ADC_CONT | HAL_LL_ADC_ALIGN | HAL_LL_ADC_JEXTSEL);

        (*adc_ptr).cr1 &= HAL_LL_RESOLUTION_MASK;
        (*adc_ptr).cr1 |= resolution;

         // Scan mode disabled.
        clear_reg_bit(  &(*adc_ptr).cr1 as *const u32 as u32, HAL_LL_ADC_SCAN_BIT );

        // Single conversion mode.
        clear_reg_bit(  &(*adc_ptr).cr2 as *const u32 as u32, HAL_LL_ADC_CONT_BIT );

        // Data alighment = right.
        clear_reg_bit(  &(*adc_ptr).cr2 as *const u32 as u32, HAL_LL_ADC_ALIGN_BIT );

        // Regular channel sequence length 0000: 1 conversion.
        (*adc_ptr).sqr1 &= HAL_LL_ADC_SEQUENCE_LENGTH_MASK;

        // Sets the channel
        (*adc_ptr).sqr3 &= !HAL_LL_ADC_CONVERSION_ONE;
        (*adc_ptr).sqr3 |= channel as u32 & HAL_LL_ADC_CONVERSION_ONE;

        // Enable ADC.
        set_reg_bit( &(*adc_ptr).cr2 as *const u32 as u32, HAL_LL_ADC_ADON_BIT );

    }
}

pub fn hal_ll_adc_init(map: &mut hal_ll_adc_hw_specifics_map_t) {

    hal_ll_gpio_analog_input( hal_ll_gpio_port_base( hal_ll_gpio_port_index( (*map).pin ) ),
                                                     hal_ll_gpio_pin_mask( (*map).pin ) );

    _hal_ll_adc_enable_clock( (*map).module_index );

    _hal_ll_adc_hw_init((*map).base, (*map).resolution, (*map).channel as u16);

}