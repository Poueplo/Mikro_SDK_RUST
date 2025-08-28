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

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::target::*;
pub use mcu_definition::spi::*;
use crate::gpio::*;
use crate::gpio::gpio_constants::*;
use system::{rcc_get_clocks_frequency, RCC_ClocksTypeDef, RCC_TypeDef, RCC_BASE};
use core::fmt;

pub const HAL_LL_SPI_MASTER_SPEED_100K : u32 = 100_000;

#[cfg(feature = "spi1")]
const HAL_LL_SPI1EN_BIT : u32 = 12;
#[cfg(feature = "spi2")]
const HAL_LL_SPI2EN_BIT : u32 = 14;
#[cfg(feature = "spi3")]
const HAL_LL_SPI3EN_BIT : u32 = 15;
#[cfg(feature = "spi4")]
const HAL_LL_SPI4EN_BIT : u32 = 13;
#[cfg(feature = "spi5")]
const HAL_LL_SPI5EN_BIT : u32 = 20;
#[cfg(feature = "spi6")]
const HAL_LL_SPI6EN_BIT : u32 = 21;

const HAL_LL_SPI_MASTER_CLK_PHASE : u32    = 0;
const HAL_LL_SPI_MASTER_CLK_POLARITY : u32 = 1;
const HAL_LL_SPI_MASTER_SELECTION : u32    = 2;
const HAL_LL_SPI_MASTER_BR_CONTROL : u32   = 3;
const HAL_LL_SPI_MASTER_ENABLE : u32       = 6;
const HAL_LL_SPI_MASTER_SSI_1 : u32        = 8;
const HAL_LL_SPI_MASTER_SSM_ENABLE : u32   = 9;

const HAL_LL_SPI_MASTER_RXNE : u32 = 0;

const HAL_LL_SPI_MASTER_SSOE_BIT : u32 = 2;

#[cfg(feature = "f7")]
const HAL_LL_SPI_MASTER_FRXTH : u32 = 1<<12;
#[cfg(not(feature = "f7"))]
const HAL_LL_SPI_MASTER_FRXTH : u32 = 0;



const HAL_LL_SPI_CONFIG : u32 = GPIO_CFG_MODE_ALT_FUNCTION | GPIO_CFG_SPEED_HIGH | GPIO_CFG_OTYPE_PP;

#[derive(Debug)]
pub enum HAL_LL_SPI_MASTER_ERROR {
    SPI_MASTER_WRONG_PINS,
    ACQUIRE_FAIL,
    SPI_MASTER_ERROR,
}

impl fmt::Display for HAL_LL_SPI_MASTER_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SPI_MASTER_WRONG_PINS => write!(f, "SPI_MASTER_WRONG_PINS occurred"),                  
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),                  
            Self::SPI_MASTER_ERROR => write!(f, "SPI_MASTER_ERROR occurred"),                  
        }
    }
}


type Result<T> = core::result::Result<T, HAL_LL_SPI_MASTER_ERROR>;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum hal_ll_spi_master_mode_t
{
    SPI_MASTER_MODE_0 = 0,
    SPI_MASTER_MODE_1,
    SPI_MASTER_MODE_2,
    SPI_MASTER_MODE_3,
}

pub const SPI_MASTER_MODE_DEFAULT : hal_ll_spi_master_mode_t = hal_ll_spi_master_mode_t::SPI_MASTER_MODE_0;


#[derive(Clone, Copy)]
pub struct hal_ll_spi_pin_id {
    pub pin_sck : u8,
    pub pin_miso : u8,
    pub pin_mosi : u8,    
}

pub struct hal_ll_spi_pin_t {
    pub pin_sck : hal_ll_pin_af_t,
    pub pin_miso : hal_ll_pin_af_t,
    pub pin_mosi : hal_ll_pin_af_t,    
}

struct hal_ll_spi_hw_specifics_map_t
{
    pub base : hal_ll_base_addr_t,
    pub module_index : hal_ll_pin_name_t,
    pub pins : hal_ll_spi_pin_t,
    pub dummy_data : u8,
    pub speed : u32,
    pub hw_actual_speed : u32,
    pub mode : hal_ll_spi_master_mode_t
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_ll_spi_master_handle_register_t
{
    pub spi_master_handle : handle_t,
    pub init_ll_state: bool
}

impl Default for hal_ll_spi_master_handle_register_t {
    fn default() -> Self {
        Self {
            spi_master_handle : 0,
            init_ll_state : false
        }
    }
}

#[repr(C)]
union dr_register {
    dr: hal_ll_base_addr_t,
    data_byte: u8,
}

#[repr(C)]
struct hal_ll_spi_master_base_handle_t {
    pub cr1: hal_ll_base_addr_t,          /* Address offset 0x00 */
    pub cr2: hal_ll_base_addr_t,          /* Address offset 0x04 */
    pub sr: hal_ll_base_addr_t,           /* Address offset 0x08 */
    pub dr: dr_register,                  /* Address offset 0x0C */
    pub crcpr: hal_ll_base_addr_t,        /* Address offset 0x10 */
    pub rxcrcr: hal_ll_base_addr_t,       /* Address offset 0x14 */
    pub txcrcr: hal_ll_base_addr_t,       /* Address offset 0x18 */
    pub i2scfgr: hal_ll_base_addr_t,      /* Address offset 0x1C */
    pub i2spr: hal_ll_base_addr_t,        /* Address offset 0x20 */
}


static mut hal_ll_module_state: [hal_ll_spi_master_handle_register_t; SPI_MODULE_COUNT as usize]  = [ 
    hal_ll_spi_master_handle_register_t{
        spi_master_handle : 0, 
        init_ll_state : false
        };
        SPI_MODULE_COUNT as usize];

static mut hal_ll_spi_hw_specifics_map: [hal_ll_spi_hw_specifics_map_t; (SPI_MODULE_COUNT + 1) as usize] = [
    #[cfg(feature = "spi1")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI1_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    #[cfg(feature = "spi2")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI2_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    #[cfg(feature = "spi3")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI3_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    #[cfg(feature = "spi4")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI4_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_4 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    #[cfg(feature = "spi5")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI5_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_5 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    #[cfg(feature = "spi6")]
    hal_ll_spi_hw_specifics_map_t{ base: SPI6_MASTER_BASE_ADDR, module_index: hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_6 as u8), pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: HAL_LL_SPI_MASTER_SPEED_100K, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
    hal_ll_spi_hw_specifics_map_t{ base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_MODULE_ERROR as u8, pins: hal_ll_spi_pin_t{ pin_sck: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_miso: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_mosi: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } }, dummy_data: 0, speed: 0, hw_actual_speed: 0, mode: SPI_MASTER_MODE_DEFAULT },
];


///////// public functions

pub fn hal_ll_spi_master_register_handle(sck: hal_ll_pin_name_t, miso: hal_ll_pin_name_t, mosi: hal_ll_pin_name_t, hal_module_id: &mut u8) -> Result<hal_ll_spi_master_handle_register_t> {
    let pin_check_result: u8;
    let mut index_list: hal_ll_spi_pin_id = 
        hal_ll_spi_pin_id{ pin_sck: HAL_LL_PIN_NC, pin_miso: HAL_LL_PIN_NC, pin_mosi: HAL_LL_PIN_NC};

    pin_check_result = hal_ll_spi_master_check_pins(sck, miso, mosi, &mut index_list);
    if pin_check_result == HAL_LL_PIN_NC {
        return Err(HAL_LL_SPI_MASTER_ERROR::SPI_MASTER_WRONG_PINS);
    }
    
    unsafe{
        if (hal_ll_spi_hw_specifics_map[pin_check_result as usize].pins.pin_sck.pin_name != sck)
        || (hal_ll_spi_hw_specifics_map[pin_check_result as usize].pins.pin_miso.pin_name != miso)
        || (hal_ll_spi_hw_specifics_map[pin_check_result as usize].pins.pin_mosi.pin_name != mosi) {
            hal_ll_spi_master_alternate_functions_set_state(&mut hal_ll_spi_hw_specifics_map[pin_check_result as usize], false );
            hal_ll_spi_master_map_pins( pin_check_result as usize,  &mut index_list);
            hal_ll_spi_master_alternate_functions_set_state(&mut hal_ll_spi_hw_specifics_map[pin_check_result as usize], true );
        
            hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        }

        *hal_module_id = pin_check_result;

        hal_ll_module_state[pin_check_result as usize].spi_master_handle = hal_ll_spi_hw_specifics_map[pin_check_result as usize].base;
        
        Ok(hal_ll_module_state[pin_check_result as usize])
    }
}

pub fn hal_ll_module_configure_spi(handle: &mut hal_ll_spi_master_handle_register_t) {
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_spi_hw_specifics_map_local.module_index as usize;

    hal_ll_spi_init( hal_ll_spi_hw_specifics_map_local );
    unsafe{
        hal_ll_module_state[pin_check_result].spi_master_handle = hal_ll_spi_hw_specifics_map[pin_check_result].base;
        hal_ll_module_state[pin_check_result].init_ll_state = true;
    }
    hal_handle.init_ll_state = true;
}

pub fn hal_ll_spi_master_set_default_write_data(handle: &mut hal_ll_spi_master_handle_register_t, dummy_data: u8) {
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    if hal_ll_spi_hw_specifics_map_local.base != HAL_LL_MODULE_ERROR {
        hal_ll_spi_hw_specifics_map_local.dummy_data = dummy_data;
    }
}


pub fn hal_ll_spi_master_set_speed(handle: &mut hal_ll_spi_master_handle_register_t, speed: u32) {
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_spi_hw_specifics_map_local.speed = speed;

    hal_ll_spi_init( hal_ll_spi_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}


pub fn hal_ll_spi_master_set_mode(handle: &mut hal_ll_spi_master_handle_register_t, mode: hal_ll_spi_master_mode_t) {
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_spi_hw_specifics_map_local.mode = mode;

    hal_ll_spi_init( hal_ll_spi_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_spi_master_write(handle: &mut hal_ll_spi_master_handle_register_t, write_data_buf: &mut [u8], len_write_data: usize ) -> Result<()>{
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_ll_spi_master_write_bare_metal( hal_ll_spi_hw_specifics_map_local, write_data_buf, len_write_data);
    Ok(())
}

pub fn hal_ll_spi_master_read(handle: &mut hal_ll_spi_master_handle_register_t, read_data_buf: &mut [u8], len_read_data: usize ) -> Result<()>{
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_ll_spi_master_read_bare_metal( hal_ll_spi_hw_specifics_map_local, read_data_buf, len_read_data);
    Ok(())
}

pub fn hal_ll_spi_master_write_then_read(handle: &mut hal_ll_spi_master_handle_register_t, write_data_buf: &mut [u8], len_write_data: usize, read_data_buf: &mut [u8], len_read_data: usize) -> Result<()>{
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_ll_spi_master_write_bare_metal( hal_ll_spi_hw_specifics_map_local, write_data_buf, len_write_data);
    hal_ll_spi_master_read_bare_metal( hal_ll_spi_hw_specifics_map_local, read_data_buf, len_read_data);
    Ok(())
}

pub fn hal_ll_spi_master_close(  handle: &mut hal_ll_spi_master_handle_register_t) {
    let hal_handle : &mut hal_ll_spi_master_handle_register_t = handle;
    let hal_ll_spi_hw_specifics_map_local: &mut hal_ll_spi_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_spi_hw_specifics_map_local.module_index as usize;
    let mut dummy: u32 = 0;

    if hal_handle.spi_master_handle != 0 {
        *hal_handle = hal_ll_spi_master_handle_register_t::default();

        hal_ll_spi_hw_specifics_map_local.mode = SPI_MASTER_MODE_DEFAULT;
        hal_ll_spi_hw_specifics_map_local.speed = HAL_LL_SPI_MASTER_SPEED_100K;
        hal_ll_spi_hw_specifics_map_local.dummy_data = 0;
        hal_ll_spi_hw_specifics_map_local.hw_actual_speed = 0;


        hal_ll_spi_master_set_clock(hal_ll_spi_hw_specifics_map_local, true, &mut dummy);
        hal_ll_spi_master_alternate_functions_set_state( hal_ll_spi_hw_specifics_map_local, false );
        hal_ll_spi_master_set_clock(hal_ll_spi_hw_specifics_map_local, false, &mut dummy);

        hal_ll_spi_hw_specifics_map_local.pins.pin_sck.pin_name = HAL_LL_PIN_NC;
        hal_ll_spi_hw_specifics_map_local.pins.pin_miso.pin_name = HAL_LL_PIN_NC;
        hal_ll_spi_hw_specifics_map_local.pins.pin_mosi.pin_name = HAL_LL_PIN_NC;
        hal_ll_spi_hw_specifics_map_local.pins.pin_sck.pin_af = 0;
        hal_ll_spi_hw_specifics_map_local.pins.pin_miso.pin_af = 0;
        hal_ll_spi_hw_specifics_map_local.pins.pin_mosi.pin_af = 0;

        unsafe{hal_ll_module_state[pin_check_result as usize] = *hal_handle;}
    }
}

///////// private functions

fn hal_ll_spi_master_transfer_bare_metal(spi_ptr: *mut hal_ll_spi_master_base_handle_t, data: u8) -> u8 {
    unsafe{
        (*spi_ptr).dr.data_byte = data;
    
        while check_reg_bit(&(*spi_ptr).sr as *const u32 as u32, HAL_LL_SPI_MASTER_RXNE) == 0 {()}

        (*spi_ptr).dr.data_byte
    }
}

fn hal_ll_spi_master_write_bare_metal( map: &mut hal_ll_spi_hw_specifics_map_t, write_data_buf: &mut [u8], len_write_data: usize) {
    let spi_ptr : *mut hal_ll_spi_master_base_handle_t = map.base as *mut hal_ll_spi_master_base_handle_t;

    // Write the first data to be transmitted into the SPI_DR register.
    for transfer_counter in  0 .. len_write_data {
        // If we are good to go ( if the tx buffer value has been shifted to the shift register ), write the data.
        hal_ll_spi_master_transfer_bare_metal( spi_ptr, write_data_buf[transfer_counter]);
    }
}

fn hal_ll_spi_master_read_bare_metal(map: &mut hal_ll_spi_hw_specifics_map_t, read_data_buf: &mut [u8], len_read_data: usize) {
    let spi_ptr : *mut hal_ll_spi_master_base_handle_t = map.base as *mut hal_ll_spi_master_base_handle_t;

    // Read the first data to be transmitted into the SPI_DR register.
    for transfer_counter in  0 .. len_read_data {
        // If we are good to go ( if the value from shift register has been shifted to the rx register ), read the data.
        read_data_buf[transfer_counter] = hal_ll_spi_master_transfer_bare_metal( spi_ptr, map.dummy_data);
    }
}

fn hal_ll_spi_master_check_pins(sck: hal_ll_pin_name_t, miso: hal_ll_pin_name_t, mosi: hal_ll_pin_name_t, index_list: &mut hal_ll_spi_pin_id) -> u8 {
    let sck_map_size: u8 = _spi_sck_map.len() as u8 ;
    let miso_map_size: u8 = _spi_miso_map.len() as u8 ;
    let mosi_map_size: u8 = _spi_mosi_map.len() as u8 ;
    
    let mut index_counter: u8 = 0;
    let mut hal_ll_module_id: u8 = 0;

    if HAL_LL_PIN_NC == sck || HAL_LL_PIN_NC == miso || HAL_LL_PIN_NC == mosi {
        return HAL_LL_PIN_NC;
    }
    
    for sck_index in 0x00 .. sck_map_size {
        if _spi_sck_map[sck_index as usize].pin == sck {
            for miso_index in 0x00 .. miso_map_size {
                if _spi_miso_map[miso_index as usize].pin == miso {
                    if _spi_miso_map[miso_index as usize].module_index == _spi_sck_map[sck_index as usize].module_index {
                        for mosi_index in 0x00 .. mosi_map_size {
                            if _spi_mosi_map[mosi_index as usize].pin == mosi {
                                if _spi_mosi_map[mosi_index as usize].module_index == _spi_sck_map[sck_index as usize].module_index {
                                    // Get module number
                                    hal_ll_module_id = _spi_sck_map[sck_index as usize].module_index;
                                    // Map pin names
                                    index_list.pin_sck = sck_index;
                                    index_list.pin_miso = miso_index;
                                    index_list.pin_mosi = mosi_index;

                                    if unsafe{hal_ll_module_state[ hal_ll_module_id as usize].spi_master_handle} == hal_ll_spi_master_handle_register_t::default().spi_master_handle {
                                        return hal_ll_module_id;
                                    } else {
                                        index_counter = index_counter + 1;
                                        if SPI_MODULE_COUNT == index_counter {
                                            return index_counter - 1;
                                        }
                                    }
                                }
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
fn hal_ll_get_specifics<'a>(handle: hal_ll_spi_master_handle_register_t) -> &'a mut hal_ll_spi_hw_specifics_map_t {
    let mut hal_ll_module_count: usize = SPI_MODULE_COUNT as usize;
    let mut hal_ll_module_error : usize = 0;
    hal_ll_module_error = hal_ll_module_count;
    
    unsafe{
        while hal_ll_module_count > 0 {
            hal_ll_module_count -= 1;

            let base: u32 = handle.spi_master_handle;

            if base == hal_ll_spi_hw_specifics_map[hal_ll_module_count].base {
                return &mut hal_ll_spi_hw_specifics_map[hal_ll_module_count];
            }
        }

        return &mut hal_ll_spi_hw_specifics_map[hal_ll_module_error];
    }
}


fn hal_ll_spi_master_map_pins(module_index: usize, index_list: &mut hal_ll_spi_pin_id) {
    unsafe{
        // Map new pins
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_sck.pin_name = _spi_sck_map[ index_list.pin_sck as usize].pin;
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_mosi.pin_name = _spi_mosi_map[ index_list.pin_mosi as usize].pin;
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_miso.pin_name = _spi_miso_map[ index_list.pin_miso as usize].pin;
        // SCL and SDA could have different alternate function settings, hence save both AF values
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_sck.pin_af = _spi_sck_map[ index_list.pin_sck as usize].af;
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_mosi.pin_af = _spi_mosi_map[ index_list.pin_mosi as usize].af;
        hal_ll_spi_hw_specifics_map[module_index as usize].pins.pin_miso.pin_af = _spi_miso_map[ index_list.pin_miso as usize].af;
    }
}

fn hal_ll_spi_master_set_clock(map: &mut hal_ll_spi_hw_specifics_map_t, hal_ll_state: bool, clock_value: &mut u32) {
    let mut rcc_clocks : RCC_ClocksTypeDef = RCC_ClocksTypeDef::default();
    rcc_get_clocks_frequency( &mut rcc_clocks );
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        #[cfg(feature = "spi1")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_1 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI1EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI1EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK2_Frequency;
        }

        #[cfg(feature = "spi2")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_2 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_SPI2EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_SPI2EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK1_Frequency;
        }

        #[cfg(feature = "spi3")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_3 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_SPI3EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_SPI3EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK1_Frequency;
        }

        #[cfg(feature = "spi4")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_4 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI4EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI4EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK2_Frequency;
        }

        #[cfg(feature = "spi5")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_5 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI5EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI5EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK2_Frequency;
        }

        #[cfg(feature = "spi6")]
        if map.module_index == hal_ll_spi_master_module_num(spi_modules::SPI_MODULE_6 as u8){
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI6EN_BIT );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_SPI6EN_BIT );
            }
            *clock_value = rcc_clocks.PCLK2_Frequency;
        }
    }
}

fn hal_ll_spi_master_alternate_functions_set_state(map: &mut hal_ll_spi_hw_specifics_map_t, hal_ll_state: bool) {
    let mut module: module_struct = module_struct::default();

    if ((*map).pins.pin_sck.pin_name != HAL_LL_PIN_NC) && ((*map).pins.pin_miso.pin_name != HAL_LL_PIN_NC) && ((*map).pins.pin_mosi.pin_name != HAL_LL_PIN_NC)  {
        module.pins[0] = VALUE( (*map).pins.pin_sck.pin_name, (*map).pins.pin_sck.pin_af );
        module.pins[1] = VALUE( (*map).pins.pin_miso.pin_name, (*map).pins.pin_miso.pin_af );
        module.pins[2] = VALUE( (*map).pins.pin_mosi.pin_name, (*map).pins.pin_mosi.pin_af );
        

        module.configs[0] = HAL_LL_SPI_CONFIG;
        module.configs[1] = HAL_LL_SPI_CONFIG;
        module.configs[2] = HAL_LL_SPI_CONFIG;

        // /* STM32F1xx specific */
        module.gpio_remap = (*map).pins.pin_sck.pin_af;

        hal_ll_gpio_module_struct_init( &mut module, hal_ll_state );
    }

}

fn hal_ll_spi_master_get_speed(clock_value : u32, speed: u32) -> u32{
    if speed >= (clock_value/2) {
        return 0;
    }

    if (speed <= (clock_value/2)) && (speed > (clock_value/4)) {
        return 0;
    }

    if (speed <= (clock_value/4)) && (speed > (clock_value/8)) {
        return 0x1;
    }

    if (speed <= (clock_value/8)) && (speed > (clock_value/16)) {
        return 0x2;
    }

    if (speed <= (clock_value/16)) && (speed > (clock_value/32)) {
        return 0x3;
    }

    if (speed <= (clock_value/32)) && (speed > (clock_value/64)) {
        return 0x4;
    }

    if (speed <= (clock_value/64)) && (speed > (clock_value/128)) {
        return 0x5;
    }

    if (speed <= (clock_value/128)) && (speed > (clock_value/256)) {
        return 0x6;
    }

    0x7
}

fn hal_ll_spi_master_get_actual_speed(clock_value : u32, divider: u32) -> u32{
    if divider == 0 {
        return clock_value/2;
    }

    if divider == 0x1 {
        return clock_value/4;
    }

    if divider == 0x2 {
        return clock_value/8;
    }

    if divider == 0x3 {
        return clock_value/16;
    }

    if divider == 0x4 {
        return clock_value/32;
    }

    if divider == 0x5 {
        return clock_value/64;
    }

    if divider == 0x6 {
        return clock_value/128;
    }
    
    clock_value/256
}

fn hal_ll_spi_master_hw_init(map: &mut hal_ll_spi_hw_specifics_map_t, clock_value: &mut u32) {
    let spi_ptr : *mut hal_ll_spi_master_base_handle_t = map.base as *mut hal_ll_spi_master_base_handle_t;
    let mut temp_speed: u32 = 0;

    unsafe{
        (*spi_ptr).cr1 = 0;
        temp_speed = hal_ll_spi_master_get_speed(*clock_value, map.speed);
        (*spi_ptr).cr1 |= temp_speed << HAL_LL_SPI_MASTER_BR_CONTROL;

        if map.mode as u8 <= hal_ll_spi_master_mode_t::SPI_MASTER_MODE_1 as u8 {
            clear_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_CLK_POLARITY);
        } else {
            set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_CLK_POLARITY);
        }

        if (map.mode == hal_ll_spi_master_mode_t::SPI_MASTER_MODE_0) || (map.mode == hal_ll_spi_master_mode_t::SPI_MASTER_MODE_2) {
            clear_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_CLK_PHASE);
        } else {
            set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_CLK_PHASE);
        }

        // Set default values for SPI Master configuration.
        set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_SSM_ENABLE);
        set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_SSI_1);
        set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_SELECTION);

        set_reg_bit(&(*spi_ptr).cr2 as *const u32 as u32, HAL_LL_SPI_MASTER_SSOE_BIT);

        (*spi_ptr).cr2 |= HAL_LL_SPI_MASTER_FRXTH;


        set_reg_bit(&(*spi_ptr).cr1 as *const u32 as u32, HAL_LL_SPI_MASTER_ENABLE);


        let tmp_actual_speed: u32 = hal_ll_spi_master_get_actual_speed(*clock_value, temp_speed);
    }

}

fn hal_ll_spi_init(map: &mut hal_ll_spi_hw_specifics_map_t) {
    let mut clock_value:u32 = 0;
    hal_ll_spi_master_set_clock(map, true, &mut clock_value);
    hal_ll_spi_master_alternate_functions_set_state(map, true);
    hal_ll_spi_master_hw_init(map, &mut clock_value);
}