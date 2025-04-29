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

use hal_ll_tim::*;
use hal_target::*;
use hal_target::pin_names::*;

pub use hal_ll_tim::hal_ll_tim_handle_register_t as hal_pwm_handle_register_t;
pub use hal_ll_tim::HAL_LL_TIM_ERROR as HAL_PWM_ERROR;

type Result<T> = core::result::Result<T, HAL_PWM_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_pwm_config_t {
    pub pin: hal_pin_name_t,
    pub freq_hz: *mut u32,
}

impl Default for hal_pwm_config_t {
    fn default() -> Self {
        Self{ pin: HAL_PIN_NC, freq_hz: 0 as *mut u32 }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_pwm_t {
    pub handle: hal_pwm_handle_register_t,
    pub config: hal_pwm_config_t
}

impl Default for hal_pwm_t {
    fn default() -> Self {
        Self { 
            handle: hal_pwm_handle_register_t{ tim_handle: 0, init_ll_state: false }, 
            config: hal_pwm_config_t{pin: HAL_PIN_NC, freq_hz: 0 as *mut u32 } 
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct hal_pwm_handle {
    pub handle : hal_pwm_handle_register_t,
    pub freq_hz : u32,
}

impl Default for hal_pwm_handle {
    fn default() -> Self {
        Self {
            handle: hal_pwm_handle_register_t{
                tim_handle: 0xFFFF_FFFF,
                init_ll_state: false,
                },
            freq_hz: 1
        }
    }
}


static mut hal_module_state: [hal_pwm_handle; TIM_MODULE_COUNT as usize]  = [ 
        hal_pwm_handle{
            handle: hal_pwm_handle_register_t{
                tim_handle: 0xFFFF_FFFF,
                init_ll_state: false,
                },
            freq_hz: 1
        };
        TIM_MODULE_COUNT as usize];


fn hal_is_handle_register<'a>(hal_module_handle: &mut hal_pwm_t) -> Option<&'a mut hal_pwm_handle_register_t> {
    let mut hal_module_state_count: u8 = TIM_MODULE_COUNT;

    while hal_module_state_count > 0 
    {   
        unsafe{
            if hal_module_handle.handle.tim_handle == hal_module_state[ (hal_module_state_count - 1) as usize ].handle.tim_handle
            {
                return Some(&mut hal_module_state[ (hal_module_state_count - 1) as usize].handle);
            }
        }
        hal_module_state_count = hal_module_state_count - 1;
    }
     return None;
}

pub fn hal_pwm_open(handle: &mut hal_pwm_t, hal_obj_open_state: bool) -> Result<()> {
    let mut hal_module_id: u8 = 0;
    let hal_obj: &mut hal_pwm_t = handle;

    match hal_is_handle_register( hal_obj ) {
        Some(_) => {
            if hal_obj_open_state
            {
                return Err(HAL_PWM_ERROR::ACQUIRE_FAIL) 
            } else {
                return Ok(())
            }
        },
        None => {
            if !hal_obj_open_state {
                return Err(HAL_PWM_ERROR::ACQUIRE_FAIL) 
            }
        },
    }
    unsafe{
        match hal_ll_tim_register_handle(hal_obj.config.pin, &mut hal_module_id )
        {

            Ok(h) => {
                hal_module_state[ hal_module_id as usize].handle.tim_handle = h.tim_handle;
                (*hal_obj).handle = h;
                (*hal_obj).config.freq_hz = &mut hal_module_state[ hal_module_id as usize].freq_hz;
                Ok(())
            },
            Err(_) => {
                hal_obj.handle = hal_pwm_t::default().handle;
                return Err(HAL_PWM_ERROR::ACQUIRE_FAIL) 
            },
        }
    }
}

pub fn hal_pwm_start(handle: &mut hal_pwm_t) -> Result<()> {
    let hal_obj: &mut hal_pwm_t = handle;
    let hal_handle: &mut hal_pwm_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_PWM_ERROR::PWM_ERROR),
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_tim(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_tim_start(hal_handle, hal_obj.config.pin) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_pwm_stop(handle: &mut hal_pwm_t) -> Result<()> {
    let hal_obj: &mut hal_pwm_t = handle;
    let hal_handle: &mut hal_pwm_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_PWM_ERROR::PWM_ERROR),
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_tim(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_tim_stop(hal_handle, hal_obj.config.pin) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_pwm_set_freq(handle: &mut hal_pwm_t, freq_hz: u32) -> Result<()> {
    let hal_obj: &mut hal_pwm_t = handle;
    let hal_handle: &mut hal_pwm_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_PWM_ERROR::PWM_ERROR),
    }

    hal_ll_tim_set_freq(hal_handle , freq_hz);
    hal_obj.handle = *hal_handle;
    unsafe{*hal_obj.config.freq_hz = freq_hz;}
    Ok(())
}

pub fn hal_pwm_set_duty(handle: &mut hal_pwm_t, duty_ratio: f32) -> Result<()> {
    let hal_obj: &mut hal_pwm_t = handle;
    let hal_handle: &mut hal_pwm_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_PWM_ERROR::PWM_ERROR),
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_tim(hal_handle);
        hal_obj.handle = *hal_handle;
    }

    hal_ll_tim_set_duty(hal_handle , hal_obj.config.pin, duty_ratio);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_pwm_close( handle: &mut hal_pwm_t) -> Result<()> {
    let hal_obj: &mut hal_pwm_t = handle;
    let hal_handle: &mut hal_pwm_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_PWM_ERROR::PWM_ERROR),
    }

    if (hal_handle.tim_handle != 0) & (hal_handle.tim_handle != 0xFFFF_FFFF)
    {
        hal_ll_tim_close( hal_handle );

        hal_obj.config = hal_pwm_config_t::default();
        *hal_obj = hal_pwm_t::default();
        hal_handle.tim_handle = 0xFFFF_FFFF;

        return Ok(())
    }

    Err(HAL_PWM_ERROR::PWM_ERROR)
}