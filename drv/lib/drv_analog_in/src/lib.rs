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

use hal_adc::*;
use drv_name::*;

pub use hal_adc::hal_adc_config_t as analog_in_config_t;
pub use hal_adc::hal_adc_t as analog_in_t;
pub use hal_adc::hal_adc_resolution_t as analog_in_resolution_t;
pub use hal_adc::HAL_ADC_ERROR as  DRV_ADC_ERROR;
pub use hal_adc::hal_adc_vref_t as analog_in_vref_t;
pub use hal_adc::{ADC_RESOLUTION_DEFAULT, ADC_VREF_DEFAULT};

type Result<T> = core::result::Result<T, DRV_ADC_ERROR>;

static mut previous_initialized: analog_in_t = analog_in_t{ 
        handle: hal_adc_handle_register_t{ adc_handle: 0, init_ll_state: false },
        config: analog_in_config_t{ pin: HAL_PIN_NC, 
                                    resolution: ADC_RESOLUTION_DEFAULT, 
                                    vref_input: hal_adc_vref_t::ADC_VREF_EXTERNAL, 
                                    vref_value: 0.0}, 
        };

fn _acquire( obj: &mut analog_in_t, obj_open_state: bool) -> Result<()> {
    if obj_open_state && (unsafe{previous_initialized == *obj}) {
        return Err(DRV_ADC_ERROR::ACQUIRE_FAIL);
    }

    if unsafe{previous_initialized != *obj} {
        match hal_adc_open( obj, obj_open_state ) {
            Ok(_) => {
                unsafe{previous_initialized = *obj;}
                return Ok(())
            },
            Err(e) => return Err(e),
        }
    }

    Ok(())
}


pub fn analog_in_open( obj: &mut analog_in_t, config: analog_in_config_t ) -> Result<()> {
    obj.config = config;

    match _acquire( obj, true ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn analog_in_set_resolution( obj: &mut analog_in_t, resolution: analog_in_resolution_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.resolution = resolution;
    
    match hal_adc_set_resolution( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }

}

pub fn analog_in_set_vref_input( obj: &mut analog_in_t, vref_input: analog_in_vref_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.vref_input = vref_input;
    
    match hal_adc_set_vref_input( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn analog_in_set_vref_value( obj: &mut analog_in_t, vref_value: f32 ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.vref_value = vref_value;
    
    match hal_adc_set_vref_value( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn analog_in_read( obj: &mut analog_in_t) -> Result<u16> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    match hal_adc_read( obj) {
        Ok(r) => Ok(r),
        Err(e) => return Err(e),
    }
}

pub fn analog_in_read_voltage( obj: &mut analog_in_t) -> Result<f32> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    match hal_adc_read_voltage( obj) {
        Ok(r) => Ok(r),
        Err(e) => return Err(e),
    }
}


pub fn analog_in_close(obj: &mut analog_in_t) -> Result<()> {
    match hal_adc_close(obj) {
        Ok(_) => {
            unsafe{previous_initialized = analog_in_t::default();}
            Ok(())
        },
        Err(e) => Err(e),
    }
}