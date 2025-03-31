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
//#![allow(unused)]
#![allow(non_upper_case_globals)]



use cortex_m_rt::entry; // The runtime

use hal_adc::hal_adc_t;
#[allow(unused_imports)]
use panic_halt;

use drv_port::*;
use drv_digital_in::*;
use drv_name::*;
use system::*;
use hal_adc::*;

const port_out: port_name_t = GPIO_PORT_E;
const pin_in_1: pin_name_t = GPIO_B0;

#[entry]
fn main() -> ! {

    system_init();

    let mut read_value: u16 = 0;
    let mut adc1: hal_adc_t = hal_adc_t::default();
    let mut adc2: hal_adc_t = hal_adc_t::default();
    let config_1: hal_adc_config_t =  hal_adc_config_t{ pin: GPIO_A3, resolution: hal_adc_resolution_t::ADC_RESOLUTION_10_BIT, vref_input: ADC_VREF_DEFAULT, vref_value: 3.3 };
    let config_2: hal_adc_config_t =  hal_adc_config_t{ pin: GPIO_A3, resolution: hal_adc_resolution_t::ADC_RESOLUTION_8_BIT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 };

    adc1.config = hal_adc_config_t{ pin: GPIO_A3, resolution: hal_adc_resolution_t::ADC_RESOLUTION_6_BIT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 };
    adc2.config = hal_adc_config_t{ pin: GPIO_C0, resolution: ADC_RESOLUTION_DEFAULT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 };

    hal_adc_open(&mut adc1, true);
    hal_adc_open(&mut adc2, true);
    hal_adc_set_resolution(&mut adc2, config_1);
    hal_adc_close(&mut adc2);

    hal_adc_set_resolution(&mut adc1, config_1);
    hal_adc_set_resolution(&mut adc1, config_2);
    hal_adc_set_vref_value(&mut adc1, config_1);
    hal_adc_set_vref_value(&mut adc1, config_2);


    let mut output1: port_t = port_t::default();

    let mut input1: digital_in_t = digital_in_t::default();

    let mut value1 : u8 = 0;

    port_init(&mut output1 , port_out, 0xFFFF, gpio_direction_t::GPIO_DIGITAL_OUTPUT);
    
    digital_in_init(&mut input1, pin_in_1);
    
    loop {
        value1 = digital_in_read(&mut input1).ok().unwrap();
        read_value = hal_adc_read(&mut adc1).ok().unwrap();


        if value1 == 1 {
            port_write(&mut output1, read_value);
        } else {
            port_write(&mut output1, 0x0000);
        }

    }
}
