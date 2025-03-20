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
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use core::{fmt, ptr::null};
use hal_gpio::*;
use drv_name::*;

type Result<T> = core::result::Result<T, DIGITAL_OUT_ERROR>;

#[derive(Debug, Clone)]
pub struct DIGITAL_OUT_ERROR;

impl fmt::Display for DIGITAL_OUT_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DIGITAL_OUT_ERROR occurred")
    }
}

pub struct digital_out_t
{
    pin: hal_gpio_pin_t, /*< Structure containing port address and port name. */
}

impl Default for digital_out_t {
    fn default() -> Self {
        Self { pin: hal_gpio_pin_t::default()}
    }
}

pub fn digital_out_init(out_pin: &mut digital_out_t, name: pin_name_t) -> Result<()>
{
    if name != HAL_PIN_NC {
        hal_gpio_configure_pin(&mut out_pin.pin as *mut _, name, hal_gpio_direction_t::from(gpio_direction_t::GPIO_DIGITAL_OUTPUT));
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_high(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_set_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_low(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_clear_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_toggle(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_toggle_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_write(out_pin: &mut digital_out_t, value: u8) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_write_pin_output(&mut out_pin.pin as *mut _, value);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}