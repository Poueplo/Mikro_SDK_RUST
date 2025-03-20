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

type Result<T> = core::result::Result<T, PORT_ERROR>;

#[derive(Debug, Clone)]
pub struct PORT_ERROR;

impl fmt::Display for PORT_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PORT_ERROR occurred")
    }
}

pub struct port_t
{
    port: hal_gpio_port_t, /*< Structure containing port address and port name. */
    name: port_name_t     /*< Port name. */
}

impl Default for port_t {
    fn default() -> Self {
        Self { port: hal_gpio_port_t::default(), name: 0}
    }
}


pub fn port_init(port: &mut port_t, name: port_name_t, mask: port_size_t, direction: gpio_direction_t) -> Result<()>
{
    if (direction != gpio_direction_t::GPIO_DIGITAL_INPUT) && (direction != gpio_direction_t::GPIO_DIGITAL_OUTPUT) {
        return Err(PORT_ERROR);
    }
    if name != HAL_PORT_NC {
        port.name = name;
        hal_gpio_configure_port( &mut port.port as *mut _, name, mask, hal_gpio_direction_t::from(direction));
        Ok(())
    } else {
        Err(PORT_ERROR)
    }
}

pub fn port_write(port: &mut port_t, value: port_size_t) -> Result<()>
{
    if port.port.base != 0 {
        hal_gpio_write_port_output(&mut port.port as *mut _, value);
        Ok(())
    } else {
        Err(PORT_ERROR)
    }
    
}

pub fn port_read_input(port: &mut port_t) -> Result<port_size_t>
{
    if port.port.base != 0 {
        Ok(hal_gpio_read_port_input(&mut port.port as *mut _))
    } else {
        Err(PORT_ERROR)
    }
}

pub fn port_read_output(port: &mut port_t) -> Result<port_size_t>
{
    if port.port.base != 0 {
        Ok(hal_gpio_read_port_output(&mut port.port as *mut _))
    } else {
        Err(PORT_ERROR)
    }
}