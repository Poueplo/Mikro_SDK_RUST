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
#![no_main]
#![allow(unused)]
#![allow(non_upper_case_globals)]

use core::default;

use cortex_m_rt::entry; // The runtime

#[allow(unused_imports)]
use panic_halt;

use drv_port::*;
use drv_digital_in::*;
use drv_digital_out::*;
use drv_name::*;
use delays::*;

const port_out: port_name_t = GPIO_PORT_D;
const pin_in_1: pin_name_t = GPIO_B0;
const pin_in_2: pin_name_t = GPIO_B1;
const pin_in_3: pin_name_t = GPIO_B2;
const pin_in_4: pin_name_t = GPIO_B4;
const pin_out_1: pin_name_t = GPIO_C0;
const pin_out_2: pin_name_t = GPIO_C2;
const pin_out_3: pin_name_t = GPIO_C3;

#[entry]
fn main() -> ! {
    let mut output1: port_t = port_t::default();

    let mut output2: digital_out_t = digital_out_t::default();
    let mut output3: digital_out_t = digital_out_t::default();
    let mut output4: digital_out_t = digital_out_t::default();

    let mut input1: digital_in_t = digital_in_t::default();
    let mut input2: digital_in_t = digital_in_t::default();
    let mut input3: digital_in_t = digital_in_t::default();
    let mut input4: digital_in_t = digital_in_t::default();

    let mut value0 : port_size_t;
    let mut value1 : u8 = 0;
    let mut value2 : u8 = 0;
    let mut value3 : u8 = 0;
    let mut value4 : u8 = 0;

    port_init(&mut output1 , port_out, 0x5555, gpio_direction_t::GPIO_DIGITAL_OUTPUT);
    
    digital_in_init(&mut input1, pin_in_1);
    digital_in_init(&mut input2, pin_in_2);
    digital_in_init(&mut input3, pin_in_3);
    digital_in_init(&mut input4, pin_in_4);
    
    digital_out_init(&mut output2, pin_out_1);
    digital_out_init(&mut output3, pin_out_2);
    digital_out_init(&mut output4, pin_out_3);

    port_write(&mut output1, 0xFFFF);

    value0 = port_read_output(&mut output1).ok().unwrap();
    
    loop {
        value1 = digital_in_read(&mut input1).ok().unwrap();
        value2 = digital_in_read(&mut input2).ok().unwrap();
        value3 = digital_in_read(&mut input3).ok().unwrap();
        value4 = digital_in_read(&mut input4).ok().unwrap();

        if value1 == 1 {
            port_write(&mut output1, 0xFFFF);
        } else {
            port_write(&mut output1, 0x0000);
        }

        if value2 == 1 {
            digital_out_high(&mut output2);
        } else {
            digital_out_low(&mut output2);
        }

        if value3 == 1 {
            digital_out_toggle(&mut output3);
            Delay_10ms();
        }

        if value4 == 1 {
            digital_out_write(&mut output4, 0xFF);
        } else {
            digital_out_write(&mut output4, 0x00);
        }

    }
}
