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
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use cortex_m_rt::entry;
// The runtime
use panic_halt;


use hal_ll_tim::*;
use drv_name::*;
use system::*;

const pin_tim_1: pin_name_t = GPIO_E9;  //channel 1
const pin_tim_2: pin_name_t = GPIO_E11; //channel 2
const pin_tim_3: pin_name_t = GPIO_E13; //channel 3
const pin_tim_4: pin_name_t = GPIO_E14; //channel 4
const pin_tim_5: pin_name_t = GPIO_E8;  //channel 1N
const pin_tim_6: pin_name_t = GPIO_E10; //channel 2N
const pin_tim_7: pin_name_t = GPIO_E12; //channel 3N

#[entry]
fn main() -> ! {

    system_init();

    let mut tim: hal_ll_tim_handle_register_t = hal_ll_tim_handle_register_t::default();
    let mut module_index: u8 = 0;
    let mut error: u8 = 0;

    tim = hal_ll_tim_register_handle(pin_tim_1, &mut module_index).ok().unwrap();
    match hal_ll_tim_register_handle(pin_tim_2, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 1,
    }

    match hal_ll_tim_register_handle(pin_tim_3, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 2,
    }

    match hal_ll_tim_register_handle(pin_tim_4, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 3,
    }

    match hal_ll_tim_register_handle(pin_tim_5, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 4,
    }

    match hal_ll_tim_register_handle(pin_tim_6, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 5,
    }

    match hal_ll_tim_register_handle(pin_tim_7, &mut module_index) {
        Ok(register) => tim = register,
        Err(_) => error = 6,
    }

    hal_ll_tim_set_freq(&mut tim, 0x55D);

    let mut duty1: f32 = 0.1;
    let mut duty2: f32 = 0.1;
    let mut duty3: f32 = 0.1;
    let mut duty4: f32 = 0.1;

    hal_ll_tim_set_duty(&mut tim, pin_tim_1, duty1);
    hal_ll_tim_set_duty(&mut tim, pin_tim_2, duty2);
    hal_ll_tim_set_duty(&mut tim, pin_tim_3, duty3);
    hal_ll_tim_set_duty(&mut tim, pin_tim_4, duty4);

    hal_ll_tim_start(&mut tim, pin_tim_1);
    hal_ll_tim_start(&mut tim, pin_tim_2);
    hal_ll_tim_start(&mut tim, pin_tim_3);
    hal_ll_tim_start(&mut tim, pin_tim_4);
    hal_ll_tim_start(&mut tim, pin_tim_5);
    hal_ll_tim_start(&mut tim, pin_tim_6);
    hal_ll_tim_start(&mut tim, pin_tim_7);

    // hal_ll_tim_set_duty(&mut tim, pin_tim_1, 0.5);
    // Delay_ms(100);
    // hal_ll_tim_stop(&mut tim, pin_tim_5);


    // duty1 += 0.1;
    // duty2 += 0.2;
    // duty3 += 0.3;
    // duty4 += 0.4;
    // hal_ll_tim_set_duty(&mut tim, pin_tim_1, duty1);
    // hal_ll_tim_set_duty(&mut tim, pin_tim_2, duty2);
    // hal_ll_tim_set_duty(&mut tim, pin_tim_3, duty3);
    // hal_ll_tim_set_duty(&mut tim, pin_tim_4, duty4);
    // Delay_ms(100);

    // hal_ll_tim_close(&mut tim);

    loop {
        hal_ll_tim_set_duty(&mut tim, pin_tim_1, duty1);
        hal_ll_tim_set_duty(&mut tim, pin_tim_2, duty2);
        hal_ll_tim_set_duty(&mut tim, pin_tim_3, duty3);
        hal_ll_tim_set_duty(&mut tim, pin_tim_4, duty4);
        Delay_ms(100);
        duty1 += 0.1;
        duty2 += 0.2;
        duty3 += 0.3;
        duty4 += 0.4;


        if duty1 > 1.0 {
            duty1 = 0.1;
        }
        if duty2 > 1.0 {
            duty2 = 0.1;
        }
        if duty3 > 1.0 {
            duty3 = 0.1;
        }
        if duty4 > 1.0 {
            duty4 = 0.1;
        }
    }
}