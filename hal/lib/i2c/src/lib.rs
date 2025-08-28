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
#![allow(non_camel_case_types)]

use hal_ll::i2c_master::*;
use hal_target::*;


pub use hal_ll::i2c_master::hal_ll_i2c_master_speed_t as hal_i2c_master_speed_t;
pub use hal_ll::i2c_master::hal_ll_i2c_master_handle_register_t as hal_i2c_master_handle_register_t;
pub use hal_ll::i2c_master::HAL_LL_I2C_MASTER_ERROR as HAL_I2C_MASTER_ERROR;

type Result<T> = core::result::Result<T, HAL_I2C_MASTER_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_i2c_master_config_t {
    pub addr: u8,

    pub sda: hal_pin_name_t,
    pub scl: hal_pin_name_t,

    pub speed: hal_i2c_master_speed_t, 
    pub timeout_pass_count: u16,
}

impl Default for hal_i2c_master_config_t {
    fn default() -> Self {
        Self{ addr: 0, sda: HAL_PIN_NC, scl: HAL_PIN_NC, speed: hal_i2c_master_speed_t::I2C_MASTER_SPEED_100K, timeout_pass_count: 10000 }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_i2c_master_t {
    pub handle: hal_i2c_master_handle_register_t,
    pub config: hal_i2c_master_config_t
}

impl Default for hal_i2c_master_t {
    fn default() -> Self {
        Self { 
            handle: hal_i2c_master_handle_register_t{ i2c_master_handle: 0, init_ll_state: false }, 
            config: hal_i2c_master_config_t{addr: 0, sda: HAL_PIN_NC, scl: HAL_PIN_NC, speed: hal_i2c_master_speed_t::I2C_MASTER_SPEED_100K, timeout_pass_count: 10000 } 
        }
    }
}

static mut hal_module_state: [hal_i2c_master_handle_register_t; I2C_MODULE_COUNT as usize]  = [ 
    hal_i2c_master_handle_register_t{
        i2c_master_handle: 0xFFFF_FFFF,
        init_ll_state: false,
        };
        I2C_MODULE_COUNT as usize];

fn hal_is_handle_register<'a>(hal_module_handle: &mut hal_i2c_master_t) -> Option<&'a mut hal_i2c_master_handle_register_t> {
    let mut hal_module_state_count: u8 = I2C_MODULE_COUNT;

    while hal_module_state_count > 0 
    {   
        unsafe{
            if hal_module_handle.handle.i2c_master_handle == hal_module_state[ (hal_module_state_count - 1) as usize ].i2c_master_handle
            {
                return Some(&mut hal_module_state[ (hal_module_state_count - 1) as usize]);
            }
        }
        hal_module_state_count = hal_module_state_count - 1;
    }
     return None;
}

pub fn hal_i2c_master_open(handle: &mut hal_i2c_master_t, hal_obj_open_state: bool) -> Result<()> {
    let mut hal_module_id: u8 = 0;
    let hal_obj: &mut hal_i2c_master_t = handle;

    match hal_is_handle_register( hal_obj ) {
        Some(_) => {
            if hal_obj_open_state
            {
                return Err(HAL_I2C_MASTER_ERROR::ACQUIRE_FAIL) 
            } else {
                return Ok(())
            }
        },
        None => {
            if !hal_obj_open_state {
                return Err(HAL_I2C_MASTER_ERROR::ACQUIRE_FAIL) 
            }
        },
    }
    unsafe{
        match hal_ll_i2c_master_register_handle(hal_obj.config.scl, hal_obj.config.sda, &mut hal_module_id )
        {

            Ok(h) => {
                hal_module_state[ hal_module_id as usize].i2c_master_handle = h.i2c_master_handle;
                (*hal_obj).handle = h;
                Ok(())
            },
            Err(_) => {
                hal_obj.handle = hal_i2c_master_t::default().handle;
                return Err(HAL_I2C_MASTER_ERROR::ACQUIRE_FAIL) 
            },
        }

    }

}

pub fn hal_i2c_master_set_slave_address(handle: &mut hal_i2c_master_t, config: hal_i2c_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    hal_ll_i2c_master_set_slave_address(hal_handle , config.addr);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_i2c_master_set_speed(handle: &mut hal_i2c_master_t, config: hal_i2c_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    hal_ll_i2c_master_set_speed(hal_handle , config.speed)?;
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_i2c_master_set_timeout(handle: &mut hal_i2c_master_t, config: hal_i2c_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    hal_ll_i2c_master_set_timeout(hal_handle , config.timeout_pass_count);
    Ok(())
}

pub fn hal_i2c_master_read(handle: &mut hal_i2c_master_t,  read_data_buf: &mut [u8], len_read_data: usize) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    if len_read_data == 0 {
        return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_i2c(hal_handle)?;
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_i2c_master_read(hal_handle, read_data_buf, len_read_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_i2c_master_write(handle: &mut hal_i2c_master_t,  write_data_buf: &mut [u8], len_write_data: usize) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    if len_write_data == 0 {
        return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_i2c(hal_handle)?;
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_i2c_master_write(hal_handle, write_data_buf, len_write_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_i2c_master_write_then_read(handle: &mut hal_i2c_master_t,  write_data_buf: &mut [u8], len_write_data: usize, 
                                                            read_data_buf: &mut [u8], len_read_data: usize) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    if len_write_data == 0 || len_read_data == 0 {
        return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_i2c(hal_handle)?;
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_i2c_master_write_then_read(hal_handle, write_data_buf, len_write_data, read_data_buf, len_read_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_i2c_master_close( handle: &mut hal_i2c_master_t) -> Result<()> {
    let hal_obj: &mut hal_i2c_master_t = handle;
    let hal_handle: &mut hal_i2c_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR),
    }

    if (hal_handle.i2c_master_handle != 0) & (hal_handle.i2c_master_handle != 0xFFFF_FFFF)
    {
        hal_ll_i2c_master_close( hal_handle );

        hal_obj.config = hal_i2c_master_config_t::default();
        *hal_obj = hal_i2c_master_t::default();
        hal_handle.i2c_master_handle = 0xFFFF_FFFF;

        return Ok(())
    }

    Err(HAL_I2C_MASTER_ERROR::I2C_MASTER_ERROR)
}