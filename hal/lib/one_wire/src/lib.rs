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

use hal_ll::one_wire::*;
use hal_target::*;
use hal_target::pin_names::*;

pub use hal_ll::one_wire::hal_ll_one_wire_t as hal_one_wire_t;
pub use hal_ll::one_wire::HAL_LL_ONE_WIRE_ERROR as HAL_ONE_WIRE_ERROR;
pub use hal_ll::one_wire::hal_ll_one_wire_rom_address_t as hal_one_wire_rom_address_t;


type Result<T> = core::result::Result<T, HAL_ONE_WIRE_ERROR>;

static mut last_initialized : hal_one_wire_t = hal_one_wire_t {
    data_pin: HAL_LL_PIN_NC,
    state: false,
};

pub fn hal_one_wire_open(obj : &mut hal_one_wire_t ) -> Result<()> {
    if obj.data_pin == HAL_LL_PIN_NC || obj.state {
        return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
    }

    hal_ll_one_wire_open( obj );

    unsafe {
        last_initialized = *obj;
    }

    Ok(())
}

pub fn hal_one_wire_reset(obj : &mut hal_one_wire_t ) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }

        if hal_ll_one_wire_reset() != 0 {
            Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR)
        } else {
            Ok(())
        }
    }
}

pub fn hal_one_wire_read_rom(obj : &mut hal_one_wire_t , device_rom_address : &mut hal_one_wire_rom_address_t) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_read_rom(device_rom_address)
    }
}

pub fn hal_one_wire_skip_rom(obj : &mut hal_one_wire_t) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_skip_rom()
    }
}

pub fn hal_one_wire_match_rom(obj : &mut hal_one_wire_t , device_rom_address : &mut hal_one_wire_rom_address_t) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_match_rom(device_rom_address)
    }
}

pub fn hal_one_wire_search_first_device(obj : &mut hal_one_wire_t , one_wire_device_list : &mut hal_one_wire_rom_address_t) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_search_first_device(obj, one_wire_device_list)
    }
}

pub fn hal_one_wire_search_next_device(obj : &mut hal_one_wire_t , one_wire_device_list : &mut hal_one_wire_rom_address_t) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_search_first_device(obj, one_wire_device_list)
    }
}

pub fn hal_one_wire_write_byte(obj : &mut hal_one_wire_t , write_data_buffer : &[u8], write_data_length : usize) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_write_byte(write_data_buffer, write_data_length);
        Ok(())
    }
}

pub fn hal_one_wire_read_byte(obj : &mut hal_one_wire_t , read_data_buffer : &mut[u8], read_data_length : usize) -> Result<()> {
    unsafe {
        if !obj.state {
            return Err(HAL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
        }

        if *obj != last_initialized {
            hal_ll_one_wire_reconfigure( obj );
            last_initialized = *obj;
        }


        hal_ll_one_wire_read_byte(read_data_buffer, read_data_length);
        Ok(())
    }
}