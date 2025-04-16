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


use drv_analog_in::*;
use drv_digital_out::*;
use drv_port::*;
use drv_name::*;
use system::*;

const port_out: port_name_t = GPIO_PORT_E;
const pin_an: pin_name_t = GPIO_A3;


#[entry]
fn main() -> ! {

    system_init();

    let mut read_value: u16 = 0;
    let mut read_voltage: f32 = 0.0;
    let mut adc1: analog_in_t = analog_in_t::default();
    let config_1: analog_in_config_t =  analog_in_config_t{ pin: pin_an, resolution: analog_in_resolution_t::ADC_RESOLUTION_12_BIT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 };

    let drv_config: analog_in_t = analog_in_t::default();

    adc1.config = analog_in_config_t{ pin: pin_an, resolution: analog_in_resolution_t::ADC_RESOLUTION_12_BIT, vref_input: ADC_VREF_DEFAULT, vref_value: 0.0 };

    analog_in_open(&mut adc1, config_1);

    analog_in_set_vref_value(&mut adc1, 25.0);


    let mut output1: port_t = port_t::default();

    port_init(&mut output1 , port_out, 0xFFFF, gpio_direction_t::GPIO_DIGITAL_OUTPUT);
    
    
    loop {
        read_value = analog_in_read(&mut adc1).ok().unwrap();
        read_voltage = analog_in_read_voltage(&mut adc1).ok().unwrap();

        port_write(&mut output1, read_value);
        
    }
}
