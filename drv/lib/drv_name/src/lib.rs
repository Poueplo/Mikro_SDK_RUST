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

use hal_gpio::hal_gpio_direction_t;
use hal_target::{hal_port_name_t, hal_port_size_t,hal_pin_name_t};
pub use hal_target::{HAL_PORT_NC,HAL_PIN_NC};
pub use hal_target::pin_names::*;

#[derive(PartialEq)]
pub enum gpio_direction_t
{
    GPIO_DIGITAL_INPUT = 0, /*< GPIO Digital input. */
    GPIO_DIGITAL_OUTPUT = 1 /*< GPIO Digital output. */
}

impl From<gpio_direction_t> for hal_gpio_direction_t {
    fn from(direction: gpio_direction_t) -> Self {
        match direction {
            gpio_direction_t::GPIO_DIGITAL_INPUT => hal_gpio_direction_t::HAL_GPIO_DIGITAL_INPUT,
            _ =>  hal_gpio_direction_t::HAL_GPIO_DIGITAL_OUTPUT,
        }
    }
}

pub type pin_name_t = hal_pin_name_t; /*< GPIO pin name. */

pub type port_name_t = hal_port_name_t; /*< GPIO port name.*/

pub type port_size_t = hal_port_size_t; /*< GPIO port size. */