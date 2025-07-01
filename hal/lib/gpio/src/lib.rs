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

use hal_ll::gpio::*;
use hal_target::*;
use hal_target::pin_names::*;

#[derive(PartialEq)]
pub enum hal_gpio_direction_t
{
    HAL_GPIO_DIGITAL_INPUT = 0,
    HAL_GPIO_DIGITAL_OUTPUT = 1
}

pub type hal_gpio_base_t = handle_t;
pub type hal_gpio_mask_t = hal_ll_gpio_mask_t;

pub struct hal_gpio_t
{
    pub base: hal_gpio_base_t,
    pub mask: hal_gpio_mask_t
}

impl Default for hal_gpio_t {
    fn default() -> Self {
        Self { base: 0, mask: 0}
    }
}

impl From<hal_gpio_direction_t> for hal_ll_gpio_direction_t {
    fn from(direction: hal_gpio_direction_t) -> Self {
        match direction {
            hal_gpio_direction_t::HAL_GPIO_DIGITAL_INPUT => hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT,
            _ => hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT,
        }
    }
}

pub type hal_gpio_pin_t = hal_gpio_t;
pub type hal_gpio_port_t = hal_gpio_t;

pub fn hal_gpio_configure_pin( pin: *mut hal_gpio_pin_t, name: hal_pin_name_t,
        direction: hal_gpio_direction_t)
{
    hal_ll_gpio_configure_pin( pin as *mut hal_ll_gpio_pin_t, name, hal_ll_gpio_direction_t::from(direction) );
}

pub fn hal_gpio_read_pin_input(pin: *mut hal_gpio_pin_t) -> u8
{
    hal_ll_gpio_read_pin_input(pin as *mut hal_ll_gpio_pin_t)
}

pub fn hal_gpio_read_pin_output(pin: *mut hal_gpio_pin_t) -> u8
{
    hal_ll_gpio_read_pin_output(pin as *mut hal_ll_gpio_pin_t)
}

pub fn hal_gpio_write_pin_output(pin: *mut hal_gpio_pin_t, value: u8)
{
    hal_ll_gpio_write_pin_output(pin as *mut hal_ll_gpio_pin_t, value);
}

pub fn hal_gpio_toggle_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_toggle_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_set_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_set_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_clear_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_clear_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_configure_port( port: *mut hal_gpio_port_t, name: hal_port_name_t,
    mask: hal_gpio_mask_t, direction: hal_gpio_direction_t)
{
    hal_ll_gpio_configure_port( port as *mut hal_ll_gpio_port_t, name, mask, hal_ll_gpio_direction_t::from(direction) );
}

pub fn hal_gpio_read_port_input(port: *mut hal_gpio_port_t) -> hal_port_size_t
{
    hal_ll_gpio_read_port_input(port as *mut hal_ll_gpio_port_t)
}

pub fn hal_gpio_read_port_output(port: *mut hal_gpio_port_t) -> hal_port_size_t
{
    hal_ll_gpio_read_port_output(port as *mut hal_ll_gpio_port_t)
}

pub fn hal_gpio_write_port_output(port: *mut hal_gpio_port_t, value: hal_port_size_t)
{
    hal_ll_gpio_write_port_output(port as *mut hal_ll_gpio_port_t, value as hal_ll_port_size_t);
}