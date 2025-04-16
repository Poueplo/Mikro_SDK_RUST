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

use hal_ll_spi_master::*;
use hal_target::*;
use hal_target::pin_names::*;
use hal_gpio::*;

pub use hal_ll_spi_master::hal_ll_spi_master_handle_register_t as hal_spi_master_handle_register_t;
pub use hal_ll_spi_master::hal_ll_spi_master_mode_t as hal_spi_master_mode_t;
pub use hal_ll_spi_master::HAL_LL_SPI_MASTER_ERROR as HAL_SPI_MASTER_ERROR;
pub use hal_ll_spi_master::HAL_LL_SPI_MASTER_SPEED_100K as HAL_SPI_MASTER_SPEED_100K;
pub use hal_ll_spi_master::SPI_MASTER_MODE_DEFAULT;

type Result<T> = core::result::Result<T, HAL_SPI_MASTER_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_spi_master_config_t {
    pub default_write_data: u8,

    pub sck: hal_pin_name_t,
    pub miso: hal_pin_name_t,
    pub mosi: hal_pin_name_t,

    pub speed: u32, 
    pub mode: hal_spi_master_mode_t,
}

impl Default for hal_spi_master_config_t {
    fn default() -> Self {
        Self { 
            default_write_data: 0x00, 
            sck: HAL_PIN_NC, 
            miso: HAL_PIN_NC, 
            mosi: HAL_PIN_NC, 
            speed: HAL_SPI_MASTER_SPEED_100K, 
            mode: SPI_MASTER_MODE_DEFAULT
        }
    }
}

#[derive(PartialEq)]
pub enum hal_spi_master_chip_select_polarity_t {
    SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_LOW = 0, /*< CS active low. */
    SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_HIGH,    /*< CS active high. */
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_spi_master_t {
    pub handle: hal_spi_master_handle_register_t,
    pub config: hal_spi_master_config_t
}

impl Default for hal_spi_master_t {
    fn default() -> Self {
        Self { 
            handle: hal_spi_master_handle_register_t{ spi_master_handle: 0, init_ll_state: false }, 
            config: hal_spi_master_config_t{ default_write_data: 0x00, sck: HAL_PIN_NC, miso: HAL_PIN_NC, mosi: HAL_PIN_NC, speed: HAL_SPI_MASTER_SPEED_100K, mode: SPI_MASTER_MODE_DEFAULT } 
        }
    }
}

static mut hal_module_state: [hal_spi_master_handle_register_t; SPI_MODULE_COUNT as usize]  = [ 
    hal_spi_master_handle_register_t{
        spi_master_handle: 0xFFFF_FFFF,
        init_ll_state: false,
        };
        SPI_MODULE_COUNT as usize];

static mut hal_spi_master_chip_select_polarity: hal_spi_master_chip_select_polarity_t = hal_spi_master_chip_select_polarity_t::SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_LOW;

fn hal_is_handle_register<'a>(hal_module_handle: &mut hal_spi_master_t) -> Option<&'a mut hal_spi_master_handle_register_t> {
    let mut hal_module_state_count: u8 = SPI_MODULE_COUNT;

    while hal_module_state_count > 0 
    {   
        unsafe{
            if hal_module_handle.handle.spi_master_handle == hal_module_state[ (hal_module_state_count - 1) as usize ].spi_master_handle
            {
                return Some(&mut hal_module_state[ (hal_module_state_count - 1) as usize]);
            }
        }
        hal_module_state_count = hal_module_state_count - 1;
    }
     return None;
}

pub fn hal_spi_master_open(handle: &mut hal_spi_master_t, hal_obj_open_state: bool) -> Result<()> {
    let mut hal_module_id: u8 = 0;
    let hal_obj: &mut hal_spi_master_t = handle;

    match hal_is_handle_register( hal_obj ) {
        Some(_) => {
            if hal_obj_open_state
            {
                return Err(HAL_SPI_MASTER_ERROR::ACQUIRE_FAIL) 
            } else {
                return Ok(())
            }
        },
        None => {
            if !hal_obj_open_state {
                return Err(HAL_SPI_MASTER_ERROR::ACQUIRE_FAIL) 
            }
        },
    }
    unsafe{
        match hal_ll_spi_master_register_handle(hal_obj.config.sck, hal_obj.config.miso, hal_obj.config.mosi, &mut hal_module_id )
        {

            Ok(h) => {
                hal_module_state[ hal_module_id as usize] = h;
                (*hal_obj).handle = hal_module_state[ hal_module_id as usize];
                Ok(())
            },
            Err(_) => {
                hal_obj.handle = hal_spi_master_t::default().handle;
                return Err(HAL_SPI_MASTER_ERROR::ACQUIRE_FAIL) 
            },
        }

    }

}

pub fn hal_spi_master_set_chip_select_polarity(polarity: hal_spi_master_chip_select_polarity_t) {
    unsafe {hal_spi_master_chip_select_polarity = polarity}
}

pub fn hal_spi_master_set_speed(handle: &mut hal_spi_master_t, config: hal_spi_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    hal_ll_spi_master_set_speed(hal_handle , config.speed);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_spi_master_set_mode(handle: &mut hal_spi_master_t, config: hal_spi_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    hal_ll_spi_master_set_mode(hal_handle , config.mode);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_spi_master_set_default_write_data(handle: &mut hal_spi_master_t, config: hal_spi_master_config_t) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    hal_ll_spi_master_set_default_write_data(hal_handle , config.default_write_data);
    Ok(())
}

pub fn hal_spi_master_select_device(chip_select: hal_pin_name_t) {
    let mut struct_cs: hal_gpio_pin_t = hal_gpio_pin_t::default();

    hal_gpio_configure_pin(&mut struct_cs, chip_select, hal_gpio_direction_t::HAL_GPIO_DIGITAL_OUTPUT);

    if chip_select != HAL_LL_PIN_NC {
        unsafe {
            if hal_spi_master_chip_select_polarity == hal_spi_master_chip_select_polarity_t::SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_HIGH {
                hal_gpio_set_pin_output( &mut struct_cs );
            } else {
                hal_gpio_clear_pin_output( &mut struct_cs );
            }
        }
    }
}

pub fn hal_spi_master_deselect_device(chip_select: hal_pin_name_t) {
    let mut struct_cs: hal_gpio_pin_t = hal_gpio_pin_t::default();

    hal_gpio_configure_pin(&mut struct_cs, chip_select, hal_gpio_direction_t::HAL_GPIO_DIGITAL_OUTPUT);

    if chip_select != HAL_LL_PIN_NC {
        unsafe {
            if hal_spi_master_chip_select_polarity == hal_spi_master_chip_select_polarity_t::SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_HIGH {
                hal_gpio_clear_pin_output( &mut struct_cs );
            } else {
                hal_gpio_set_pin_output( &mut struct_cs );
            }
        }
    }
}

pub fn hal_spi_master_read(handle: &mut hal_spi_master_t,  read_data_buf: &mut [u8], len_read_data: usize) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    if len_read_data == 0 {
        return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_spi(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_spi_master_read(hal_handle, read_data_buf, len_read_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_spi_master_write(handle: &mut hal_spi_master_t,  write_data_buffer: &mut [u8], write_data_length: usize) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    if write_data_length == 0 {
        return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_spi(hal_handle);
        hal_obj.handle = *hal_handle;
    }
    
    match hal_ll_spi_master_write(hal_handle, write_data_buffer, write_data_length) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_spi_master_write_then_read(handle: &mut hal_spi_master_t,  write_data_buf: &mut [u8], len_write_data: usize, 
                                        read_data_buf: &mut [u8], len_read_data: usize) -> Result<()> {
   let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    if len_write_data == 0 || len_read_data == 0 {
        return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_spi(hal_handle);
        hal_obj.handle = *hal_handle;
    }

    match hal_ll_spi_master_write_then_read(hal_handle, write_data_buf, len_write_data, read_data_buf, len_read_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn hal_spi_master_close( handle: &mut hal_spi_master_t) -> Result<()> {
    let hal_obj: &mut hal_spi_master_t = handle;
    let hal_handle: &mut hal_spi_master_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR),
    }

    if (hal_handle.spi_master_handle != 0) & (hal_handle.spi_master_handle != 0xFFFF_FFFF)
    {
        hal_ll_spi_master_close( hal_handle );

        hal_obj.config = hal_spi_master_config_t::default();
        *hal_obj = hal_spi_master_t::default();
        hal_handle.spi_master_handle = 0xFFFF_FFFF;

        return Ok(())
    }

    Err(HAL_SPI_MASTER_ERROR::SPI_MASTER_ERROR)
}