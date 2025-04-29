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

use core::prelude::v1;

use cortex_m_rt::entry;
// The runtime
use panic_halt;


use drv_pwm::*;
use drv_name::*;
use system::*;

const pin_tim_1: pin_name_t = GPIO_E9;  //channel 1
const pin_tim_2: pin_name_t = GPIO_E11; //channel 2
const pin_tim_3: pin_name_t = GPIO_E13; //channel 3
const pin_tim_4: pin_name_t = GPIO_E14; //channel 4
const pin_tim_5: pin_name_t = GPIO_E8;  //channel 1N
const pin_tim_6: pin_name_t = GPIO_E10; //channel 2N
const pin_tim_7: pin_name_t = GPIO_E12; //channel 3N

const pin_tim_8: pin_name_t = GPIO_C6;  //channel 1
const pin_tim_9: pin_name_t = GPIO_C7; //channel 2
const pin_tim_10: pin_name_t = GPIO_C8; //channel 3

#[entry]
fn main() -> ! {

    system_init();

    let mut pwm_1: pwm_t = pwm_t::default();
    let mut pwm_2: pwm_t = pwm_t::default();
    let mut pwm_3: pwm_t = pwm_t::default();
    let mut pwm_4: pwm_t = pwm_t::default();
    let mut pwm_5: pwm_t = pwm_t::default();
    let mut pwm_6: pwm_t = pwm_t::default();
    let mut pwm_7: pwm_t = pwm_t::default();

    
    let mut pwm_8: pwm_t = pwm_t::default();
    let mut pwm_9: pwm_t = pwm_t::default();
    let mut pwm_10: pwm_t = pwm_t::default();
    

    let mut pwm_config_1: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_2: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_3: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_4: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_5: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_6: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_7: pwm_config_t = pwm_config_t::default();

    
    let mut pwm_config_8: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_9: pwm_config_t = pwm_config_t::default();
    let mut pwm_config_10: pwm_config_t = pwm_config_t::default();

    pwm_config_1.pin = pin_tim_1;
    pwm_config_2.pin = pin_tim_2;
    pwm_config_3.pin = pin_tim_3;
    pwm_config_4.pin = pin_tim_4;
    pwm_config_5.pin = pin_tim_5;
    pwm_config_6.pin = pin_tim_6;
    pwm_config_7.pin = pin_tim_7;

    pwm_config_8.pin = pin_tim_8;
    pwm_config_9.pin = pin_tim_9;
    pwm_config_10.pin = pin_tim_10;

    pwm_open(&mut pwm_1, pwm_config_1);
    pwm_open(&mut pwm_2, pwm_config_2);
    pwm_open(&mut pwm_3, pwm_config_3);
    pwm_open(&mut pwm_4, pwm_config_4);
    pwm_open(&mut pwm_5, pwm_config_5);
    pwm_open(&mut pwm_6, pwm_config_6);
    pwm_open(&mut pwm_7, pwm_config_7);

    pwm_open(&mut pwm_8, pwm_config_8);
    pwm_open(&mut pwm_9, pwm_config_9);
    pwm_open(&mut pwm_10,pwm_config_10);

    pwm_set_freq(&mut pwm_4, 0x55D);

    let mut duty1: f32 = 0.1;
    let mut duty2: f32 = 0.1;
    let mut duty3: f32 = 0.1;
    let mut duty4: f32 = 0.1;

    pwm_set_duty(&mut pwm_5, duty1);
    pwm_set_duty(&mut pwm_6, duty2);
    pwm_set_duty(&mut pwm_7, duty3);
    pwm_set_duty(&mut pwm_4, duty4);

    
    pwm_set_duty(&mut pwm_8, 0.5);
    pwm_set_duty(&mut pwm_9, 0.4);
    pwm_set_duty(&mut pwm_10, 0.3);

    pwm_start(&mut pwm_1);
    pwm_start(&mut pwm_2);
    pwm_start(&mut pwm_3);
    pwm_start(&mut pwm_4);
    pwm_start(&mut pwm_5);
    pwm_start(&mut pwm_6);
    pwm_start(&mut pwm_7);

    
    pwm_start(&mut pwm_8);
    pwm_start(&mut pwm_9);
    pwm_start(&mut pwm_10);

    Delay_ms(1000);
    pwm_stop(&mut pwm_9);
    Delay_ms(1000);
    pwm_close(&mut pwm_10);
    pwm_close(&mut pwm_9);

    loop {
        pwm_set_duty(&mut pwm_5, duty1);
        pwm_set_duty(&mut pwm_6, duty2);
        pwm_set_duty(&mut pwm_7, duty3);
        pwm_set_duty(&mut pwm_4, duty4);
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