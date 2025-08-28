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
#![allow(non_snake_case)]

use crate::target::*;
pub use mcu_definition::uart::*;
use crate::gpio::*;
use crate::gpio::gpio_constants::*;
use system::{rcc_get_clocks_frequency, RCC_ClocksTypeDef, RCC_TypeDef, RCC_BASE};
use interrupt::interrupt_helper::*;
use core::fmt;

const HAL_LL_UART_ACCEPTABLE_ERROR : f32 = 1.0;

#[cfg(all(feature = "uart1", any(feature = "f7")))]
const HAL_LL_USART1EN : u32 =  4;
#[cfg(feature = "uart2")]
const HAL_LL_USART2EN : u32 = 17;
#[cfg(feature = "uart3")]
const HAL_LL_USART3EN : u32 = 18;
#[cfg(feature = "uart4")]
const HAL_LL_UART4EN  : u32 = 19;
#[cfg(feature = "uart5")]
const HAL_LL_UART5EN  : u32 = 20;
#[cfg(feature = "uart6")]
const HAL_LL_USART6EN : u32 =  5;
#[cfg(all(feature = "uart7", any(feature = "f7")))]
const HAL_LL_UART7EN  : u32 = 30;
#[cfg(all(feature = "uart8", any(feature = "f7")))]
const HAL_LL_UART8EN  : u32 = 31;

#[cfg(feature = "uart1")]
pub const uart_index_1: usize = 0;
#[cfg(feature = "uart2")]
pub const uart_index_2: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 };
#[cfg(feature = "uart3")]
pub const uart_index_3: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 };
#[cfg(feature = "uart4")]
pub const uart_index_4: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 };
#[cfg(feature = "uart5")]
pub const uart_index_5: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 };
#[cfg(feature = "uart6")]
pub const uart_index_6: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 }
    + if cfg!(feature = "uart5") { 1 } else { 0 };
#[cfg(feature = "uart7")]
pub const uart_index_7: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 }
    + if cfg!(feature = "uart5") { 1 } else { 0 }
    + if cfg!(feature = "uart6") { 1 } else { 0 };
#[cfg(feature = "uart8")]
pub const uart_index_8: usize = 0
    + if cfg!(feature = "uart1") { 1 } else { 0 }
    + if cfg!(feature = "uart2") { 1 } else { 0 }
    + if cfg!(feature = "uart3") { 1 } else { 0 }
    + if cfg!(feature = "uart4") { 1 } else { 0 }
    + if cfg!(feature = "uart5") { 1 } else { 0 }
    + if cfg!(feature = "uart6") { 1 } else { 0 }
    + if cfg!(feature = "uart7") { 1 } else { 0 };

// const HAL_LL_UART_IT_CLEAR_MASK : u32 = 0x360;
const HAL_LL_UART_IT_CR1_MASK : u32 = 0x1F0;
const HAL_LL_UART_IT_CR2_MASK : u32 = 0x40;
const HAL_LL_UART_IT_CR3_MASK : u32 = 0x401;

const DATA_BIT_8 : u32 = 0x0;
const DATA_BIT_9 : u32 = 0x1000;

const PARITY_NO : u32 = 0x0;
const PARITY_ENABLED : u32 = 0x400;
const PARITY_EVEN : u32 = 0x0;
const PARITY_ODD : u32 = 0x200;

const STOP_BITS_HALF : u32 = 0x1000;
const STOP_BITS_ONE : u32 = 0x0;
const STOP_BITS_ONE_AND_A_HALF : u32 = 0x3000;
const STOP_BITS_TWO : u32 = 0x2000;

const HAL_LL_UART_CR1_UE : u32 = 13;
const HAL_LL_UART_CR1_TE : u32 = 3;
const HAL_LL_UART_CR1_RE : u32 = 2;

const HAL_LL_UART_STATUS_RXNE_FLAG : u32 =  1 << 5;
const HAL_LL_UART_STATUS_TXE_FLAG  : u32 = 1 << 7;

// const HAL_LL_UART_IT_PE : u32 = 0x10000100;
const HAL_LL_UART_IT_TXE : u32 = 7;
// const HAL_LL_UART_IT_TC : u32 = 0x10000040;
const HAL_LL_UART_IT_RXNE : u32 = 5;
// const HAL_LL_UART_IT_IDLE : u32 = 0x10000010;

const HAL_LL_UART_AF_CONFIG : u32 = GPIO_CFG_MODE_ALT_FUNCTION | GPIO_CFG_SPEED_HIGH | GPIO_CFG_OTYPE_PP;

pub type hal_ll_uart_isr_t = fn(handle : &mut hal_ll_uart_handle_register_t, event : hal_ll_uart_irq_t );

#[derive(Debug)]
pub enum HAL_LL_UART_ERROR {
    UART_WRONG_PINS,
    MODULE_ERROR,
    ACQUIRE_FAIL,
    UART_ERROR,
}

impl fmt::Display for HAL_LL_UART_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UART_WRONG_PINS => write!(f, "UART_WRONG_PINS occurred"),
            Self::MODULE_ERROR => write!(f, "MODULE_ERROR occurred"),             
            Self::ACQUIRE_FAIL => write!(f, "ACQUIRE_FAIL occurred"),                  
            Self::UART_ERROR => write!(f, "UART_ERROR occurred"),                  
        }
    }
}

type Result<T> = core::result::Result<T, HAL_LL_UART_ERROR>;

#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_uart_irq_t
{
    HAL_LL_UART_IRQ_RX,  /* RX INT   */
    HAL_LL_UART_IRQ_TX,  /* TX INT   */
}

#[derive(Clone, Copy)]
pub struct hal_ll_uart_pin_id {
    pub pin_tx : u8,
    pub pin_rx : u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct hal_ll_uart_handle_register_t
{
    pub uart_handle : handle_t,
    pub init_ll_state: bool
}

impl Default for hal_ll_uart_handle_register_t {
    fn default() -> Self {
        Self {
            uart_handle : 0,
            init_ll_state : false
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_uart_parity_t
{
    HAL_LL_UART_PARITY_NONE,
    HAL_LL_UART_PARITY_EVEN,
    HAL_LL_UART_PARITY_ODD,
}

pub const HAL_LL_UART_PARITY_DEFAULT : hal_ll_uart_parity_t = hal_ll_uart_parity_t::HAL_LL_UART_PARITY_NONE;

#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_uart_stop_bits_t
{
    HAL_LL_UART_STOP_BITS_HALF,
    HAL_LL_UART_STOP_BITS_ONE,
    HAL_LL_UART_STOP_BITS_ONE_AND_A_HALF,
    HAL_LL_UART_STOP_BITS_TWO,
}

pub const HAL_LL_UART_STOP_BITS_DEFAULT : hal_ll_uart_stop_bits_t = hal_ll_uart_stop_bits_t::HAL_LL_UART_STOP_BITS_ONE;

#[derive(Clone, Copy, PartialEq)]
pub enum hal_ll_uart_data_bits_t
{
    HAL_LL_UART_DATA_BITS_7,
    HAL_LL_UART_DATA_BITS_8,
    HAL_LL_UART_DATA_BITS_9,
}

pub const HAL_LL_UART_DATA_BITS_DEFAULT : hal_ll_uart_data_bits_t = hal_ll_uart_data_bits_t::HAL_LL_UART_DATA_BITS_8;


#[repr(C)]
struct hal_ll_uart_base_handle_t
{
    pub cr1: hal_ll_base_addr_t,
    pub cr2: hal_ll_base_addr_t,
    pub cr3: hal_ll_base_addr_t,
    pub brr: hal_ll_base_addr_t,
    pub gtpr: hal_ll_base_addr_t,
    pub rtor: hal_ll_base_addr_t,
    pub rqr: hal_ll_base_addr_t,
    pub isr: hal_ll_base_addr_t,
    pub icr: hal_ll_base_addr_t,
    pub rdr: hal_ll_base_addr_t,
    pub tdr: hal_ll_base_addr_t,
}

pub struct hal_ll_uart_baud_t
{
    pub baud : u32,
    pub real_baud : u32,
}

struct hal_ll_uart_pins_t {
    pub pin_tx: hal_ll_pin_af_t,
    pub pin_rx: hal_ll_pin_af_t,
}

struct hal_ll_uart_hw_specifics_map_t
{
    pub base : hal_ll_base_addr_t,
    pub module_index : hal_ll_pin_name_t,
    pub pins : hal_ll_uart_pins_t,
    pub baud_rate : hal_ll_uart_baud_t,
    pub parity : hal_ll_uart_parity_t,
    pub stop_bit : hal_ll_uart_stop_bits_t,
    pub data_bit : hal_ll_uart_data_bits_t,
}

#[allow(dead_code)]
enum hal_ll_uart_state_t
{
    HAL_LL_UART_DISABLE,
    HAL_LL_UART_ENABLE
}

static mut hal_ll_module_state: [hal_ll_uart_handle_register_t; UART_MODULE_COUNT as usize]  = [ 
    hal_ll_uart_handle_register_t{
        uart_handle : 0, 
        init_ll_state : false
        };
        UART_MODULE_COUNT as usize];

static mut hal_ll_uart_hw_specifics_map:[hal_ll_uart_hw_specifics_map_t; (UART_MODULE_COUNT+1) as usize] = [
    #[cfg(feature = "uart1")]
    hal_ll_uart_hw_specifics_map_t{ base: UART1_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart2")]
    hal_ll_uart_hw_specifics_map_t{ base: UART2_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart3")]
    hal_ll_uart_hw_specifics_map_t{ base: UART3_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart4")]
    hal_ll_uart_hw_specifics_map_t{ base: UART4_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart5")]
    hal_ll_uart_hw_specifics_map_t{ base: UART5_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart6")]
    hal_ll_uart_hw_specifics_map_t{ base: UART6_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart7")]
    hal_ll_uart_hw_specifics_map_t{ base: UART7_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    #[cfg(feature = "uart8")]
    hal_ll_uart_hw_specifics_map_t{ base: UART8_BASE_ADDR, module_index: hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8), pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
    hal_ll_uart_hw_specifics_map_t{ base: HAL_LL_MODULE_ERROR, module_index: HAL_LL_PIN_NC, pins: hal_ll_uart_pins_t{ pin_tx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 }, pin_rx: hal_ll_pin_af_t{ pin_name: HAL_LL_PIN_NC, pin_af: 0 } } , baud_rate: hal_ll_uart_baud_t{ baud: 115200, real_baud: 0 }, parity: HAL_LL_UART_PARITY_DEFAULT, stop_bit: HAL_LL_UART_STOP_BITS_DEFAULT, data_bit: HAL_LL_UART_DATA_BITS_DEFAULT },
];

static mut irq_handler : hal_ll_uart_isr_t = empty_handler;

///////// public functions
pub fn hal_ll_uart_register_handle(tx_pin: hal_ll_pin_name_t, rx_pin: hal_ll_pin_name_t, hal_module_id: &mut u8) -> Result<hal_ll_uart_handle_register_t> {
    let pin_check_result: u8;
    let mut index_list: hal_ll_uart_pin_id = 
        hal_ll_uart_pin_id {
            pin_tx: HAL_LL_PIN_NC,
            pin_rx: HAL_LL_PIN_NC,
        };

    pin_check_result = hal_ll_uart_check_pins(tx_pin, rx_pin, &mut index_list);
    if pin_check_result == HAL_LL_PIN_NC {
        return Err(HAL_LL_UART_ERROR::UART_WRONG_PINS);
    }
    
    unsafe{
        if (hal_ll_uart_hw_specifics_map[pin_check_result as usize].pins.pin_rx.pin_name != rx_pin)
        || (hal_ll_uart_hw_specifics_map[pin_check_result as usize].pins.pin_tx.pin_name != tx_pin){
            hal_ll_uart_alternate_functions_set_state(&mut hal_ll_uart_hw_specifics_map[pin_check_result as usize], false );
            hal_ll_uart_map_pins( pin_check_result as usize, &mut index_list);
            hal_ll_uart_alternate_functions_set_state(&mut hal_ll_uart_hw_specifics_map[pin_check_result as usize], true );
        
            hal_ll_module_state[pin_check_result as usize].init_ll_state = false;
        }

        *hal_module_id = pin_check_result;

        hal_ll_module_state[pin_check_result as usize].uart_handle = hal_ll_uart_hw_specifics_map[pin_check_result as usize].base;
        
        Ok(hal_ll_module_state[pin_check_result as usize])
    }
}

pub fn hal_ll_uart_register_irq_handler(handler : hal_ll_uart_isr_t)
{
    unsafe {irq_handler = handler;}
}

pub fn hal_ll_module_configure_uart(handle: &mut hal_ll_uart_handle_register_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_uart_hw_specifics_map_local.module_index as usize;

    hal_ll_uart_init( hal_ll_uart_hw_specifics_map_local );
    unsafe{
        hal_ll_module_state[pin_check_result].uart_handle = hal_ll_uart_hw_specifics_map[pin_check_result].base;
        hal_ll_module_state[pin_check_result].init_ll_state = true;
    }
    hal_handle.init_ll_state = true;
}

pub fn hal_ll_uart_irq_enable(handle: &mut hal_ll_uart_handle_register_t, irq : hal_ll_uart_irq_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let uart_ptr : *mut hal_ll_uart_base_handle_t = hal_ll_uart_hw_specifics_map_local.base as *mut hal_ll_uart_base_handle_t;

    unsafe {
        match irq {
            hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX => {
                    set_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_RXNE);
            },
            hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX => {
                    set_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE);
            },
        }
    }
    
    #[cfg(feature = "uart1")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8)
    {
        hal_ll_core_enable_irq( UART1_NVIC );
    }
    
    #[cfg(feature = "uart2")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8)
    {
         hal_ll_core_enable_irq( UART2_NVIC );
    }
    
    #[cfg(feature = "uart3")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8)
    {
        hal_ll_core_enable_irq( UART3_NVIC );
    }
    
    #[cfg(feature = "uart4")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8)
    {
        hal_ll_core_enable_irq( UART4_NVIC );
    }
    
    #[cfg(feature = "uart5")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8)
    {
        hal_ll_core_enable_irq( UART5_NVIC );
    }
    
    #[cfg(feature = "uart6")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8)
    {
        hal_ll_core_enable_irq( UART6_NVIC );
    }
    
    #[cfg(feature = "uart7")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8)
    {
        hal_ll_core_enable_irq( UART7_NVIC );
    }
    
    #[cfg(feature = "uart8")]
    if hal_ll_uart_hw_specifics_map_local.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8)
    {
        hal_ll_core_enable_irq( UART8_NVIC );
    } 
    
}

pub fn hal_ll_uart_irq_disable(handle: &mut hal_ll_uart_handle_register_t, irq : hal_ll_uart_irq_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let uart_ptr : *mut hal_ll_uart_base_handle_t = hal_ll_uart_hw_specifics_map_local.base as *mut hal_ll_uart_base_handle_t;

    unsafe {
        match irq {
            hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX => {
                    clear_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_RXNE);
            },
            hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX => {
                    clear_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE);
            },
        }
    }
}

pub fn hal_ll_uart_set_baud(handle: &mut hal_ll_uart_handle_register_t, baud: u32) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_uart_hw_specifics_map_local.baud_rate.baud = baud;

    hal_ll_uart_init( hal_ll_uart_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_uart_set_parity(handle: &mut hal_ll_uart_handle_register_t, parity: hal_ll_uart_parity_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_uart_hw_specifics_map_local.parity = parity;

    hal_ll_uart_init( hal_ll_uart_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_uart_set_stop_bits(handle: &mut hal_ll_uart_handle_register_t, stop_bit: hal_ll_uart_stop_bits_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_uart_hw_specifics_map_local.stop_bit = stop_bit;

    hal_ll_uart_init( hal_ll_uart_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_uart_set_data_bits(handle: &mut hal_ll_uart_handle_register_t, data_bit: hal_ll_uart_data_bits_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);

    hal_handle.init_ll_state = false;

    hal_ll_uart_hw_specifics_map_local.data_bit = data_bit;

    hal_ll_uart_init( hal_ll_uart_hw_specifics_map_local );

    hal_handle.init_ll_state = true;
}

pub fn hal_ll_uart_write(handle: &mut hal_ll_uart_handle_register_t, wr_data: u8) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let uart_ptr : *mut hal_ll_uart_base_handle_t = hal_ll_uart_hw_specifics_map_local.base as *mut hal_ll_uart_base_handle_t;

    unsafe {(*uart_ptr).dr = wr_data as u32 & 0xFF;}
}

pub fn hal_ll_uart_read(handle: &mut hal_ll_uart_handle_register_t) -> u8 {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let uart_ptr : *mut hal_ll_uart_base_handle_t = hal_ll_uart_hw_specifics_map_local.base as *mut hal_ll_uart_base_handle_t;

    unsafe {((*uart_ptr).dr & 0xFF) as u8}
}

pub fn hal_ll_uart_close(handle: &mut hal_ll_uart_handle_register_t) {
    let hal_handle : &mut hal_ll_uart_handle_register_t = handle;
    let hal_ll_uart_hw_specifics_map_local: &mut hal_ll_uart_hw_specifics_map_t = hal_ll_get_specifics(*hal_handle);
    let pin_check_result: usize = hal_ll_uart_hw_specifics_map_local.module_index as usize;
    let uart_ptr : *mut hal_ll_uart_base_handle_t = hal_ll_uart_hw_specifics_map_local.base as *mut hal_ll_uart_base_handle_t;

    
    if hal_handle.uart_handle != 0 {

        hal_ll_uart_set_clock(hal_ll_uart_hw_specifics_map_local, true);
        hal_ll_uart_alternate_functions_set_state(hal_ll_uart_hw_specifics_map_local, false );
        
        hal_ll_uart_irq_disable(hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
        hal_ll_uart_irq_disable(hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);

        hal_ll_uart_clear_regs(uart_ptr);
        hal_ll_uart_set_clock(hal_ll_uart_hw_specifics_map_local, false);

        *hal_handle = hal_ll_uart_handle_register_t::default();


        hal_ll_uart_hw_specifics_map_local.pins.pin_tx.pin_name = HAL_LL_PIN_NC;
        hal_ll_uart_hw_specifics_map_local.pins.pin_tx.pin_af = 0;
        hal_ll_uart_hw_specifics_map_local.pins.pin_rx.pin_name = HAL_LL_PIN_NC;
        hal_ll_uart_hw_specifics_map_local.pins.pin_rx.pin_af = 0;

        hal_ll_uart_hw_specifics_map_local.baud_rate.baud = 115200;
        hal_ll_uart_hw_specifics_map_local.baud_rate.real_baud = 0;
        
        unsafe{hal_ll_module_state[pin_check_result as usize] = *hal_handle;}
    }
}

///////// private functions

fn hal_ll_uart_check_pins(tx_pin: hal_ll_pin_name_t , rx_pin: hal_ll_pin_name_t , index_list: &mut hal_ll_uart_pin_id) -> u8 {
    let tx_map_size: u8 = hal_ll_uart_tx_map.len() as u8 ;
    let rx_map_size: u8 = hal_ll_uart_rx_map.len() as u8 ;

    let mut index_counter: u8 = 0;
    let mut hal_ll_module_id: u8 = 0;

    if HAL_LL_PIN_NC == tx_pin || HAL_LL_PIN_NC == rx_pin {
        return HAL_LL_PIN_NC;
    }
    
    for  tx_index in 0x00 .. tx_map_size
    {
        if hal_ll_uart_tx_map[tx_index as usize].pin == tx_pin {

            for  rx_index in 0x00 .. rx_map_size
            {
                if hal_ll_uart_rx_map[rx_index as usize].pin == rx_pin
                {   
                    if  hal_ll_uart_tx_map[tx_index as usize].module_index == hal_ll_uart_rx_map[rx_index as usize].module_index {
                        // Get module number
                        hal_ll_module_id = hal_ll_uart_tx_map[tx_index as usize].module_index;
                        // Map pin names
                        index_list.pin_tx = tx_index;
                        index_list.pin_rx = rx_index;

                        if unsafe{hal_ll_module_state[ hal_ll_module_id as usize].uart_handle} == hal_ll_uart_handle_register_t::default().uart_handle{
                            return hal_ll_module_id;
                        } else {
                            index_counter = index_counter + 1;
                            if UART_MODULE_COUNT == index_counter {
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
fn hal_ll_get_specifics<'a>(handle: hal_ll_uart_handle_register_t) -> &'a mut hal_ll_uart_hw_specifics_map_t {
    let mut hal_ll_module_count: usize = UART_MODULE_COUNT as usize;
    let mut hal_ll_module_error : usize = 0;
    hal_ll_module_error = hal_ll_module_count;
    
    unsafe{
        while hal_ll_module_count > 0 {
            hal_ll_module_count -= 1;

            let base: u32 = handle.uart_handle;

            if base == hal_ll_uart_hw_specifics_map[hal_ll_module_count].base {
                return &mut hal_ll_uart_hw_specifics_map[hal_ll_module_count];
            }
        }

        return &mut hal_ll_uart_hw_specifics_map[hal_ll_module_error];
    }
}

fn hal_ll_uart_map_pins(module_index: usize, index_list: &mut hal_ll_uart_pin_id) {
    unsafe{
        // Map new pins
        hal_ll_uart_hw_specifics_map[module_index].pins.pin_rx.pin_name = hal_ll_uart_rx_map[ index_list.pin_rx as usize].pin;
        hal_ll_uart_hw_specifics_map[module_index].pins.pin_tx.pin_name = hal_ll_uart_tx_map[ index_list.pin_tx as usize].pin;
        // TX and RX could have different alternate function settings, hence save both AF values
        hal_ll_uart_hw_specifics_map[module_index].pins.pin_rx.pin_af = hal_ll_uart_rx_map[ index_list.pin_rx as usize].af;
        hal_ll_uart_hw_specifics_map[module_index].pins.pin_tx.pin_af = hal_ll_uart_tx_map[ index_list.pin_tx as usize].af;
    }
}

fn hal_ll_uart_alternate_functions_set_state(map: &mut hal_ll_uart_hw_specifics_map_t, hal_ll_state: bool) {
    let mut module: module_struct = module_struct::default();

    if ((*map).pins.pin_rx.pin_name != HAL_LL_PIN_NC) && ((*map).pins.pin_tx.pin_name != HAL_LL_PIN_NC)  {
        module.pins[0] = VALUE( (*map).pins.pin_rx.pin_name, (*map).pins.pin_rx.pin_af );
        module.pins[1] = VALUE( (*map).pins.pin_tx.pin_name, (*map).pins.pin_tx.pin_af );
        

        module.configs[0] = HAL_LL_UART_AF_CONFIG;
        module.configs[1] = HAL_LL_UART_AF_CONFIG;


        hal_ll_gpio_module_struct_init( &mut module, hal_ll_state );
    }
}

fn hal_ll_uart_set_clock(map: &mut hal_ll_uart_hw_specifics_map_t, hal_ll_state : bool) {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        #[cfg(feature = "uart1")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_USART1EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_USART1EN );
            }
        }
        #[cfg(feature = "uart2")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_USART2EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_USART2EN );
            }
        }
        #[cfg(feature = "uart3")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_USART3EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_USART3EN );
            }
        }
        #[cfg(feature = "uart4")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART4EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART4EN );
            }
        }
        #[cfg(feature = "uart5")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART5EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART5EN );
            }
        }
        #[cfg(feature = "uart6")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_USART6EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB2ENR as *const u32 as u32, HAL_LL_USART6EN );
            }
        }
        #[cfg(feature = "uart7")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART7EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART7EN );
            }
        }
        #[cfg(feature = "uart8")]
        if map.module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8)
        {
            if hal_ll_state {
                set_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART8EN );
            } else {
                clear_reg_bit( &(*rcc_ptr).APB1ENR as *const u32 as u32, HAL_LL_UART8EN );
            }
        }
    }
    
}

fn hal_ll_uart_get_clock_speed(module_index : hal_ll_pin_name_t) -> u32 {
    let mut rcc_clocks : RCC_ClocksTypeDef = RCC_ClocksTypeDef::default();

    rcc_get_clocks_frequency( &mut rcc_clocks );

    #[cfg(feature = "uart1")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_1 as u8)
    {
        return rcc_clocks.PCLK2_Frequency;
    }
    #[cfg(feature = "uart2")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_2 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }
    #[cfg(feature = "uart3")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_3 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }
    #[cfg(feature = "uart4")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_4 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }
    #[cfg(feature = "uart5")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_5 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }
    #[cfg(feature = "uart6")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_6 as u8)
    {
        return rcc_clocks.PCLK2_Frequency;
    }
    #[cfg(feature = "uart7")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_7 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }
    #[cfg(feature = "uart8")]
    if module_index == hal_ll_uart_module_num(uart_modules::UART_MODULE_8 as u8)
    {
        return rcc_clocks.PCLK1_Frequency;
    }

    return 0;
}

fn hal_ll_uart_clear_regs(uart_ptr : *mut hal_ll_uart_base_handle_t) {
    unsafe{
        (*uart_ptr).cr1 &= HAL_LL_UART_IT_CR1_MASK;
        (*uart_ptr).cr2 &= HAL_LL_UART_IT_CR2_MASK;
        (*uart_ptr).cr3 &= HAL_LL_UART_IT_CR3_MASK;
    }
}

fn hal_ll_uart_set_data_bits_bare_metal(map: &mut hal_ll_uart_hw_specifics_map_t) {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = map.base as *mut hal_ll_uart_base_handle_t;
    unsafe {
        match map.data_bit
        {
            hal_ll_uart_data_bits_t::HAL_LL_UART_DATA_BITS_8 => {
                    (*uart_ptr).cr1 |= DATA_BIT_8;
                }
            hal_ll_uart_data_bits_t::HAL_LL_UART_DATA_BITS_9 => {
                    (*uart_ptr).cr1 |= DATA_BIT_9;
                }
            _ => ()
        }
    }
}

fn hal_ll_uart_set_parity_bare_metal(map: &mut hal_ll_uart_hw_specifics_map_t) {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = map.base as *mut hal_ll_uart_base_handle_t;

    unsafe {
        match map.parity
        {
            hal_ll_uart_parity_t::HAL_LL_UART_PARITY_NONE => {
                    (*uart_ptr).cr1 |= PARITY_NO;
                },
            hal_ll_uart_parity_t::HAL_LL_UART_PARITY_EVEN => {
                    (*uart_ptr).cr1 |= PARITY_ENABLED | PARITY_EVEN;
                },
            hal_ll_uart_parity_t::HAL_LL_UART_PARITY_ODD => {
                    (*uart_ptr).cr1 |= PARITY_ENABLED | PARITY_ODD;
                },
        }
    }
}

fn hal_ll_uart_set_stop_bits_bare_metal(map: &mut hal_ll_uart_hw_specifics_map_t) {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = map.base as *mut hal_ll_uart_base_handle_t;

    unsafe {
        match map.stop_bit
        {
            hal_ll_uart_stop_bits_t::HAL_LL_UART_STOP_BITS_HALF => {
                            (*uart_ptr).cr2 |= STOP_BITS_HALF;
                        },
            hal_ll_uart_stop_bits_t::HAL_LL_UART_STOP_BITS_ONE => {
                            (*uart_ptr).cr2 |= STOP_BITS_ONE;
                        },
            hal_ll_uart_stop_bits_t::HAL_LL_UART_STOP_BITS_ONE_AND_A_HALF => {
                            (*uart_ptr).cr2 |= STOP_BITS_ONE_AND_A_HALF;
                        },
            hal_ll_uart_stop_bits_t::HAL_LL_UART_STOP_BITS_TWO => {
                            (*uart_ptr).cr2 |= STOP_BITS_TWO;
                        },
        }
    }
}

#[allow(unused_assignments)]
fn hal_ll_uart_set_baud_bare_metal(map: &mut hal_ll_uart_hw_specifics_map_t) {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = map.base as *mut hal_ll_uart_base_handle_t;
    let hal_ll_baud_value : u32 = ((hal_ll_uart_get_clock_speed(map.module_index) / map.baud_rate.baud) / 4) - 1;
    let hal_ll_baud_real_value : u32 = hal_ll_uart_get_clock_speed(map.module_index) / (4 * (hal_ll_baud_value + 1));
    let hal_ll_baud_real_value_error : f32 = (i32::abs((hal_ll_baud_real_value - map.baud_rate.baud) as i32) as u32 / map.baud_rate.baud) as f32 * 100.0;
    let mut integer_divider : u32 = 0;
    let mut fractional_divider : u32 = 0;
    let mut reg_value : u32 = 0;

    if hal_ll_baud_real_value_error > HAL_LL_UART_ACCEPTABLE_ERROR {
        map.baud_rate.real_baud = hal_ll_baud_real_value_error as u32;
    } else {
        map.baud_rate.real_baud = hal_ll_baud_real_value;

        integer_divider = (25 * hal_ll_uart_get_clock_speed(map.module_index)) / (4 * map.baud_rate.baud);
        reg_value = (integer_divider / 100) << 4;

        fractional_divider = integer_divider - (100 * (reg_value >> 4));
        reg_value |= ((((fractional_divider * 16) + 50) / 100)) & 0x0F;

        unsafe {
            (*uart_ptr).brr = reg_value;
        }
    }

}

fn hal_ll_uart_set_transmitter(uart_ptr : *mut hal_ll_uart_base_handle_t, pin_state : hal_ll_uart_state_t) {
    unsafe {
        match pin_state
        {
            hal_ll_uart_state_t::HAL_LL_UART_DISABLE =>{
                    clear_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_TE );
                }

            hal_ll_uart_state_t::HAL_LL_UART_ENABLE => {
                    set_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_TE );
                }
        }
    }
}

fn hal_ll_uart_set_receiver(uart_ptr : *mut hal_ll_uart_base_handle_t, pin_state : hal_ll_uart_state_t) {
    unsafe {
        match pin_state
        {
            hal_ll_uart_state_t::HAL_LL_UART_DISABLE =>{
                    clear_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_RE );
                }

            hal_ll_uart_state_t::HAL_LL_UART_ENABLE => {
                    set_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_RE );
                }
        }
    }
}

fn hal_ll_uart_set_module(uart_ptr : *mut hal_ll_uart_base_handle_t, pin_state : hal_ll_uart_state_t) {
    unsafe {
        match pin_state
        {
            hal_ll_uart_state_t::HAL_LL_UART_DISABLE =>{
                    clear_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_UE );
                }

            hal_ll_uart_state_t::HAL_LL_UART_ENABLE => {
                    set_reg_bit( &(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_CR1_UE );
                }
        }
    }
}

fn hal_ll_uart_hw_init(map: &mut hal_ll_uart_hw_specifics_map_t) {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = map.base as *mut hal_ll_uart_base_handle_t;
    hal_ll_uart_clear_regs(uart_ptr);
    hal_ll_uart_set_data_bits_bare_metal(map);
    hal_ll_uart_set_parity_bare_metal(map);
    hal_ll_uart_set_stop_bits_bare_metal(map);
    hal_ll_uart_set_baud_bare_metal(map);
    hal_ll_uart_set_transmitter(uart_ptr, hal_ll_uart_state_t::HAL_LL_UART_ENABLE);
    hal_ll_uart_set_receiver(uart_ptr, hal_ll_uart_state_t::HAL_LL_UART_ENABLE);
    hal_ll_uart_set_module(uart_ptr, hal_ll_uart_state_t::HAL_LL_UART_ENABLE);
}

fn hal_ll_uart_init(map: &mut hal_ll_uart_hw_specifics_map_t) {
    hal_ll_uart_set_clock(map, true);
    hal_ll_uart_alternate_functions_set_state(map, true);
    hal_ll_uart_hw_init(map);
}

#[allow(unused_variables)]
fn empty_handler(handle : &mut hal_ll_uart_handle_register_t, event : hal_ll_uart_irq_t ){}

///// INTERRUPT HANDLERS
#[unsafe(no_mangle)]
#[cfg(feature = "uart1")]
pub extern "Rust" fn UART1_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART1_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_1], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_1], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart2")]
pub extern "Rust" fn UART2_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART2_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_2], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_2], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart3")]
pub extern "Rust" fn UART3_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART3_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_3], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_3], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart4")]
pub extern "Rust" fn UART4_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART4_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_4], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_4], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart5")]
pub extern "Rust" fn UART5_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART5_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_5], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_5], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart6")]
pub extern "Rust" fn UART6_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART6_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_6], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_6], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart7")]
pub extern "Rust" fn UART7_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART7_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_7], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_7], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}

#[unsafe(no_mangle)]
#[cfg(feature = "uart8")]
pub extern "Rust" fn UART8_IRQHandler() {
    let uart_ptr : *mut hal_ll_uart_base_handle_t = UART8_BASE_ADDR as *mut hal_ll_uart_base_handle_t;

    unsafe {
        if (*uart_ptr).sr & HAL_LL_UART_STATUS_RXNE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32,HAL_LL_UART_IT_RXNE) != 0 {
                clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_RXNE); //software clear
                irq_handler(&mut hal_ll_module_state[uart_index_8], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
            }
        }

        if (*uart_ptr).sr & HAL_LL_UART_STATUS_TXE_FLAG > 0 {
            
            if check_reg_bit(&(*uart_ptr).cr1 as *const u32 as u32, HAL_LL_UART_IT_TXE) != 0 {
                //clear_reg_bit(&(*uart_ptr).sr as *const u32 as u32, HAL_LL_UART_IT_TXE); //hardware clear by setting data to reg_dr
                irq_handler(&mut hal_ll_module_state[uart_index_8], hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            }
        }
    }
    
}