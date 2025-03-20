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

use hal_ll_gpio_port::*;
use hal_ll_gpio_port::pin_names::{hal_ll_port_name_t,hal_ll_port_size_t,hal_ll_gpio_mask_t, hal_ll_pin_name_t};
pub use hal_ll_gpio_port::pin_names;
pub use hal_ll_gpio_port::{hal_ll_gpio_direction_t,hal_ll_gpio_port_t,hal_ll_gpio_pin_t};

pub fn hal_ll_gpio_configure_pin(pin: *mut hal_ll_gpio_pin_t, name: hal_ll_pin_name_t , direction: hal_ll_gpio_direction_t)
{
    unsafe{
        (*pin).base = hal_ll_gpio_port_base(hal_ll_gpio_port_index(name));
        (*pin).mask = hal_ll_gpio_pin_mask(name);

        if direction == hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT
        {
            hal_ll_gpio_digital_input((*pin).base, (*pin).mask);
        }
        else
        {
            hal_ll_gpio_digital_output((*pin).base, (*pin).mask);
        }

    }
}

pub fn hal_ll_gpio_read_pin_input(pin: *mut hal_ll_gpio_pin_t) -> u8
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*pin).base as *mut hal_ll_gpio_base_handle_t;

        if (*base_reg).idr & ((*pin).mask) as u32 != 0 {
            0x01
        } else {
            0x00
        }
    }
}

pub fn hal_ll_gpio_read_pin_output(pin: *mut hal_ll_gpio_pin_t) -> u8
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*pin).base as *mut hal_ll_gpio_base_handle_t;

        if (*base_reg).odr & ((*pin).mask) as u32 != 0 {
            0x01
        } else {
            0x00
        }
    }
}

pub fn hal_ll_gpio_write_pin_output(pin: *mut hal_ll_gpio_pin_t, value: u8)
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*pin).base as *mut hal_ll_gpio_base_handle_t;

        if value != 0 {
            (*base_reg).bsrr = (*pin).mask as u32;
        } else {
            (*base_reg).bsrr = ((*pin).mask as u32) << (RESET_PINS_OFFSET as u32);
        }

    }
}

pub fn hal_ll_gpio_toggle_pin_output(pin: *mut hal_ll_gpio_pin_t)
{
    let value: u8;
    if hal_ll_gpio_read_pin_output(pin) == 0x00 {
        value = 0x01;
    } else {
        value = 0x00;
    }
    hal_ll_gpio_write_pin_output(pin, value);
}

pub fn hal_ll_gpio_set_pin_output(pin: *mut hal_ll_gpio_pin_t)
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*pin).base as *mut hal_ll_gpio_base_handle_t;
        (*base_reg).bsrr = (*pin).mask as u32;
    }
}

pub fn hal_ll_gpio_clear_pin_output(pin: *mut hal_ll_gpio_pin_t)
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*pin).base as *mut hal_ll_gpio_base_handle_t;
        (*base_reg).bsrr = ((*pin).mask as u32) << (RESET_PINS_OFFSET as u32);
    }
}

pub fn hal_ll_gpio_configure_port(port: *mut hal_ll_gpio_port_t, name: hal_ll_port_name_t , mask: hal_ll_gpio_mask_t, direction: hal_ll_gpio_direction_t)
{
    unsafe{
        (*port).base = hal_ll_gpio_port_base(name);
        (*port).mask = mask;

        if direction == hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT
        {
            hal_ll_gpio_digital_input((*port).base, (*port).mask);
        }
        else
        {
            hal_ll_gpio_digital_output((*port).base, (*port).mask);
        }
    }
}

pub fn hal_ll_gpio_read_port_input(port: *mut hal_ll_gpio_port_t) -> hal_ll_port_size_t
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_base_handle_t;

        ((*base_reg).idr & ((*port).mask) as u32)as u16
    }
}

pub fn hal_ll_gpio_read_port_output(port: *mut hal_ll_gpio_port_t) -> hal_ll_port_size_t
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_base_handle_t;

        ((*base_reg).odr & ((*port).mask) as u32)as u16
    }
}

pub fn hal_ll_gpio_write_port_output(port: *mut hal_ll_gpio_port_t, value: hal_ll_port_size_t)
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_base_handle_t;

        (*base_reg).bsrr = (value as u32 & (*port).mask as u32) | (((!value & (*port).mask) as u32) << (RESET_PINS_OFFSET as u32));
    }
}