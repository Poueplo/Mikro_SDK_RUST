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
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use hal_ll_adc::*;
use hal_target::*;
use core::fmt;

pub use hal_ll_adc::hal_ll_adc_resolution_t as hal_adc_resolution_t;
pub use hal_ll_adc::hal_ll_adc_voltage_reference_t as hal_adc_vref_t;
pub use hal_ll_adc::hal_ll_adc_handle_register_t as hal_adc_handle_register_t;
pub use hal_ll_adc::{ADC_RESOLUTION_DEFAULT, ADC_VREF_DEFAULT};

#[derive(Debug)]
pub enum HAL_ADC_ERROR {
    ADC_ERROR,
    ACQUIRE_FAIL
}

impl fmt::Display for HAL_ADC_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ADC_ERROR => write!(f, "ADC_ERROR occurred"),
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),
        }
    }
}

type Result<T> = core::result::Result<T, HAL_ADC_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_adc_config_t {
    pub pin: hal_pin_name_t,
    pub resolution: hal_adc_resolution_t,
    pub vref_input: hal_adc_vref_t,
    pub vref_value: f32
}

impl Default for hal_adc_config_t {
    fn default() -> Self {
        Self { pin: HAL_PIN_NC, resolution: ADC_RESOLUTION_DEFAULT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_adc_t {
    pub handle: hal_adc_handle_register_t,
    pub config: hal_adc_config_t
}

impl Default for hal_adc_t {
    fn default() -> Self {
        Self {
            handle: hal_adc_handle_register_t{ adc_handle: 0, init_ll_state: false },
            config: hal_adc_config_t{ pin: HAL_PIN_NC, resolution: ADC_RESOLUTION_DEFAULT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 },
        }
    }
}

static mut hal_module_state: [hal_adc_handle_register_t; ADC_MODULE_COUNT as usize]  = [ 
    hal_adc_handle_register_t{
        adc_handle: 0xFFFF_FFFF,
        init_ll_state: false,
        };
    ADC_MODULE_COUNT as usize];

fn hal_is_handle_register<'a>(hal_module_handle: &mut hal_adc_t) -> Option<&'a mut hal_adc_handle_register_t> {
    let mut hal_module_state_count: u8 = ADC_MODULE_COUNT;

    while hal_module_state_count > 0 
    {   
        unsafe{
            if hal_module_handle.handle.adc_handle == hal_module_state[ (hal_module_state_count - 1) as usize ].adc_handle
            {
                return Some(&mut hal_module_state[ (hal_module_state_count - 1) as usize]);
            }
        }
        hal_module_state_count = hal_module_state_count - 1;
    }
     return None;
}

pub fn hal_adc_open(handle: &mut hal_adc_t, hal_obj_open_state: bool) -> Result<()> {
    let mut hal_module_id: u8 = 0;
    let hal_obj: &mut hal_adc_t = handle;

    match hal_is_handle_register( hal_obj ) {
        Some(_) => {
            if hal_obj_open_state
            {
                return Err(HAL_ADC_ERROR::ACQUIRE_FAIL) 
            } else {
                return Ok(())
            }
        },
        None => {
            if !hal_obj_open_state {
                return Err(HAL_ADC_ERROR::ACQUIRE_FAIL) 
            }
        },
    }
    unsafe{
        match hal_ll_adc_register_handle( hal_obj.config.pin, hal_obj.config.vref_input,
                                                hal_obj.config.resolution, &mut hal_module_id )
        {

            Ok(h) => {
                hal_module_state[ hal_module_id as usize].adc_handle = h.adc_handle;
                (*hal_obj).handle = h;
                Ok(())
            },
            Err(_) => {
                hal_obj.handle = hal_adc_t::default().handle;
                return Err(HAL_ADC_ERROR::ACQUIRE_FAIL) 
            },
        }

    }

}

pub fn hal_adc_set_resolution(handle: &mut hal_adc_t, config: hal_adc_config_t) -> Result<()> {
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    match hal_ll_adc_set_resolution(hal_handle , config.resolution) {
        Ok(_) =>{ 
            hal_obj.handle = *hal_handle;
            Ok(())
        },
        Err(_) => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }
}

pub fn hal_adc_set_vref_input(handle: &mut hal_adc_t, config: hal_adc_config_t) -> Result<()> {
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    match hal_ll_adc_set_vref_input(hal_handle , config.vref_input) {
        Ok(_) =>{ 
            hal_obj.handle = *hal_handle;
            Ok(())
        },
        Err(_) => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }
}

pub fn hal_adc_set_vref_value(handle: &mut hal_adc_t, config: hal_adc_config_t) -> Result<()> {
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    hal_ll_adc_set_vref_value(hal_handle , config.vref_value);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_adc_read(handle: &mut hal_adc_t) -> Result<u16> {
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t; 
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_adc(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_adc_read(hal_handle) {
        Ok(r) => Ok(r),
        Err(_) => Err(HAL_ADC_ERROR::ADC_ERROR),
    }
}

pub fn hal_adc_read_voltage(handle: &mut hal_adc_t) -> Result<f32> {
    let read_value: u16;
    let local_resolution: u16;
    
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t; 
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_adc(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_adc_read(hal_handle) {
        Ok(r) => read_value = r,
        Err(_) => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    match hal_obj.config.resolution {
        hal_ll_adc_resolution_t::ADC_RESOLUTION_6_BIT => local_resolution = HAL_LL_ADC_RESOLUTION_MASK::ADC_6BIT_MASK_VAL as u16,
        hal_ll_adc_resolution_t::ADC_RESOLUTION_8_BIT => local_resolution = HAL_LL_ADC_RESOLUTION_MASK::ADC_8BIT_MASK_VAL as u16,
        hal_ll_adc_resolution_t::ADC_RESOLUTION_10_BIT => local_resolution = HAL_LL_ADC_RESOLUTION_MASK::ADC_10BIT_MASK_VAL as u16,
        hal_ll_adc_resolution_t::ADC_RESOLUTION_12_BIT => local_resolution = HAL_LL_ADC_RESOLUTION_MASK::ADC_12BIT_MASK_VAL as u16,
        _ => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    return Ok((read_value as f32 *  hal_obj.config.vref_value ) / local_resolution as f32)
}

pub fn hal_adc_close( handle: &mut hal_adc_t) -> Result<()> {
    let hal_obj: &mut hal_adc_t = handle;
    let hal_handle: &mut hal_adc_handle_register_t; 
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_ADC_ERROR::ADC_ERROR),
    }

    if (hal_handle.adc_handle != 0) & (hal_handle.adc_handle != 0xFFFF_FFFF)
    {
        hal_ll_adc_close( hal_handle );

        hal_obj.config = hal_adc_config_t::default();
        *hal_obj = hal_adc_t::default();
        hal_handle.adc_handle = 0xFFFF_FFFF;

        return Ok(())
    }

    Err(HAL_ADC_ERROR::ADC_ERROR)
}