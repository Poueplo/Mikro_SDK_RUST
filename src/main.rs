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

// The runtime
use panic_halt;

use drv_name::*;
use system::*;

use drv_one_wire::*;

const pin: pin_name_t = GPIO_F4;

const device_command : &[[u8;1]] = &[[0x44], [0xBE]];

#[unsafe(no_mangle)]
fn main() -> ! {
    let mut device_address : one_wire_rom_address_t = one_wire_rom_address_t{
        address: [0; 8],
    };

    let mut reading_buffer : [u8; 2] = [0; 2];
    let mut is_converting : [u8;1] = [0;1];
    let mut temperature : f32 = 0.0;

    let mut one_wire : one_wire_t = one_wire_t::default();

    one_wire.data_pin = pin;

    one_wire_open(&mut one_wire);

    //ll_one_wire_reset();

    loop {
        is_converting[0] = 0x0;
        one_wire_read_rom(&mut one_wire, &mut device_address);
        one_wire_write_byte(&mut one_wire, &device_command[0], 1);
        

        //the device writes 1s when it finished conversion
        while is_converting[0] != 0xFF {
            one_wire_read_byte(&mut one_wire, &mut is_converting, 1);
        }

        //Delay_ms(150);


        one_wire_read_rom(&mut one_wire, &mut device_address);
        one_wire_write_byte(&mut one_wire, &device_command[1], 1);
        one_wire_read_byte(&mut one_wire,&mut reading_buffer, 2);
        one_wire_reset(&mut one_wire);

        match get_temperature_from_reading(&mut reading_buffer) {
            Ok(t) => temperature = t,
            Err(_) => temperature = -500.0,
        }

    }
}

pub fn get_temperature_from_reading(reading_buffer : &[u8]) -> Result<f32, ()> {
    if reading_buffer.len() < 2 {
        return Err(());
    }

    let mut local_temperature: f32 = 0.0;

    //assuming normal mode

    if (reading_buffer[0] & (1 << 0)) > 0 {
        local_temperature += 0.0625;
    }
    if (reading_buffer[0] & (1 << 1)) > 0 {
        local_temperature += 0.125;
    }
    if (reading_buffer[0] & (1 << 2)) > 0 {
        local_temperature += 0.25;
    }
    if (reading_buffer[0] & (1 << 3)) > 0 {
        local_temperature += 0.5;
    }
    if (reading_buffer[0] & (1 << 4)) > 0 {
        local_temperature += 1.0;
    }
    if (reading_buffer[0] & (1 << 5)) > 0 {
        local_temperature += 2.0;
    }
    if (reading_buffer[0] & (1 << 6)) > 0 {
        local_temperature += 4.0;
    }
    if (reading_buffer[0] & (1 << 7)) > 0 {
        local_temperature += 8.0;
    }

    if (reading_buffer[1] & (1 << 0)) > 0 {
        local_temperature += 16.0;
    }
    if (reading_buffer[1] & (1 << 1)) > 0 {
        local_temperature += 32.0;
    }
    if (reading_buffer[1] & (1 << 2)) > 0 {
        local_temperature += 64.0;
    }

    if (reading_buffer[1] & (1 << 7)) > 0 {
        local_temperature = -local_temperature;
    }


    Ok(local_temperature)
}