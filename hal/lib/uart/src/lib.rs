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

mod constant_setting;
use constant_setting::*;


use hal_ll_uart::*;
use hal_target::*;
use hal_target::pin_names::*;
use hal_gpio::*;
use interrupt::interrupt_helper::*;

use hal_ll_uart::hal_ll_uart_irq_t as hal_uart_irq_t;
pub use hal_ll_uart::hal_ll_uart_handle_register_t as hal_uart_handle_register_t;
pub use hal_ll_uart::HAL_LL_UART_ERROR as HAL_UART_ERROR;
pub use hal_ll_uart::hal_ll_uart_stop_bits_t as hal_uart_stop_bits_t;
pub use hal_ll_uart::HAL_LL_UART_STOP_BITS_DEFAULT as HAL_UART_STOP_BITS_DEFAULT;
pub use hal_ll_uart::hal_ll_uart_parity_t as hal_uart_parity_t;
pub use hal_ll_uart::HAL_LL_UART_PARITY_DEFAULT as HAL_UART_PARITY_DEFAULT;
pub use hal_ll_uart::hal_ll_uart_data_bits_t as hal_uart_data_bits_t;
pub use hal_ll_uart::HAL_LL_UART_DATA_BITS_DEFAULT as HAL_UART_DATA_BITS_DEFAULT;
use ring::ring_buf8_t;
use system::Delay_ms;

type Result<T> = core::result::Result<T, HAL_UART_ERROR>;
type Result_ring = core::result::Result<u8, ()>;

#[derive(Clone, Copy, PartialEq)]
pub struct hal_uart_config_t
{
    pub tx: hal_pin_name_t, /*< Tx pin. */
    pub rx: hal_pin_name_t, /*< Rx pin. */

    pub baud: u32,                       /*< Baud rate. */
    pub data_bits: hal_uart_data_bits_t, /*< Data bits. See #hal_uart_data_bits_t for valid values.*/
    pub parity: hal_uart_parity_t,       /*< Parity bits. See #hal_uart_parity_t for valid values.*/
    pub stop_bits: hal_uart_stop_bits_t, /*< Stop bits. See #hal_uart_stop_bits_t for valid values.*/

    pub is_tx_irq_enabled: bool, /*< Tx interrupt enabled. */
    pub is_rx_irq_enabled: bool, /*< Rx interrupt enabled. */

    pub is_blocking: bool, /*< Is blocking. */
}

impl Default for hal_uart_config_t {
    fn default() -> Self {
        Self {
            tx: HAL_PIN_NC,
            rx: HAL_PIN_NC,
            baud: 115200,
            data_bits: HAL_UART_DATA_BITS_DEFAULT,
            parity: HAL_UART_PARITY_DEFAULT,
            stop_bits: HAL_UART_STOP_BITS_DEFAULT,
            is_tx_irq_enabled: false,
            is_rx_irq_enabled: false,
            is_blocking: false,
        }
    }
}

struct ring_buffers_t <const tx_capacity : usize, const rx_capacity : usize> {
    pub tx_buf: ring_buf8_t<tx_capacity>, /*< Tx buffer. */
    pub rx_buf: ring_buf8_t<rx_capacity>, /*< Rx buffer. */
}

impl<const tx_capacity : usize, const rx_capacity : usize> ring_buffers_t<tx_capacity, rx_capacity> {
    pub const fn init() -> Self {
        Self {
            tx_buf: ring_buf8_t::<tx_capacity>::init(),
            rx_buf: ring_buf8_t::<rx_capacity>::init(),
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct hal_uart_t
{
    pub handle: hal_uart_handle_register_t, /*< UART HAL handle. */
    pub config: hal_uart_config_t, /*< UART HAL configuration structure. */

    // uint8_t *tx_ring_buffer, /*< Pointer to Tx ring buffer. */
    // uint8_t *rx_ring_buffer, /*< Pointer to Rx ring buffer. */
}

impl Default for hal_uart_t {
    fn default() -> Self {
        Self { 
            handle: hal_uart_handle_register_t{ uart_handle: 0, init_ll_state: false, }, 
            config: hal_uart_config_t {
                tx: HAL_PIN_NC,
                rx: HAL_PIN_NC,
                baud: 115200,
                data_bits: HAL_UART_DATA_BITS_DEFAULT,
                parity: HAL_UART_PARITY_DEFAULT,
                stop_bits: HAL_UART_STOP_BITS_DEFAULT,
                is_tx_irq_enabled: false,
                is_rx_irq_enabled: false,
                is_blocking: false,
            },
        }
    }
}

static mut hal_module_state: [hal_uart_handle_register_t; UART_MODULE_COUNT as usize]  = [ 
    hal_uart_handle_register_t{ 
        uart_handle: 0xFFFF_FFFF, 
        init_ll_state: false,
    };
    UART_MODULE_COUNT as usize];


static mut ring_buffer_uart_1 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_2 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_3 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_4 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_5 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_6 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_7 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();
static mut ring_buffer_uart_8 : ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE> = ring_buffers_t::<RING_BUFFER_TX_SIZE, RING_BUFFER_RX_SIZE>::init();

fn hal_is_handle_register<'a>(hal_module_handle: &mut hal_uart_t) -> Option<&'a mut hal_uart_handle_register_t> {
    let mut hal_module_state_count: u8 = UART_MODULE_COUNT;

    while hal_module_state_count > 0 
    {   
        unsafe{
            if hal_module_handle.handle.uart_handle == hal_module_state[ (hal_module_state_count - 1) as usize ].uart_handle
            {
                return Some(&mut hal_module_state[ (hal_module_state_count - 1) as usize]);
            }
        }
        hal_module_state_count = hal_module_state_count - 1;
    }
     return None;
}

pub fn hal_uart_open(handle: &mut hal_uart_t, hal_obj_open_state: bool) -> Result<()> {
    let mut hal_module_id: u8 = 0;
    let hal_obj: &mut hal_uart_t = handle;

    match hal_is_handle_register( hal_obj ) {
        Some(_) => {
            if hal_obj_open_state
            {
                return Err(HAL_UART_ERROR::ACQUIRE_FAIL) 
            } else {
                return Ok(())
            }
        },
        None => {
            if !hal_obj_open_state {
                return Err(HAL_UART_ERROR::ACQUIRE_FAIL) 
            }
        },
    }


    unsafe{
        match hal_ll_uart_register_handle(hal_obj.config.tx, hal_obj.config.rx,  &mut hal_module_id )
        {

            Ok(h) => {
                hal_module_state[ hal_module_id as usize] = h;
                (*hal_obj).handle = hal_module_state[ hal_module_id as usize];
                hal_ll_uart_register_irq_handler(interruption_handler);
                hal_ll_core_enable_interrupts();

                Ok(())
            },
            Err(_) => {
                hal_obj.handle = hal_uart_t::default().handle;
                return Err(HAL_UART_ERROR::ACQUIRE_FAIL) 
            },
        }

    }
}

pub fn hal_uart_set_parity(handle: &mut hal_uart_t, config: hal_uart_config_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    hal_ll_uart_set_parity(hal_handle , config.parity);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_uart_set_stop_bits(handle: &mut hal_uart_t, config: hal_uart_config_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    hal_ll_uart_set_stop_bits(hal_handle , config.stop_bits);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_uart_set_data_bits(handle: &mut hal_uart_t, config: hal_uart_config_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    hal_ll_uart_set_data_bits(hal_handle , config.data_bits);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_uart_set_baud(handle: &mut hal_uart_t, config: hal_uart_config_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    hal_ll_uart_set_baud(hal_handle , config.baud);
    hal_obj.handle = *hal_handle;
    Ok(())
}

pub fn hal_uart_set_blocking(handle: &mut hal_uart_t, config: hal_uart_config_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    match hal_is_handle_register( hal_obj ){
        Some(_) => () ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    hal_obj.config.is_blocking = config.is_blocking;

    Ok(())
}

pub fn hal_uart_read(handle: &mut hal_uart_t,  read_data_buf: &mut [u8], len_read_data: usize) -> Result<usize> {
    let hal_obj: &mut hal_uart_t = handle;
    let mut hal_handle: &mut hal_uart_handle_register_t;
    let mut size: usize = len_read_data;
    let mut size_data_read: usize = 0;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    if len_read_data == 0 || len_read_data > read_data_buf.len() {
        return Err(HAL_UART_ERROR::UART_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_uart(hal_handle);
        hal_obj.handle = *hal_handle;
    }

    if !hal_obj.config.is_rx_irq_enabled {
        hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
        hal_obj.config.is_rx_irq_enabled = true;
    }

    hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);


    if is_buffer_empty(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX) {
        if !hal_obj.config.is_blocking {
                return Ok(size_data_read);
        }

        while is_buffer_empty(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX) {
            ();
        }
    }
    
    
    while size > 0 && !is_buffer_empty(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX) {
        let data_byte: u8;

        hal_ll_core_disable_interrupts();
        {     
            match buffer_pop(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX )
            {
                Ok(data) => data_byte = data,
                Err(_) => {
                    hal_ll_core_enable_interrupts();
                    return Err(HAL_UART_ERROR::UART_ERROR);
                    },
            }

            if !hal_obj.config.is_rx_irq_enabled && !is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX) {
                hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
                hal_obj.config.is_rx_irq_enabled = true;
            }
        }
        hal_ll_core_enable_interrupts();

        read_data_buf[size_data_read] = data_byte;
        size_data_read += 1;
        size -= 1;

    }

    Ok(size_data_read)
}

pub fn hal_uart_write(handle: &mut hal_uart_t,  write_data_buffer: &mut [u8], write_data_length: usize) -> Result<usize> {
    let hal_obj: &mut hal_uart_t = handle;
    let mut hal_handle: &mut hal_uart_handle_register_t;
    let mut size_data_written: usize = 0;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    if write_data_length == 0 || write_data_length > write_data_buffer.len() {
        return Err(HAL_UART_ERROR::UART_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_uart(hal_handle);
        hal_obj.handle = *hal_handle;
    }

    if !hal_obj.config.is_tx_irq_enabled {
        hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
        hal_obj.config.is_tx_irq_enabled = true;
    }

    hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);

    
    while size_data_written < write_data_length {
        if is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
            if !hal_obj.config.is_blocking {
                break;
            }
            
            while is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
                ()
            }
        }

        hal_ll_core_disable_interrupts();
        {
            buffer_push(hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX, write_data_buffer[size_data_written]);

            if !hal_obj.config.is_tx_irq_enabled && !is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
                hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
                hal_obj.config.is_tx_irq_enabled = true;
            }
        }
        hal_ll_core_enable_interrupts();
        size_data_written += 1;
    }

    if size_data_written != 0 {
        let first_data_byte: u8;
        match buffer_pop(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX )
        {
            Ok(data) => first_data_byte = data,
            Err(_) => first_data_byte = 0x00,
        }

        hal_ll_uart_write(hal_handle, first_data_byte);
    }

    Ok(size_data_written)
}

pub fn hal_uart_print(handle: &mut hal_uart_t, print_string: &str) -> Result<usize> {
    let hal_obj: &mut hal_uart_t = handle;
    let mut hal_handle: &mut hal_uart_handle_register_t;
    let mut size_data_written: usize = 0;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    if print_string.len() == 0 {
        return Err(HAL_UART_ERROR::UART_ERROR);
    }

    if !hal_handle.init_ll_state {
        hal_ll_module_configure_uart(hal_handle);
        hal_obj.handle = *hal_handle;
    }

    if !hal_obj.config.is_tx_irq_enabled {
        hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
        hal_obj.config.is_tx_irq_enabled = true;
    }

    hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);

    for character in print_string.as_bytes() {
        if is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
            if !hal_obj.config.is_blocking {
                break;
            }
            
            while is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
                ()
            }
        }

        hal_ll_core_disable_interrupts();
        {
            buffer_push(hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX, *character);

            if !hal_obj.config.is_tx_irq_enabled && !is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
                hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
                hal_obj.config.is_tx_irq_enabled = true;
            }
        }
        hal_ll_core_enable_interrupts();
        size_data_written += 1;
    }


    Ok(size_data_written)
}

pub fn hal_uart_println(handle: &mut hal_uart_t, print_string: &str) -> Result<usize> {
    let hal_obj: &mut hal_uart_t = handle;
    let mut hal_handle: &mut hal_uart_handle_register_t;
    let mut size_data_written: usize = 0;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    size_data_written = hal_uart_print(hal_obj, print_string )?;


    hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);

    while is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
        if !hal_obj.config.is_blocking {
            break;
        }
    }

    hal_ll_core_disable_interrupts();
    {
        buffer_push(hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX, b'\n');

        if !hal_obj.config.is_tx_irq_enabled && !is_buffer_full(hal_handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX) {
            hal_ll_uart_irq_enable(&mut hal_handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
            hal_obj.config.is_tx_irq_enabled = true;
        }
    }
    hal_ll_core_enable_interrupts();
    size_data_written += 1;


    Ok(size_data_written)
}

pub fn hal_uart_close(handle: &mut hal_uart_t)  -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;
    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    if (hal_handle.uart_handle != 0) & (hal_handle.uart_handle != 0xFFFF_FFFF)
    {
        hal_ll_uart_close( hal_handle );

        hal_obj.config = hal_uart_config_t::default();
        *hal_obj = hal_uart_t::default();
        hal_handle.uart_handle = 0xFFFF_FFFF;

        return Ok(())
    }

    Err(HAL_UART_ERROR::UART_ERROR)
}

///// cursed programming ahead

pub fn hal_uart_bytes_available(handle: &mut hal_uart_t) -> Result<usize> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    unsafe{
        match get_handle_index(hal_handle) {
            1 => {
                let buffer = &raw mut ring_buffer_uart_1.rx_buf;
                    Ok((*buffer).get_size())
            },
            2 => {
                let buffer = &raw mut ring_buffer_uart_2.rx_buf;
                    Ok((*buffer).get_size())
            },
            3 => {
                let buffer = &raw mut ring_buffer_uart_3.rx_buf;
                    Ok((*buffer).get_size())
            },
            4 => {
                let buffer = &raw mut ring_buffer_uart_4.rx_buf;
                    Ok((*buffer).get_size())
            },
            5 => {
                let buffer = &raw mut ring_buffer_uart_5.rx_buf;
                    Ok((*buffer).get_size())
            },
            6 => {
                let buffer = &raw mut ring_buffer_uart_6.rx_buf;
                    Ok((*buffer).get_size())
            },
            7 => {
                let buffer = &raw mut ring_buffer_uart_7.rx_buf;
                    Ok((*buffer).get_size())
            },
            8 => {
                let buffer = &raw mut ring_buffer_uart_8.rx_buf;
                    Ok((*buffer).get_size())
            },
            _ => Err(HAL_UART_ERROR::UART_ERROR),
        }

    }
}

pub fn hal_uart_clear(handle: &mut hal_uart_t) -> Result<()> {
    let hal_obj: &mut hal_uart_t = handle;
    let hal_handle: &mut hal_uart_handle_register_t;

    match hal_is_handle_register( hal_obj ){
        Some(h) => hal_handle = h ,
        None => return Err(HAL_UART_ERROR::UART_ERROR),
    }

    unsafe{
        match get_handle_index(hal_handle) {
            1 => {
                let buffer_rx = &raw mut ring_buffer_uart_1.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_1.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            2 => {
                let buffer_rx = &raw mut ring_buffer_uart_2.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_2.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            3 => {
                let buffer_rx = &raw mut ring_buffer_uart_3.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_3.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            4 => {
                let buffer_rx = &raw mut ring_buffer_uart_4.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_4.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            5 => {
                let buffer_rx = &raw mut ring_buffer_uart_5.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_5.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            6 => {
                let buffer_rx = &raw mut ring_buffer_uart_6.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_6.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            7 => {
                let buffer_rx = &raw mut ring_buffer_uart_7.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_7.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            8 => {
                let buffer_rx = &raw mut ring_buffer_uart_8.rx_buf;
                let buffer_tx = &raw mut ring_buffer_uart_8.tx_buf;
                (*buffer_rx).clear();
                (*buffer_tx).clear();
            },
            _ => return Err(HAL_UART_ERROR::UART_ERROR),
        }
        Ok(())
    }
}

fn get_handle_index(handle : &mut hal_uart_handle_register_t) -> u8 {
    for i in 0 .. UART_MODULE_COUNT as usize {
        unsafe{
            if handle.uart_handle == hal_module_state[i].uart_handle {
                return ( i + 1 ) as u8;
            }
        }
    }
    0xFF
}

fn is_buffer_full(handle : &mut hal_uart_handle_register_t, event : hal_uart_irq_t) -> bool {
    unsafe{
        match get_handle_index(handle) {
            1 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_1.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_1.tx_buf;
                    return (*buffer).is_full();
                }
            },
            2 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_2.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_2.tx_buf;
                    return (*buffer).is_full();
                }
            },
            3 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_3.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_3.tx_buf;
                    return (*buffer).is_full();
                }
            },
            4 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_4.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_4.tx_buf;
                    return (*buffer).is_full();
                }
            },
            5 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_5.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_5.tx_buf;
                    return (*buffer).is_full();
                }
            },
            6 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_6.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_6.tx_buf;
                    return (*buffer).is_full();
                }
            },
            7 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_7.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_7.tx_buf;
                    return (*buffer).is_full();
                }
            },
            8 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_8.rx_buf;
                    return (*buffer).is_full();
                } else {
                    let buffer = &raw mut ring_buffer_uart_8.tx_buf;
                    return (*buffer).is_full();
                }
            },
            _ => true,
        }
    }
    
}

fn is_buffer_empty(handle : &mut hal_uart_handle_register_t, event : hal_uart_irq_t) -> bool {
    unsafe{
        match get_handle_index(handle) {
            1 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_1.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_1.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            2 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_2.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_2.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            3 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_3.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_3.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            4 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_4.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_4.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            5 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_5.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_5.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            6 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_6.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_6.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            7 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_7.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_7.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            8 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_8.rx_buf;
                    return (*buffer).is_empty();
                } else {
                    let buffer = &raw mut ring_buffer_uart_8.tx_buf;
                    return (*buffer).is_empty();
                }
            },
            _ => true,
        }
    }
    
}

fn buffer_push(handle : &mut hal_uart_handle_register_t, event : hal_uart_irq_t, data : u8) {
    unsafe{
        match get_handle_index(handle) {
            1 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_1.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_1.tx_buf;
                    (*buffer).push(data);
                }
            },
            2 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_2.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_2.tx_buf;
                    (*buffer).push(data);
                }
            },
            3 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_3.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_3.tx_buf;
                    (*buffer).push(data);
                }
            },
            4 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_4.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_4.tx_buf;
                    (*buffer).push(data);
                }
            },
            5 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_5.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_5.tx_buf;
                    (*buffer).push(data);
                }
            },
            6 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_6.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_6.tx_buf;
                    (*buffer).push(data);
                }
            },
            7 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_7.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_7.tx_buf;
                    (*buffer).push(data);
                }
            },
            8 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_8.rx_buf;
                    (*buffer).push(data);
                } else {
                    let buffer = &raw mut ring_buffer_uart_8.tx_buf;
                    (*buffer).push(data);
                }
            },
            _ => (),
        }
    }
    
}

fn buffer_pop(handle : &mut hal_uart_handle_register_t, event : hal_uart_irq_t) -> Result_ring {
    unsafe{
        match get_handle_index(handle) {
            1 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_1.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_1.tx_buf;
                    (*buffer).pop()
                }
            },
            2 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_2.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_2.tx_buf;
                    (*buffer).pop()
                }
            },
            3 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_3.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_3.tx_buf;
                    (*buffer).pop()
                }
            },
            4 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_4.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_4.tx_buf;
                    (*buffer).pop()
                }
            },
            5 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_5.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_5.tx_buf;
                    (*buffer).pop()
                }
            },
            6 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_6.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_6.tx_buf;
                    (*buffer).pop()
                }
            },
            7 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_7.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_7.tx_buf;
                    (*buffer).pop()
                }
            },
            8 => {
                if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
                {
                    let buffer = &raw mut ring_buffer_uart_8.rx_buf;
                    (*buffer).pop()
                } else {
                    let buffer = &raw mut ring_buffer_uart_8.tx_buf;
                    (*buffer).pop()
                }
            },
            _ => Err(()),
        }
    }
    
}

//// end of the curse

fn interruption_handler(handle : &mut hal_uart_handle_register_t, event : hal_uart_irq_t) {
    {
        if event == hal_uart_irq_t::HAL_LL_UART_IRQ_RX
        {
            let rd_data : u8;
            if is_buffer_full(handle, event)
            {
                hal_ll_uart_irq_disable( handle, hal_uart_irq_t::HAL_LL_UART_IRQ_RX );
                return;
            }

            rd_data = hal_ll_uart_read( handle );
            buffer_push(handle, event, rd_data );
        }
    }

    {
        // If TX interrupt triggered
        if event == hal_uart_irq_t::HAL_LL_UART_IRQ_TX
        {
            let wr_data : u8;
            
            match buffer_pop(handle, event)
            {
                Ok(data) => wr_data = data,
                Err(_) => {
                    hal_ll_uart_irq_disable( handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX );
                    return;
                },
            }
            hal_ll_uart_write(handle, wr_data);

            if is_buffer_empty(handle, event)
            {
                hal_ll_uart_irq_disable( handle, hal_uart_irq_t::HAL_LL_UART_IRQ_TX );
            }
        }
    }
}