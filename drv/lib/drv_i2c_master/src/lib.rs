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
#![allow(non_upper_case_globals)]

use hal_i2c_master::*;
use drv_name::*;

pub use hal_i2c_master::hal_i2c_master_config_t as i2c_master_config_t;
pub use hal_i2c_master::hal_i2c_master_t as i2c_master_t;
pub use hal_i2c_master::hal_i2c_master_speed_t as i2c_master_speed_t;
pub use hal_i2c_master::HAL_I2C_MASTER_ERROR as  DRV_I2C_MASTER_ERROR;

type Result<T> = core::result::Result<T, DRV_I2C_MASTER_ERROR>;

static mut previous_initialized: i2c_master_t = i2c_master_t{ 
    handle: hal_i2c_master_handle_register_t{ i2c_master_handle: 0, init_ll_state: false },
    config: i2c_master_config_t{ addr: 0, sda: HAL_PIN_NC, scl: HAL_PIN_NC, speed: hal_i2c_master_speed_t::I2C_MASTER_SPEED_100K, timeout_pass_count: 10000 }, 
    };

fn _acquire( obj: &mut i2c_master_t, obj_open_state: bool) -> Result<()> {
    if obj_open_state && (unsafe{previous_initialized == *obj}) {
        return Err(DRV_I2C_MASTER_ERROR::ACQUIRE_FAIL);
    }

    if unsafe{previous_initialized != *obj} {
        match hal_i2c_master_open( obj, obj_open_state ) {
            Ok(_) => {
                unsafe{previous_initialized = *obj;}
                return Ok(())
            },
            Err(e) => return Err(e),
        }
    }

    Ok(())
}


pub fn i2c_master_open( obj: &mut i2c_master_t, config: i2c_master_config_t ) -> Result<()> {
    obj.config = config;

    match _acquire( obj, true ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn i2c_master_set_speed( obj: &mut i2c_master_t, speed: hal_i2c_master_speed_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.speed = speed;
    
    match hal_i2c_master_set_speed( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_set_timeout( obj: &mut i2c_master_t, timeout_pass_count: u16 ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.timeout_pass_count = timeout_pass_count;
    
    match hal_i2c_master_set_timeout( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_set_slave_address( obj: &mut i2c_master_t, address: u8 ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.addr = address;
    
    match hal_i2c_master_set_slave_address( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_write( obj: &mut i2c_master_t, write_data_buf: &mut [u8], len_write_data: usize ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    match hal_i2c_master_write( obj, write_data_buf, len_write_data ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_read( obj: &mut i2c_master_t, read_data_buf: &mut [u8], len_read_data: usize ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    match hal_i2c_master_read( obj, read_data_buf, len_read_data ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_write_then_read( obj: &mut i2c_master_t, write_data_buf: &mut [u8], len_write_data: usize, read_data_buf: &mut [u8], len_read_data: usize ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    
    match hal_i2c_master_write_then_read( obj, write_data_buf, len_write_data, read_data_buf, len_read_data ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn i2c_master_close(obj: &mut i2c_master_t) -> Result<()> {
    match hal_i2c_master_close(obj) {
        Ok(_) => {
            unsafe{previous_initialized = i2c_master_t::default();}
            Ok(())
        },
        Err(e) => Err(e),
    }
}