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
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

//to change place at some point
//==========================================================//
pub use hal_ll_target::pin_names;
use hal_ll_target::*;
use hal_ll_gpio_constants::*;
pub mod gpio_constants {
    pub use hal_ll_gpio_constants::{
        GPIO_CFG_MODE_ALT_FUNCTION,
        GPIO_CFG_SPEED_LOW,
        GPIO_CFG_SPEED_HIGH,
        GPIO_CFG_OTYPE_OD,
        GPIO_CFG_OTYPE_PP,
    };
}

//==========================================================//

pub const RESET_PINS_OFFSET: u8 = 16;

pub const GPIO_MODULE_STRUCT_END: u32 = 0xFFFF_FFFF;
pub const GPIO_AF_OFFSET: u8 = 8;

pub type hal_ll_gpio_base_t = handle_t;

pub struct module_struct
{
  pub pins: [u32; 13],
  pub configs: [u32; 13],
  pub gpio_remap: u32
}

impl Default for module_struct {
    fn default() -> Self {
        Self { pins: [GPIO_MODULE_STRUCT_END; 13], configs: [GPIO_MODULE_STRUCT_END; 13], gpio_remap: 0 }
    }
}

pub struct hal_ll_gpio_t
{
    pub base: hal_ll_gpio_base_t,
    pub mask: hal_ll_gpio_mask_t
}

impl Default for hal_ll_gpio_port_t {
    fn default() -> Self {
        Self { base: 0, mask: 0}
    }
}

#[derive(PartialEq)]
pub enum hal_ll_gpio_direction_t
{
    HAL_LL_GPIO_DIGITAL_INPUT = 0,
    HAL_LL_GPIO_DIGITAL_OUTPUT = 1
}

pub type hal_ll_gpio_pin_t = hal_ll_gpio_t;
pub type hal_ll_gpio_port_t = hal_ll_gpio_t;

const _hal_ll_gpio_port_base: [u32; 11] =
[
    #[cfg(feature = "gpioa")]
    GPIOA_BASE_ADDR,
    #[cfg(not(feature = "gpioa"))]
    0,
    #[cfg(feature = "gpiob")]
    GPIOB_BASE_ADDR,
    #[cfg(not(feature = "gpiob"))]
    0,
    #[cfg(feature = "gpioc")]
    GPIOC_BASE_ADDR,
    #[cfg(not(feature = "gpioc"))]
    0,
    #[cfg(feature = "gpiod")]
    GPIOD_BASE_ADDR,
    #[cfg(not(feature = "gpiod"))]
    0,
    #[cfg(feature = "gpioe")]
    GPIOE_BASE_ADDR,
    #[cfg(not(feature = "gpioe"))]
    0,
    #[cfg(feature = "gpiof")]
    GPIOF_BASE_ADDR,
    #[cfg(not(feature = "gpiof"))]
    0,
    #[cfg(feature = "gpiog")]
    GPIOG_BASE_ADDR,
    #[cfg(not(feature = "gpiog"))]
    0,
    #[cfg(feature = "gpioh")]
    GPIOH_BASE_ADDR,
    #[cfg(not(feature = "gpioh"))]
    0,
    #[cfg(feature = "gpioi")]
    GPIOI_BASE_ADDR,
    #[cfg(not(feature = "gpioi"))]
    0,
    #[cfg(feature = "gpioj")]
    GPIOJ_BASE_ADDR,
    #[cfg(not(feature = "gpioj"))]
    0,
    #[cfg(feature = "gpiok")]
    GPIOK_BASE_ADDR,
    #[cfg(not(feature = "gpiok"))]
    0
];

#[repr(C)]
pub struct hal_ll_gpio_base_handle_t 
{
    pub moder:   u32,
    pub otyper:  u32,
    pub ospeedr: u32,
    pub pupdr:   u32,
    pub idr:     u32,
    pub odr:     u32,
    pub bsrr:    u32,
    pub lckr:    u32,
    pub afrl:    u32,
    pub afrh:    u32,
}

const RCC_GPIOCLOCK: u32 = RCC_AHB1ENR;

fn hal_ll_gpio_pin_index(name: hal_ll_pin_name_t) -> u8
{
    name % PORT_SIZE
}

pub fn hal_ll_gpio_port_index(name: hal_ll_pin_name_t) -> u8
{
    name / PORT_SIZE
}

pub fn hal_ll_gpio_pin_mask(name: hal_ll_pin_name_t) -> u16
{
    0x01 << hal_ll_gpio_pin_index( name )
}

pub fn hal_ll_gpio_port_base( name: hal_ll_port_name_t ) -> u32
{
    _hal_ll_gpio_port_base[ name as usize ]
}

pub fn hal_ll_gpio_analog_input(port: u32, pin_mask: u16)
{
    hal_ll_gpio_config(port, pin_mask, GPIO_CFG_ANALOG_INPUT);
}

pub fn hal_ll_gpio_digital_input(port: u32, pin_mask: u16) 
{
    hal_ll_gpio_config(port, pin_mask, GPIO_CFG_DIGITAL_INPUT);
}

pub fn hal_ll_gpio_digital_output(port: u32, pin_mask: u16) 
{
    hal_ll_gpio_config(port, pin_mask, GPIO_CFG_DIGITAL_OUTPUT);
}

pub fn hal_ll_gpio_module_struct_init(module: &mut module_struct, state: bool)
{
    let mut index: i32 = 0;

    while  module.pins[ index as usize] != GPIO_MODULE_STRUCT_END 
    {
        _hal_ll_gpio_config_pin_alternate_enable(  module.pins[ index as usize],  module.configs[ index as usize], state );

        index = index + 1;
    }
}

//TODO : optimize per MCU
fn hal_ll_gpio_clock_enable(port: u32) {
    let mut pos = 0;

    unsafe 
    {

        match port & 0xFFFFFC00 {
            #[cfg(feature = "gpioa")]
            GPIOA_BASE_ADDR => {pos = 0x1;},
            #[cfg(feature = "gpiob")]
            GPIOB_BASE_ADDR => {pos = 0x2;},
            #[cfg(feature = "gpioc")]
            GPIOC_BASE_ADDR => {pos = 0x4;},
            #[cfg(feature = "gpiod")]
            GPIOD_BASE_ADDR => {pos = 0x8;},
            #[cfg(feature = "gpioe")]
            GPIOE_BASE_ADDR => {pos = 0x10;},
            #[cfg(feature = "gpiof")]
            GPIOF_BASE_ADDR => {pos = 0x20;},
            #[cfg(feature = "gpiog")]
            GPIOG_BASE_ADDR => {pos = 0x40;},
            #[cfg(feature = "gpioh")]
            GPIOH_BASE_ADDR => {pos = 0x80;},
            #[cfg(feature = "gpioi")]
            GPIOI_BASE_ADDR => {pos = 0x100;},
            #[cfg(feature = "gpioj")]
            GPIOJ_BASE_ADDR => {pos = 0x200;},
            #[cfg(feature = "gpiok")]
            GPIOK_BASE_ADDR => {pos = 0x400;},
            _ => {}
        }

        *(RCC_GPIOCLOCK as *mut u32) |= pos;
    }
}

//TODO : optimize per MCU, extra config tunnings
//only treated conf digital I/O any mask
fn hal_ll_gpio_config(port: u32, pin_mask: u16, config: u32) 
{
    let mut pos         : u32   = 0;
    let mut current_pin : u32   = 0;

    let mut mode        : u32   = 0;
    let mut speed       : u32   = 0;
    let mut otype       : u32   = 0;
    let mut pull        : u32   = 0;

    let port_ptr : *mut hal_ll_gpio_base_handle_t = port as *mut hal_ll_gpio_base_handle_t;
    
    hal_ll_gpio_clock_enable(port);
    

    if pin_mask == GPIO_PIN_MASK_LOW as u16 
    {
        unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_HIGH_32BIT; }
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe {
                (*port_ptr).moder |= 0x0000_5555;
                (*port_ptr).otyper &= 0xFFFF_FF00;
                (*port_ptr).ospeedr |= HAL_LL_NIBBLE_LOW_32BIT;
            }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }

    } else if pin_mask == GPIO_PIN_MASK_HIGH as u16
    {
        unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_LOW_32BIT; }
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe {
                (*port_ptr).moder |= 0x5555_0000;
                (*port_ptr).otyper &= 0xFFFF_00FF;
                (*port_ptr).ospeedr |= HAL_LL_NIBBLE_HIGH_32BIT; 
            }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_LOW_32BIT; }
            return;
        }

    }

    if pin_mask == GPIO_PIN_MASK_ALL as u16
    {
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe {
                (*port_ptr).moder = 0x5555_5555;
                (*port_ptr).otyper = 0;
                (*port_ptr).ospeedr = HAL_LL_NIBBLE_HIGH_32BIT;
            }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder = 0; }
            return;
        }
    }


    if config & GPIO_CFG_MODE_ANALOG != 0 {
        mode = 3;
    } else if config & GPIO_CFG_MODE_ALT_FUNCTION != 0 {
        mode = 2;
    } else if config & GPIO_CFG_MODE_OUTPUT != 0 {
        mode = 1;
    } else {
        mode = 0;
    }

    //TODO adapt per cpu
    if config & (GPIO_CFG_SPEED_VERY_HIGH | GPIO_CFG_SPEED_MAX) != 0 {
        speed = 3;
    } else if config & GPIO_CFG_SPEED_HIGH != 0 {
        speed = 2;
    } else if config & GPIO_CFG_SPEED_MEDIUM != 0 {
        speed = 1;
    } else {
        speed = 0;
    }

    if config & GPIO_CFG_OTYPE_OD != 0 {
        otype = 1;
    } else {
        otype = 0;
    }

    if config & GPIO_CFG_PULL_DOWN != 0 {
        pull = 2;
    } else if config & GPIO_CFG_PULL_UP != 0 {
        pull = 1;
    } else {
        pull = 0;
    }

    for pin_pos in 0x00 .. 0x10 {
        pos = 0x01 << pin_pos;
        current_pin = (pin_mask) as u32 & pos;

        if current_pin == pos
        {
            unsafe{
                (*port_ptr).moder &= !( GPIO_MODER_MODER0 << ( pin_pos * 2 ) );
                (*port_ptr).moder |=  ( mode ) << ( pin_pos * 2 );
            }

            if config & ( GPIO_CFG_MODE_OUTPUT | GPIO_CFG_MODE_ALT_FUNCTION) != 0
            {
                unsafe {                
                    (*port_ptr).ospeedr &= !( GPIO_OSPEEDER_OSPEEDR0 << ( pin_pos * 2 ) );
                    (*port_ptr).ospeedr |=  speed  << ( pin_pos * 2 );
                    
                    (*port_ptr).otyper &= !( GPIO_OTYPER_OT_0 << ( pin_pos ) as u16 ) ;
                    (*port_ptr).otyper |= otype << pin_pos;
                }
            }

            unsafe{
                (*port_ptr).pupdr &= !( GPIO_PUPDR_PUPDR0 << ( pin_pos * 2 ) );
                (*port_ptr).pupdr |=  pull  << ( pin_pos * 2 );
            }

        }

    }
}

pub fn _hal_ll_gpio_config_pin_alternate_enable(module_pin: u32, module_config: u32, state: bool)
{
    let pin_name: hal_ll_pin_name_t;
    let mut pin_index: u8;
    let alternate_function: u32;
    let port_ptr: *mut hal_ll_gpio_base_handle_t;

    pin_name  = (module_pin & GPIO_PIN_NAME_MASK) as u8;

    alternate_function = ( module_pin >> GPIO_AF_OFFSET ) & GPIO_AF_MASK;

    port_ptr = hal_ll_gpio_port_base( hal_ll_gpio_port_index ( pin_name ) ) as *mut hal_ll_gpio_base_handle_t;

    hal_ll_gpio_clock_enable( port_ptr as u32 );

    pin_index = hal_ll_gpio_pin_index( pin_name );
    unsafe{
        if ( pin_index > 7 )
        {
            pin_index -= 8;
            if ( state ) {
                (*port_ptr).afrh &= !(( GPIO_AF_MASK ) << ( pin_index * 4 ) );
                (*port_ptr).afrh |= (( alternate_function ) << ( pin_index * 4 ) );
            } else {
                (*port_ptr).afrh &= !(( alternate_function ) << ( pin_index * 4 ) );
            }
        }
        else
        {
            if ( state ) {
                (*port_ptr).afrl &= !(( GPIO_AF_MASK ) << ( pin_index * 4 ) );
                (*port_ptr).afrl |= (( alternate_function ) << ( pin_index * 4 ) );
            } else {
                (*port_ptr).afrl &= !(( alternate_function ) << ( pin_index * 4 ) );
            }
        }
    }

    hal_ll_gpio_config( port_ptr as u32, hal_ll_gpio_pin_mask( pin_name ), module_config );
}

pub fn VALUE(pin: hal_ll_pin_name_t, func: u32)  -> u32 {
    pin as u32 | (func << GPIO_AF_OFFSET)
}