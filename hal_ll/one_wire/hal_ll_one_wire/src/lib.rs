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


use hal_ll_gpio::*;
use hal_ll_gpio::pin_names::*;
use system::Delay_us;
use core::fmt;

const RESET_PINS_OFFSET : u8 = 16;
const HAL_LL_ONE_WIRE_MINIMUM_BITS_PER_TRANSFER : u8 = 8;

const ONE_WIRE_CMD_ROM_READ : u8 = 0x33;
const ONE_WIRE_CMD_ROM_SKIP : u8 = 0xCC;
const ONE_WIRE_CMD_ROM_MATCH : u8 = 0x55;
const ONE_WIRE_CMD_ROM_SEARCH : u8 = 0xF0;
const ONE_WIRE_CMD_ROM_READ_LEGACY : u8 = 0x0F;

#[derive(Debug)]
pub enum HAL_LL_ONE_WIRE_ERROR {
    ONE_WIRE_WRONG_PINS,
    ACQUIRE_FAIL,
    ONE_WIRE_ERROR
}

impl fmt::Display for HAL_LL_ONE_WIRE_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ONE_WIRE_WRONG_PINS => write!(f, "ONE_WIRE_WRONG_PINS occurred"),
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),                    
            Self::ONE_WIRE_ERROR => write!(f, "ONE_WIRE_ERROR occurred"),                    
        }
    }
}


type Result<T> = core::result::Result<T, HAL_LL_ONE_WIRE_ERROR>;

pub struct hal_ll_one_wire_rom_address_t {
    pub address : [u8; 8],
}

pub struct hal_ll_one_wire_t {
    pub data_pin : hal_ll_pin_name_t,
    pub state : bool,
}

impl Default for hal_ll_one_wire_t {
    fn default() -> Self {
        Self { data_pin: HAL_LL_PIN_NC, state: false }
    }
}

pub struct hal_ll_one_wire_local_t {
    pub data_pin : hal_ll_pin_name_t, // One Wire data pin.
    pub moder : u32, // Register for altering GPIO pin direction.
    pub moder_set : u32, // Variable for configuring pin as General purpose output.
    pub moder_clear : u32, // Variable for configuring pin as General purpose input (reset state).
    pub bsrr : u32, // Register for GPIO port bit set/reset.
    pub idr : u32, // Register for reading current GPIO pin state.
}

impl Default for hal_ll_one_wire_local_t {
    fn default() -> Self {
        Self {
            data_pin: HAL_LL_PIN_NC,
            moder: 0,
            moder_set: 0,
            moder_clear: 0,
            bsrr: 0,
            idr: 0,
        }
    }
}


static mut one_wire_handle : hal_ll_one_wire_local_t = hal_ll_one_wire_local_t{
            data_pin: HAL_LL_PIN_NC,
            moder: 0,
            moder_set: 0,
            moder_clear: 0,
            bsrr: 0,
            idr: 0,
        };

static mut last_device_flag : u8 = 0;
static mut last_discrepancy : u8 = 0;
static mut last_family_discrepancy : u8 = 0;


pub fn hal_ll_one_wire_open( obj : &mut hal_ll_one_wire_t ) {
    // Local instance of One Wire pin.
    let mut one_wire_pin : hal_ll_gpio_pin_t = hal_ll_gpio_pin_t::default();

    // Enable appropriate PORT clock, set pin to be digital input.
    hal_ll_gpio_configure_pin( &mut one_wire_pin, obj.data_pin, hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT );

    /* Register One Wire handle (memorize appropriate GPIO registers and bit positions). */
    hal_ll_one_wire_reconfigure( obj );
}

pub fn hal_ll_one_wire_reset() -> u8 {
    let mut device_response: u8 = 1;

    unsafe {
        *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
        *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask(one_wire_handle.data_pin) as u32) << RESET_PINS_OFFSET;
    }

    one_wire_timing_value_h();

    unsafe {
        *(one_wire_handle.moder as *mut u32) &= one_wire_handle.moder_clear;
    }

    one_wire_timing_value_i();

    unsafe {
        if *(one_wire_handle.idr as *mut u32) & hal_ll_gpio_pin_mask(one_wire_handle.data_pin) as u32  > 0 {
            device_response = 0x01;
        } else {
            device_response = 0x00;
        }
    }

    one_wire_timing_value_j();

    device_response
}

pub fn hal_ll_one_wire_read_rom(device_rom_address : &mut hal_ll_one_wire_rom_address_t) -> Result<()> {
    if hal_ll_one_wire_reset() > 0 {
        return Err(HAL_LL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
    }

    let read_command : [u8; 1 ] = [ONE_WIRE_CMD_ROM_READ];

    hal_ll_one_wire_write_byte(&read_command, 1);

    hal_ll_one_wire_read_byte(&mut device_rom_address.address, 8);

    Ok(())
}

pub fn hal_ll_one_wire_skip_rom() -> Result<()> {
    if hal_ll_one_wire_reset() > 0 {
        return Err(HAL_LL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
    }

    let read_command : [u8; 1 ] = [ONE_WIRE_CMD_ROM_SKIP];

    hal_ll_one_wire_write_byte(&read_command, 1);

    Ok(())
}

pub fn hal_ll_one_wire_match_rom(device_rom_address : &mut hal_ll_one_wire_rom_address_t) -> Result<()> {
    if hal_ll_one_wire_reset() > 0 {
        return Err(HAL_LL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
    }

    let read_command : [u8; 1 ] = [ONE_WIRE_CMD_ROM_MATCH];

    hal_ll_one_wire_write_byte(&read_command, 1);

    hal_ll_one_wire_read_byte(&mut device_rom_address.address, 8);

    Ok(())
}

pub fn hal_ll_one_wire_search_first_device(obj : &mut hal_ll_one_wire_t, one_wire_device_list : &mut hal_ll_one_wire_rom_address_t) -> Result<()> {
    unsafe {
        last_discrepancy = 0;
        last_device_flag = 0;
        last_family_discrepancy = 0;
    }

    hal_ll_one_wire_search( obj, one_wire_device_list )?;
    Ok(())
}

pub fn hal_ll_one_wire_search_next_device(obj : &mut hal_ll_one_wire_t, one_wire_device_list : &mut hal_ll_one_wire_rom_address_t) -> Result<()> {
    hal_ll_one_wire_search( obj, one_wire_device_list )?;
    Ok(())
}

fn hal_ll_one_wire_search(obj : &mut hal_ll_one_wire_t, one_wire_device_list : &mut hal_ll_one_wire_rom_address_t) -> Result<u8> {
    let mut search_result: u8 = 0;
    
    let mut rom_byte_number: u8 = 0;
    let mut rom_byte_mask: u8 = 0;
    let mut id_bit_number: u8 = 0;
    let mut last_zero: u8 = 0;

    let mut id_bit: u8 = 0;

    let mut cmp_id_bit: u8 = 0;
    
    let mut search_direction: u8 = 0;

    unsafe {
        if last_device_flag == 0 {
            if hal_ll_one_wire_reset() > 0 {
                last_discrepancy = 0;
                last_device_flag = 0;
                last_family_discrepancy = 0;

                return Err(HAL_LL_ONE_WIRE_ERROR::ONE_WIRE_ERROR);
            }
        

            let read_command : [u8; 1 ] = [ONE_WIRE_CMD_ROM_SEARCH];

            hal_ll_one_wire_write_byte(&read_command, 1);  

            loop {
                hal_ll_one_wire_read_bit(&mut id_bit);
                hal_ll_one_wire_read_bit(&mut cmp_id_bit);

                if (id_bit == 1) && (cmp_id_bit == 1) {
                    break;
                }

                if id_bit != cmp_id_bit {
                    search_direction = id_bit;
                } else {
                    if id_bit_number < last_discrepancy {
                        search_direction = (one_wire_device_list.address[rom_byte_number as usize] & rom_byte_mask > 0) as u8;
                    } else {
                        search_direction = (id_bit_number == last_discrepancy) as u8;
                    }

                    if search_direction == 0 {
                        last_zero = id_bit_number;

                        if last_zero < 9  {
                                last_family_discrepancy = last_zero;
                        }
                    }
                }

                if search_direction == 1 {
                    one_wire_device_list.address[rom_byte_number as usize] |= rom_byte_mask;
                } else {
                    one_wire_device_list.address[rom_byte_number as usize] &= !rom_byte_mask;
                }
                
                hal_ll_one_wire_write_bit( search_direction );

                id_bit_number += 1;
                rom_byte_mask <<= 1;

                if rom_byte_mask == 0 {
                    rom_byte_number += 1;
                    rom_byte_mask = 1;
                }


                if rom_byte_number >= HAL_LL_ONE_WIRE_MINIMUM_BITS_PER_TRANSFER {
                    break;
                }

            }

            if id_bit_number >= 65 {
                last_discrepancy = last_zero;

                if last_discrepancy == 0 {
                    last_device_flag = 1;
                }

                search_result = 1;
            }
        }

        if (search_result == 0) || one_wire_device_list.address[0] == 0 {
            last_discrepancy = 0;
            last_family_discrepancy = 0;
            last_device_flag = 0;
            search_result = 0;
        }

    }
    
    Ok(search_result)
}


pub fn hal_ll_one_wire_write_byte(write_data_buffer : &[u8], write_data_length : usize) {
    let mut local_byte_checker: usize = 0;
    let mut local_bit_checker: u8;
    let mut local_byte: u8;
    let mut bit_state: u8;

    while local_byte_checker != write_data_length {
        local_bit_checker = 0;
        local_byte = write_data_buffer[local_byte_checker];
        local_byte_checker += 1;

        while local_bit_checker != HAL_LL_ONE_WIRE_MINIMUM_BITS_PER_TRANSFER {
            bit_state = (local_byte >> local_bit_checker) & 1;
            local_bit_checker += 1;

            if bit_state > 0 {
                unsafe {
                    *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
                    *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;
                
                    one_wire_timing_value_a();

                    *(one_wire_handle.moder as *mut u32) &= one_wire_handle.moder_clear;

                    one_wire_timing_value_b();
                }
            } else {
                unsafe {
                    *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
                    *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;
                    
                    one_wire_timing_value_c();

                    *(one_wire_handle.moder as *mut u32) &= one_wire_handle.moder_clear;

                    one_wire_timing_value_d();
                }
            }
        }
    }
}

pub fn hal_ll_one_wire_read_byte(read_data_buffer : &mut [u8], read_data_length : usize) {
    let mut local_byte_checker: usize = 0;
    let mut local_bit_checker: u8;
    let mut local_buffer: u8;

    while local_byte_checker != read_data_length {
        local_bit_checker = 0;
        local_buffer = 0;

        while local_bit_checker != HAL_LL_ONE_WIRE_MINIMUM_BITS_PER_TRANSFER {
            unsafe {
                *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
                *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;
            
                one_wire_timing_value_a();

                *(one_wire_handle.moder as *mut u32) &= one_wire_handle.moder_clear;

                one_wire_timing_value_e();


                if *(one_wire_handle.idr as *mut u32) & hal_ll_gpio_pin_mask(one_wire_handle.data_pin) as u32 > 0 {
                    local_buffer += 1 << local_bit_checker;
                }

                local_bit_checker += 1;

                one_wire_timing_value_f();
            }
        }

        read_data_buffer[local_byte_checker] = local_buffer;
        local_byte_checker += 1;
    }
}


pub fn hal_ll_one_wire_reconfigure(obj : &mut hal_ll_one_wire_t) {
    // Local instance of One Wire pin.
    let mut one_wire_pin : hal_ll_gpio_pin_t = hal_ll_gpio_pin_t::default();
    one_wire_pin.base = hal_ll_gpio_port_base(hal_ll_gpio_port_index(obj.data_pin));
    one_wire_pin.mask = hal_ll_gpio_pin_mask(obj.data_pin);

    unsafe {
        one_wire_handle.data_pin = obj.data_pin % PORT_SIZE;
        
        let gpio_ptr : *mut hal_ll_gpio_base_handle_t = one_wire_pin.base as *mut hal_ll_gpio_base_handle_t;

        one_wire_handle.moder = &(*gpio_ptr).moder as *const u32 as u32;

        one_wire_handle.idr = &(*gpio_ptr).idr as *const u32 as u32;

        one_wire_handle.moder_set = 0x1 << (one_wire_handle.data_pin * 2);

        one_wire_handle.moder_clear = !(0x3 << (one_wire_handle.data_pin * 2));

        one_wire_handle.bsrr = &(*gpio_ptr).bsrr as *const u32 as u32;

        obj.state = true;
    }
}

fn hal_ll_one_wire_write_bit(write_data_buffer : u8) {
    let bit_state : u8 = write_data_buffer & 0x1;

    unsafe {
        if bit_state > 0 {
            *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
            *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;

            one_wire_timing_value_a();
            *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_clear;
            
            one_wire_timing_value_b();
        } else {
            *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
            *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;

            one_wire_timing_value_c();
            *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_clear;
            
            one_wire_timing_value_d();
        }
    }


}

fn hal_ll_one_wire_read_bit(read_data_buffer : &mut u8) {
    unsafe {
        *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_set;
        *(one_wire_handle.bsrr as *mut u32) = (hal_ll_gpio_pin_mask( one_wire_handle.data_pin ) as u32) << RESET_PINS_OFFSET;
        
        one_wire_timing_value_a();
        *(one_wire_handle.moder as *mut u32) |= one_wire_handle.moder_clear;

        one_wire_timing_value_e();

        if *(one_wire_handle.idr as *mut u32) & hal_ll_gpio_pin_mask(one_wire_handle.data_pin) as u32 > 0 {
            *read_data_buffer = 1;
        } else {
            *read_data_buffer = 0;
        }

        one_wire_timing_value_f();
    }
   
}


#[inline(always)]
pub fn one_wire_timing_value_a() {
    Delay_us(5);
}
#[inline(always)]
pub fn one_wire_timing_value_b() {
    Delay_us(64);
}
#[inline(always)]
pub fn one_wire_timing_value_c() {
    Delay_us(60);
}
#[inline(always)]
pub fn one_wire_timing_value_d() {
    Delay_us(6);
}
#[inline(always)]
pub fn one_wire_timing_value_e() {
    Delay_us(6);
}
#[inline(always)]
pub fn one_wire_timing_value_f() {
    Delay_us(50);
}
#[inline(always)]
pub fn one_wire_timing_value_h() {
    Delay_us(480);
}
#[inline(always)]
pub fn one_wire_timing_value_i() {
    Delay_us(70);
}
#[inline(always)]
pub fn one_wire_timing_value_j() {
    Delay_us(410);
}