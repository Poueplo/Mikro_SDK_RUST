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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::target::*;
pub use mcu_definition::i2c::*;
use crate::gpio::*;
use crate::gpio::gpio_constants::*;
use system::{RCC_TypeDef, RCC_BASE};
use core::fmt;


pub const HAL_LL_I2C_DEFAULT_PASS_COUNT : u16 = 0xFFFF;

const HAL_LL_I2C1EN_BIT : u32 = 21;
const HAL_LL_I2C2EN_BIT : u32 = 22;
#[cfg(all(feature = "i2c3", feature = "f7"))]
const HAL_LL_I2C3EN_BIT : u32 = 23;
#[cfg(all(feature = "i2c4", feature = "f7"))]
const HAL_LL_I2C4EN_BIT : u32 = 24;
#[cfg(feature = "f7")]
const HAL_LL_I2C1SEL1_BIT : u32 = 17;
#[cfg(feature = "f7")]
const HAL_LL_I2C2SEL1_BIT : u32 = 19;
#[cfg(feature = "f7")]
const HAL_LL_I2C3SEL1_BIT : u32 = 21;
#[cfg(feature = "f7")]
const HAL_LL_I2C4SEL1_BIT : u32 = 23;

const HAL_LL_I2C_CR1_PE_BIT : u32 = 0;

const HAL_LL_I2C_CR1_FILTERS_OFF_MASK : u32 = 0xFFFFE0FF;
const HAL_LL_I2C_CR1_RXTX_INT_MASK : u32 = 0x6;


//const HAL_LL_I2C_CR2_START_BIT : u32 = 13;
const HAL_LL_I2C_CR2_STOP_BIT : u32 = 14;

const HAL_LL_I2C_CR2_SADD_MASK : u32 = 0x3FF;
const HAL_LL_I2C_CR2_NBYTES_MASK : u32 = 0xFF0000;
const HAL_LL_I2C_CR2_RD_WRN_MASK : u32 = 1 << 10;
const HAL_LL_I2C_CR2_HEAD10R_MASK : u32 = 1 << 12;
const HAL_LL_I2C_CR2_START_MASK : u32 = 1 << 13;
const HAL_LL_I2C_CR2_STOP_MASK : u32 = 1 << 14;
const HAL_LL_I2C_CR2_RELOAD_MASK : u32 = 1 << 24;
const HAL_LL_I2C_CR2_AUTOEND_MASK : u32 = 1 << 25;

const HAL_LL_I2C_ISR_TXE_BIT : u32 = 0;
const HAL_LL_I2C_ISR_TXIS_BIT : u32 = 1;
const HAL_LL_I2C_ISR_RXNE_BIT : u32 = 2;
const HAL_LL_I2C_ISR_STOPF_BIT : u32 = 5;
//const HAL_LL_I2C_ISR_ARLO_BIT : u32 = 9;

const HAL_LL_I2C_ICR_STOPCF_BIT : u32 = 5;

//const HAL_LL_I2C_LOOP_INDEX : u16 = 10000;

const HAL_LL_I2C_AF_CONFIG : u32 = GPIO_CFG_MODE_ALT_FUNCTION | GPIO_CFG_SPEED_HIGH | GPIO_CFG_OTYPE_OD;

#[derive(Debug)]
pub enum HAL_LL_I2C_MASTER_ERROR {
    I2C_MASTER_WRONG_PINS,
    I2C_MASTER_MODULE_ERROR,
    I2C_MASTER_TIMEOUT_START,
    I2C_MASTER_TIMEOUT_STOP,
    I2C_MASTER_TIMEOUT_WRITE,
    I2C_MASTER_TIMEOUT_READ,
    I2C_MASTER_ARBITRATION_LOST,
    I2C_MASTER_TIMEOUT_INIT,
    I2C_MASTER_TIMEOUT_WAIT_IDLE,
    I2C_MASTER_BUFFER_ERROR,
    I2C_MASTER_TOO_MUCH_BYTES_ERROR,
    ACQUIRE_FAIL,
    I2C_MASTER_ERROR
}

impl fmt::Display for HAL_LL_I2C_MASTER_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I2C_MASTER_WRONG_PINS => write!(f, "I2C_MASTER_WRONG_PINS occurred"),
            Self::I2C_MASTER_MODULE_ERROR => write!(f, "I2C_MASTER_MODULE_ERROR occurred"),
            Self::I2C_MASTER_TIMEOUT_START => write!(f, "I2C_MASTER_TIMEOUT_START occurred"),
            Self::I2C_MASTER_TIMEOUT_STOP => write!(f, "I2C_MASTER_TIMEOUT_STOP occurred"),
            Self::I2C_MASTER_TIMEOUT_WRITE => write!(f, "I2C_MASTER_TIMEOUT_WRITE occurred"),
            Self::I2C_MASTER_TIMEOUT_READ => write!(f, "I2C_MASTER_TIMEOUT_READ occurred"),
            Self::I2C_MASTER_ARBITRATION_LOST => write!(f, "I2C_MASTER_ARBITRATION_LOST occurred"),
            Self::I2C_MASTER_TIMEOUT_INIT => write!(f, "I2C_MASTER_TIMEOUT_INIT occurred"),
            Self::I2C_MASTER_TIMEOUT_WAIT_IDLE => write!(f, "I2C_MASTER_TIMEOUT_WAIT_IDLE occurred"),
            Self::I2C_MASTER_BUFFER_ERROR => write!(f, "I2C_MASTER_BUFFER_ERROR occurred"),
            Self::I2C_MASTER_TOO_MUCH_BYTES_ERROR => write!(f, "I2C_MASTER_TOO_MUCH_BYTES_ERROR occurred"),
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),                    
            Self::I2C_MASTER_ERROR => write!(f, "I2C_MASTER_ERROR occurred"),                    
        }
    }
}


type Result<T> = core::result::Result<T, HAL_LL_I2C_MASTER_ERROR>;


#[derive(PartialEq)]
enum hal_ll_i2c_master_end_mode_t
{
    HAL_LL_I2C_MASTER_END_MODE_STOP,
    HAL_LL_I2C_MASTER_WRITE_THEN_READ
}


#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_i2c_master_speed_t
{
    I2C_MASTER_SPEED_100K = 100000,
    I2C_MASTER_SPEED_400K = 400000,
    I2C_MASTER_SPEED_1M = 1000000,
}

#[cfg(feature = "f7")]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_i2c_master_speed_config_reg_t
{
    I2C_TIMINGR_100K = 0x00303D5D,
    I2C_TIMINGR_400K = 0x0010061A,
    I2C_TIMINGR_1M = 0x00000107,
}


#[derive(Clone, Copy)]
struct hal_ll_i2c_pin_id {
    pub pin_scl: u8,
    pub pin_sda: u8,
}

struct hal_ll_i2c_pins_t {
    pub pin_scl: hal_ll_pin_af_t,
    pub pin_sda: hal_ll_pin_af_t,
}

struct hal_ll_i2c_hw_specifics_map_t
{
    pub base : hal_ll_base_addr_t,
    pub module_index : hal_ll_pin_name_t,
    pub pins : hal_ll_i2c_pins_t,
    pub speed : hal_ll_i2c_master_speed_t,
    pub address : u8,
    pub timeout : u16
}


#[derive(Clone, Copy, PartialEq)]
pub struct hal_ll_i2c_master_handle_register_t
{
    pub i2c_master_handle : handle_t,
    pub init_ll_state: bool
}

impl Default for hal_ll_i2c_master_handle_register_t {
    fn default() -> Self {
        Self {
            i2c_master_handle : 0,
            init_ll_state : false
        }
    }
}

#[repr(C)]
struct hal_ll_i2c_base_handle_t
{
    pub cr1: hal_ll_base_addr_t,
    pub cr2: hal_ll_base_addr_t,
    pub oar1: hal_ll_base_addr_t,
    pub oar2: hal_ll_base_addr_t,
    pub timingr: hal_ll_base_addr_t,
    pub timoutr: hal_ll_base_addr_t,
    pub isr: hal_ll_base_addr_t,
    pub icr: hal_ll_base_addr_t,
    pub pecr: hal_ll_base_addr_t,
    pub rxdr: hal_ll_base_addr_t,
    pub txdr: hal_ll_base_addr_t,
}




static mut hal_ll_module_state: [hal_ll_i2c_master_handle_register_t; I2C_MODULE_COUNT as usize]  = [ 
    hal_ll_i2c_master_handle_register_t{
        i2c_master_handle : 0, 
        init_ll_state : false
        };
        I2C_MODULE_COUNT as usize];

static mut hal_ll_i2c_hw_specifics_map:[hal_ll_i2c_hw_specifics_map_t; (I2C_MODULE_COUNT+1) as usize] = [
    #[cfg(feature = "i2c1")]
    hal_ll_i2c_hw_specifics_map_t{base: I2C1_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8), pins: hal_ll_i2c_pins_t{ pin_scl: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_sda: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, speed: hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K, address: 0, timeout: HAL_LL_I2C_DEFAULT_PASS_COUNT },
    #[cfg(feature = "i2c2")]
    hal_ll_i2c_hw_specifics_map_t{base: I2C2_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8), pins: hal_ll_i2c_pins_t{ pin_scl: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_sda: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, speed: hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K, address: 0, timeout: HAL_LL_I2C_DEFAULT_PASS_COUNT },
    #[cfg(feature = "i2c3")]
    hal_ll_i2c_hw_specifics_map_t{base: I2C3_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8), pins: hal_ll_i2c_pins_t{ pin_scl: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_sda: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, speed: hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K, address: 0, timeout: HAL_LL_I2C_DEFAULT_PASS_COUNT },
    #[cfg(feature = "i2c4")]
    hal_ll_i2c_hw_specifics_map_t{base: I2C4_BASE_ADDR, module_index: hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_4 as u8), pins: hal_ll_i2c_pins_t{ pin_scl: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_sda: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, speed: hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K, address: 0, timeout: HAL_LL_I2C_DEFAULT_PASS_COUNT },
    hal_ll_i2c_hw_specifics_map_t{base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_MODULE_ERROR as u8, pins: hal_ll_i2c_pins_t{ pin_scl: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_sda: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, speed: hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K, address: 0, timeout: 0}
];


///////// public functions

pub fn hal_ll_i2c_master_register_handle(scl: hal_ll_pin_name_t, sda: hal_ll_pin_name_t, hal_module_id: &mut u8) -> Result<hal_ll_i2c_master_handle_register_t> {
    let pin_check_result: u8;
    let mut index_list: hal_ll_i2c_pin_id = hal_ll_i2c_pin_id{ pin_scl: HAL_LL_PIN_NC, pin_sda: HAL_LL_PIN_NC };


    pin_check_result = hal_ll_i2c_master_check_pins(scl, sda, &mut index_list);
    if pin_check_result == HAL_LL_PIN_NC {
        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_WRONG_PINS);
    }
    
    unsafe{
        if (hal_ll_i2c_hw_specifics_map[pin_check_result as usize].pins.pin_scl.pin_name != scl)
        || (hal_ll_i2c_hw_specifics_map[pin_check_result as usize].pins.pin_sda.pin_name != sda) {
            hal_ll_i2c_master_alternate_functions_set_state(&mut hal_ll_i2c_hw_specifics_map[pin_check_result as usize], false );
            hal_ll_i2c_master_map_pins( pin_check_result as usize,  &mut index_list);
            hal_ll_i2c_master_alternate_functions_set_state(&mut hal_ll_i2c_hw_specifics_map[pin_check_result as usize], true );
        
            hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        }

        *hal_module_id = pin_check_result;

        hal_ll_module_state[pin_check_result as usize].i2c_master_handle = hal_ll_i2c_hw_specifics_map[pin_check_result as usize].base;
        
        Ok(hal_ll_module_state[pin_check_result as usize])
    }
}

pub fn hal_ll_module_configure_i2c(handle: &mut hal_ll_i2c_master_handle_register_t) -> Result<()> {
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_i2c_hw_specifics_map_local.module_index as usize;

    hal_ll_i2c_init( hal_ll_i2c_hw_specifics_map_local )?;
    unsafe{
        hal_ll_module_state[pin_check_result].i2c_master_handle = hal_ll_i2c_hw_specifics_map[pin_check_result].base;
        hal_ll_module_state[pin_check_result].init_ll_state = true;
    }
    hal_handle.init_ll_state = true;
    Ok(())
}

pub fn hal_ll_i2c_master_set_speed(handle: &mut hal_ll_i2c_master_handle_register_t, speed: hal_ll_i2c_master_speed_t) -> Result<()> {
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_i2c_hw_specifics_map_local.speed = speed;

    hal_ll_i2c_init( hal_ll_i2c_hw_specifics_map_local )?;

    hal_handle.init_ll_state = true;

    Ok(())
}

pub fn hal_ll_i2c_master_set_timeout(handle: &mut hal_ll_i2c_master_handle_register_t, timeout: u16) {
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    if hal_ll_i2c_hw_specifics_map_local.base != HAL_LL_MODULE_ERROR {
        hal_ll_i2c_hw_specifics_map_local.timeout = timeout;
    }
}

pub fn hal_ll_i2c_master_set_slave_address(handle: &mut hal_ll_i2c_master_handle_register_t, addr: u8) {
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    if hal_ll_i2c_hw_specifics_map_local.base != HAL_LL_MODULE_ERROR {
        hal_ll_i2c_hw_specifics_map_local.address = addr;
    }
}

pub fn hal_ll_i2c_master_read( handle: &mut hal_ll_i2c_master_handle_register_t, read_data_buf: &mut [u8], len_read_data: usize ) -> Result<()>{
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_ll_i2c_master_read_bare_metal( hal_ll_i2c_hw_specifics_map_local, read_data_buf, len_read_data, hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_END_MODE_STOP )
}

pub fn hal_ll_i2c_master_write(  handle: &mut hal_ll_i2c_master_handle_register_t, write_data_buf: &mut [u8], len_write_data: usize ) -> Result<()>{
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_ll_i2c_master_write_bare_metal( hal_ll_i2c_hw_specifics_map_local, write_data_buf, len_write_data, hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_END_MODE_STOP )
}

pub fn hal_ll_i2c_master_write_then_read(  handle: &mut hal_ll_i2c_master_handle_register_t, write_data_buf: &mut [u8], len_write_data: usize,  read_data_buf: &mut [u8], len_read_data: usize) -> Result<()>{
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    match hal_ll_i2c_master_write_bare_metal( hal_ll_i2c_hw_specifics_map_local, write_data_buf, len_write_data, hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_WRITE_THEN_READ ) {
        Ok(_) => (),
        Err(e) =>  return Err(e),
    }

    hal_ll_i2c_master_read_bare_metal( hal_ll_i2c_hw_specifics_map_local, read_data_buf, len_read_data, hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_WRITE_THEN_READ )?;
    Ok(())
    

}

pub fn hal_ll_i2c_master_close(  handle: &mut hal_ll_i2c_master_handle_register_t) {
    let hal_handle : &mut hal_ll_i2c_master_handle_register_t = handle;
    let hal_ll_i2c_hw_specifics_map_local: &mut hal_ll_i2c_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_i2c_hw_specifics_map_local.module_index as usize;

    if hal_handle.i2c_master_handle != 0 {
        *hal_handle = hal_ll_i2c_master_handle_register_t::default();

        hal_ll_i2c_hw_specifics_map_local.address = 0;
        hal_ll_i2c_hw_specifics_map_local.timeout = HAL_LL_I2C_DEFAULT_PASS_COUNT;
        hal_ll_i2c_hw_specifics_map_local.speed = hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_100K;

        hal_ll_i2c_master_set_clock(hal_ll_i2c_hw_specifics_map_local, true);
        hal_ll_i2c_master_alternate_functions_set_state( hal_ll_i2c_hw_specifics_map_local, false );
        hal_ll_i2c_master_set_clock(hal_ll_i2c_hw_specifics_map_local, false);

        hal_ll_i2c_hw_specifics_map_local.pins.pin_scl.pin_name = HAL_LL_PIN_NC;
        hal_ll_i2c_hw_specifics_map_local.pins.pin_sda.pin_name = HAL_LL_PIN_NC;
        hal_ll_i2c_hw_specifics_map_local.pins.pin_scl.pin_af = 0;
        hal_ll_i2c_hw_specifics_map_local.pins.pin_sda.pin_af = 0;

        unsafe{hal_ll_module_state[pin_check_result as usize] = *hal_handle;}
    }
}

///////// private functions
fn hal_ll_i2c_master_read_bare_metal(map: &mut hal_ll_i2c_hw_specifics_map_t, read_data_buf: &mut [u8], len_read_data: usize, mode: hal_ll_i2c_master_end_mode_t) -> Result<()> {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    let mut time_counter: u16 = map.timeout;
    let mut transfer_counter: usize = 0;

    if read_data_buf.len() < len_read_data {
        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_BUFFER_ERROR);
    }

    if len_read_data > 0xFF {
        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TOO_MUCH_BYTES_ERROR);
    }

    
    hal_ll_i2c_master_configure_transfer(map, 
                                        ((map.address as u32) << 1 & HAL_LL_I2C_CR2_SADD_MASK ) |
                                        ((len_read_data as u32) << 16 & HAL_LL_I2C_CR2_NBYTES_MASK ) |
                                        HAL_LL_I2C_CR2_RD_WRN_MASK |
                                        HAL_LL_I2C_CR2_START_MASK);
    
    unsafe {
        while transfer_counter < len_read_data {
            while check_reg_bit( &(*i2c_ptr).isr as *const u32 as u32, HAL_LL_I2C_ISR_RXNE_BIT ) == 0 {
                if map.timeout > 0 {
                    if time_counter == 0 {
                        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TIMEOUT_READ);
                    }

                    time_counter -= 1;
                }
            }
            read_data_buf[ transfer_counter ] = (*i2c_ptr).rxdr as u8;
            transfer_counter += 1;
        }
    }

    if mode == hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_WRITE_THEN_READ || mode == hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_END_MODE_STOP {
        match hal_ll_i2c_master_stop( map ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    // currently unreachable
    // else {
    //     match hal_ll_i2c_master_start( map ) {
    //         Ok(_) => (),
    //         Err(e) => return Err(e),
    //     }
    // }

    hal_ll_i2c_master_reset_cr2( map );

    Ok(())
}

fn hal_ll_i2c_master_write_bare_metal(map: &mut hal_ll_i2c_hw_specifics_map_t, write_data_buf: &mut [u8], len_write_data: usize, mode: hal_ll_i2c_master_end_mode_t) -> Result<()> {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    let mut time_counter: u16 = map.timeout;
    let mut transfer_counter: usize = 0;

    if write_data_buf.len() < len_write_data {
        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_BUFFER_ERROR);
    }

    if len_write_data > 0xFF {
        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TOO_MUCH_BYTES_ERROR);
    }

    hal_ll_i2c_master_configure_transfer(map, 
                                        ((map.address as u32) << 1 & HAL_LL_I2C_CR2_SADD_MASK ) |
                                        ((len_write_data as u32) << 16 & HAL_LL_I2C_CR2_NBYTES_MASK ) |
                                        HAL_LL_I2C_CR2_START_MASK);
    
    unsafe {
        while transfer_counter < len_write_data {
            while check_reg_bit( &(*i2c_ptr).isr as *const u32 as u32, HAL_LL_I2C_ISR_TXIS_BIT ) == 0 {
                if map.timeout > 0 {
                    if time_counter == 0 {
                        return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TIMEOUT_WRITE);
                    }

                    time_counter -= 1;
                }
            }

            (*i2c_ptr).txdr = write_data_buf[ transfer_counter ] as u32;
            transfer_counter += 1;
        }

        while check_reg_bit( &(*i2c_ptr).isr as *const u32 as u32, HAL_LL_I2C_ISR_TXE_BIT ) == 0 {
            if map.timeout > 0 {
                if time_counter == 0 {
                    return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TIMEOUT_WRITE);
                }

                time_counter -= 1;
            }
        }

    }

    if mode == hal_ll_i2c_master_end_mode_t::HAL_LL_I2C_MASTER_END_MODE_STOP {
        match hal_ll_i2c_master_stop( map ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    hal_ll_i2c_master_reset_cr2( map );

    Ok(())
}

fn hal_ll_i2c_master_configure_transfer(map: &mut hal_ll_i2c_hw_specifics_map_t, config: u32 ) {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    unsafe {
        (*i2c_ptr).cr2 &= !(HAL_LL_I2C_CR2_SADD_MASK | 
                            HAL_LL_I2C_CR2_NBYTES_MASK | 
                            HAL_LL_I2C_CR2_RELOAD_MASK |
                            HAL_LL_I2C_CR2_AUTOEND_MASK |
                            HAL_LL_I2C_CR2_RD_WRN_MASK |
                            HAL_LL_I2C_CR2_START_MASK |
                            HAL_LL_I2C_CR2_STOP_MASK);

        (*i2c_ptr).cr2 |= config;
    }

}

fn hal_ll_i2c_master_reset_cr2(map: &mut hal_ll_i2c_hw_specifics_map_t) {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    unsafe {
        (*i2c_ptr).cr2 &= !(HAL_LL_I2C_CR2_SADD_MASK |
                            HAL_LL_I2C_CR2_HEAD10R_MASK |
                            HAL_LL_I2C_CR2_NBYTES_MASK |
                            HAL_LL_I2C_CR2_RELOAD_MASK |
                            HAL_LL_I2C_CR2_RD_WRN_MASK
                            );
    }
}

//currently dead code
// fn hal_ll_i2c_master_start(map: &mut hal_ll_i2c_hw_specifics_map_t) -> Result<()> {
//     let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;

//     unsafe{
//         set_reg_bit( &(*i2c_ptr).cr2 as *const u32 as u32, HAL_LL_I2C_CR2_START_BIT);
    
//         if check_reg_bit( &(*i2c_ptr).isr as *const u32 as u32, HAL_LL_I2C_ISR_ARLO_BIT ) == 1 {
//             return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_ARBITRATION_LOST);
//         }
//     }
//     #[allow(unused_variables)]
//     for loop_index in 0 .. HAL_LL_I2C_LOOP_INDEX {
//         ();
//     }

//     Ok(())
// }

fn hal_ll_i2c_master_stop(map: &mut hal_ll_i2c_hw_specifics_map_t) -> Result<()> {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    let mut time_counter: u16 = map.timeout;

    unsafe{
        set_reg_bit( &(*i2c_ptr).cr2 as *const u32 as u32, HAL_LL_I2C_CR2_STOP_BIT);
    
        while check_reg_bit( &(*i2c_ptr).isr as *const u32 as u32, HAL_LL_I2C_ISR_STOPF_BIT ) == 0 {
            if map.timeout > 0 {
                if time_counter == 0 {
                    return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TIMEOUT_READ);
                }

                time_counter -= 1;
            }
        }

        //present in the C implementation currently useless for the F7 family
        //slows down the flow a lot
        // if map.timeout < HAL_LL_I2C_LOOP_INDEX {
        //     #[allow(unused_variables)]
        //     for loop_index in 0 .. HAL_LL_I2C_LOOP_INDEX {
        //         ();
        //     }
        // }

        (*i2c_ptr).icr |= 1 << HAL_LL_I2C_ICR_STOPCF_BIT;
    }

    Ok(())
}

fn hal_ll_i2c_master_check_pins(scl: hal_ll_pin_name_t, sda: hal_ll_pin_name_t, index_list: &mut hal_ll_i2c_pin_id) -> u8 {
    let scl_map_size: u8 = hal_ll_i2c_scl_map.len() as u8 ;
    let sda_map_size: u8 = hal_ll_i2c_sda_map.len() as u8 ;

    let mut index_counter: u8 = 0;
    let mut hal_ll_module_id: u8 = 0;

    if HAL_LL_PIN_NC == scl || HAL_LL_PIN_NC == sda {
        return HAL_LL_PIN_NC;
    }

    for  scl_index in 0x00 .. scl_map_size
    {
        if hal_ll_i2c_scl_map[scl_index as usize].pin == scl {

            for  sda_index in 0x00 .. sda_map_size
            {
                if hal_ll_i2c_sda_map[sda_index as usize].pin == sda
                {   
                    if  hal_ll_i2c_scl_map[scl_index as usize].module_index == hal_ll_i2c_sda_map[sda_index as usize].module_index {
                        // Get module number
                        hal_ll_module_id = hal_ll_i2c_scl_map[scl_index as usize].module_index;
                        // Map pin names
                        index_list.pin_scl = scl_index;
                        index_list.pin_sda = sda_index;

                        if unsafe{hal_ll_module_state[ hal_ll_module_id as usize].i2c_master_handle} == hal_ll_i2c_master_handle_register_t::default().i2c_master_handle{
                            return hal_ll_module_id;
                        } else {
                            index_counter = index_counter + 1;
                            if I2C_MODULE_COUNT == index_counter {
                                return index_counter - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    if  index_counter > 0 {
        return hal_ll_module_id;
    } else {
        return HAL_LL_PIN_NC;
    }
}

#[allow(unused_variables, unused_assignments)]
fn hal_ll_get_specifics<'a>(handle: hal_ll_i2c_master_handle_register_t) -> &'a mut hal_ll_i2c_hw_specifics_map_t {
    let mut hal_ll_module_count: usize = I2C_MODULE_COUNT as usize;
    let mut hal_ll_module_error : usize = 0;
    hal_ll_module_error = hal_ll_module_count;
    
    unsafe{
        while hal_ll_module_count > 0 {
            hal_ll_module_count -= 1;

            let base: u32 = handle.i2c_master_handle;

            if base == hal_ll_i2c_hw_specifics_map[hal_ll_module_count].base {
                return &mut hal_ll_i2c_hw_specifics_map[hal_ll_module_count];
            }
        }

        return &mut hal_ll_i2c_hw_specifics_map[hal_ll_module_error];
    }
}

fn hal_ll_i2c_master_set_clock(map: &mut hal_ll_i2c_hw_specifics_map_t, hal_ll_state: bool) {
    
    #[cfg(feature = "i2c1")]
    if map.module_index == hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_1 as u8)
    {
        hal_ll_i2c1_set_clock(hal_ll_state);
        return;
    }
    #[cfg(feature = "i2c2")]
    if map.module_index == hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_2 as u8)
    {
        hal_ll_i2c2_set_clock(hal_ll_state);
        return;
    }
    #[cfg(feature = "i2c3")]
    if map.module_index == hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_3 as u8)
    {
        hal_ll_i2c3_set_clock(hal_ll_state);
        return;
    }
    #[cfg(feature = "i2c4")]
    if map.module_index == hal_ll_i2c_module_num(i2c_modules::I2C_MODULE_4 as u8)
    {
        hal_ll_i2c4_set_clock(hal_ll_state);
        return;
    }
}

fn hal_ll_i2c_master_map_pins(module_index: usize, index_list: &mut hal_ll_i2c_pin_id) {
    unsafe{
        // Map new pins
        hal_ll_i2c_hw_specifics_map[module_index].pins.pin_scl.pin_name = hal_ll_i2c_scl_map[ index_list.pin_scl as usize].pin;
        hal_ll_i2c_hw_specifics_map[module_index].pins.pin_sda.pin_name = hal_ll_i2c_sda_map[ index_list.pin_sda as usize].pin;
        // SCL and SDA could have different alternate function settings, hence save both AF values
        hal_ll_i2c_hw_specifics_map[module_index].pins.pin_scl.pin_af = hal_ll_i2c_scl_map[ index_list.pin_scl as usize].af;
        hal_ll_i2c_hw_specifics_map[module_index].pins.pin_sda.pin_af = hal_ll_i2c_sda_map[ index_list.pin_sda as usize].af;
    }
}

fn hal_ll_i2c_master_alternate_functions_set_state(map: &mut hal_ll_i2c_hw_specifics_map_t, hal_ll_state: bool) {
    let mut module: module_struct = module_struct::default();

    if ((*map).pins.pin_scl.pin_name != HAL_LL_PIN_NC) && ((*map).pins.pin_sda.pin_name != HAL_LL_PIN_NC)  {
        module.pins[0] = VALUE( (*map).pins.pin_scl.pin_name, (*map).pins.pin_scl.pin_af );
        module.pins[1] = VALUE( (*map).pins.pin_sda.pin_name, (*map).pins.pin_sda.pin_af );
        

        module.configs[0] = HAL_LL_I2C_AF_CONFIG;
        module.configs[1] = HAL_LL_I2C_AF_CONFIG;

        hal_ll_gpio_module_struct_init( &mut module, hal_ll_state );
    }
}

fn hal_ll_i2c_get_speed(speed: hal_ll_i2c_master_speed_t) -> u32 {
    
    if speed == hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_400K {
        return hal_ll_i2c_master_speed_config_reg_t::I2C_TIMINGR_400K as u32;
    } else if speed == hal_ll_i2c_master_speed_t::I2C_MASTER_SPEED_1M {
        return hal_ll_i2c_master_speed_config_reg_t::I2C_TIMINGR_1M as u32;
    }

    hal_ll_i2c_master_speed_config_reg_t::I2C_TIMINGR_100K as u32
}

fn hal_ll_i2c1_set_clock(hal_ll_state : bool) {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        if hal_ll_state {
            #[cfg(feature = "f7")]
            {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C1EN_BIT );
                set_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C1SEL1_BIT );
            }
        } else {
            #[cfg(feature = "f7")]
            {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C1EN_BIT );
                clear_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C1SEL1_BIT );
            }
        }
    }
}

fn hal_ll_i2c2_set_clock(hal_ll_state : bool) {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        if hal_ll_state {
            #[cfg(feature = "f7")]
            {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C2EN_BIT );
                set_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C2SEL1_BIT );
            }
        } else {
            #[cfg(feature = "f7")]
            {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C2EN_BIT );
                clear_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C2SEL1_BIT );
            }
        }
    }
}

#[cfg(feature = "i2c3")]
fn hal_ll_i2c3_set_clock(hal_ll_state : bool) {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        if hal_ll_state {
            #[cfg(feature = "f7")]
            {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C3EN_BIT );
                set_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C3SEL1_BIT );
            }
        } else {
            #[cfg(feature = "f7")]
            {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C3EN_BIT );
                clear_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C3SEL1_BIT );
            }
        }
    }
}

#[cfg(feature = "i2c4")]
fn hal_ll_i2c4_set_clock(hal_ll_state : bool) {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        if hal_ll_state {
            #[cfg(feature = "f7")]
            {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C4EN_BIT );
                set_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C4SEL1_BIT );
            }
        } else {
            #[cfg(feature = "f7")]
            {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_I2C4EN_BIT );
                clear_reg_bit( &(*rcc_ptr).DCKCFGR2 as *const u32 as u32, HAL_LL_I2C4SEL1_BIT );
            }
        }
    }
}

fn hal_ll_i2c_hw_init(map: &mut hal_ll_i2c_hw_specifics_map_t) -> Result<()> {
    let i2c_ptr : *mut hal_ll_i2c_base_handle_t = map.base as *mut hal_ll_i2c_base_handle_t;
    let mut time_counter: u16 = map.timeout;

    unsafe{
        clear_reg_bit( &(*i2c_ptr).cr1 as *const u32 as u32, HAL_LL_I2C_CR1_PE_BIT );

        while check_reg_bit( &(*i2c_ptr).cr1 as *const u32 as u32, HAL_LL_I2C_CR1_PE_BIT ) == 1 {
            if map.timeout > 0 {
                if time_counter == 0 {
                    return Err(HAL_LL_I2C_MASTER_ERROR::I2C_MASTER_TIMEOUT_INIT);
                }

                time_counter -= 1;
            }
        }

        (*i2c_ptr).cr1 &= HAL_LL_I2C_CR1_FILTERS_OFF_MASK;

        (*i2c_ptr).timingr = hal_ll_i2c_get_speed(map.speed);


        (*i2c_ptr).cr1 |= HAL_LL_I2C_CR1_RXTX_INT_MASK;

        set_reg_bit( &(*i2c_ptr).cr1  as *const u32 as u32, HAL_LL_I2C_CR1_PE_BIT );
    }
    Ok(())
}

fn hal_ll_i2c_init(map: &mut hal_ll_i2c_hw_specifics_map_t) -> Result<()> {
    hal_ll_i2c_master_set_clock(map, true);
    hal_ll_i2c_hw_init(map)
}