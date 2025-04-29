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

use hal_pwm::*;
use drv_name::*;

pub use hal_pwm::hal_pwm_config_t as pwm_config_t;
pub use hal_pwm::hal_pwm_t as pwm_t;
pub use hal_pwm::HAL_PWM_ERROR as  DRV_PWM_ERROR;

type Result<T> = core::result::Result<T, DRV_PWM_ERROR>;

static mut previous_initialized: pwm_t = pwm_t { 
    handle: hal_pwm_handle_register_t{ tim_handle: 0, init_ll_state: false },
    config: pwm_config_t{ pin: HAL_PIN_NC, freq_hz: 0 as *mut u32}, 
    };

fn _acquire( obj: &mut pwm_t, obj_open_state: bool) -> Result<()> {
    if obj_open_state && (unsafe{previous_initialized == *obj}) {
        return Err(DRV_PWM_ERROR::ACQUIRE_FAIL);
    }

    if unsafe{previous_initialized != *obj} {
        match hal_pwm_open( obj, obj_open_state ) {
            Ok(_) => {
                unsafe{previous_initialized = *obj;}
                return Ok(())
            },
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub fn pwm_open( obj: &mut pwm_t, config: pwm_config_t ) -> Result<()> {
    obj.config = config;

    match _acquire( obj, true ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn pwm_start( obj: &mut pwm_t) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_pwm_start( obj) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn pwm_stop( obj: &mut pwm_t) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_pwm_stop( obj) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn pwm_set_duty( obj: &mut pwm_t, duty: f32) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_pwm_set_duty( obj, duty) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn pwm_set_freq( obj: &mut pwm_t, freq_hz: u32) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_pwm_set_freq( obj, freq_hz) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn pwm_close(obj: &mut pwm_t) -> Result<()> {
    match hal_pwm_close(obj) {
        Ok(_) => {
            unsafe{previous_initialized = pwm_t::default();}
            Ok(())
        },
        Err(e) => Err(e),
    }
}