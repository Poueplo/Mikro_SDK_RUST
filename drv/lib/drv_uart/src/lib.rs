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

use hal_uart::*;
use drv_name::*;

pub use hal_uart::hal_uart_config_t as uart_config_t;
pub use hal_uart::hal_uart_t as uart_t;
pub use hal_uart::HAL_UART_ERROR as DRV_UART_ERROR;
pub use hal_uart::hal_uart_parity_t as uart_parity_t;
pub use hal_uart::hal_uart_data_bits_t as uart_data_bits_t;
pub use hal_uart::hal_uart_stop_bits_t as uart_stop_bits_t;

type Result<T> = core::result::Result<T, DRV_UART_ERROR>;


static mut previous_initialized: uart_t = uart_t{ 
    handle: hal_uart_handle_register_t{ uart_handle: 0, init_ll_state: false },
    config: uart_config_t {
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
    };

fn _acquire( obj: &mut uart_t, obj_open_state: bool) -> Result<()> {
    if obj_open_state && (unsafe{previous_initialized == *obj}) {
        return Err(DRV_UART_ERROR::ACQUIRE_FAIL);
    }

    if unsafe{previous_initialized != *obj} {
        match hal_uart_open( obj, obj_open_state ) {
            Ok(_) => {
                unsafe{previous_initialized = *obj;}
                return Ok(())
            },
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

pub fn uart_open( obj: &mut uart_t, config: uart_config_t ) -> Result<()> {
    obj.config = config;

    match _acquire( obj, true ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn uart_set_baud( obj: &mut uart_t, baud: u32 ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.baud = baud;
    
    match hal_uart_set_baud( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn uart_set_parity( obj: &mut uart_t, parity: uart_parity_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.parity = parity;
    
    match hal_uart_set_parity( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn uart_set_stop_bits( obj: &mut uart_t, stop: uart_stop_bits_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.stop_bits = stop;
    
    match hal_uart_set_stop_bits( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn uart_set_data_bits( obj: &mut uart_t, bits: uart_data_bits_t ) -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.data_bits = bits;
    
    match hal_uart_set_data_bits( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn uart_set_blocking(obj: &mut uart_t, blocking: bool)  -> Result<()> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    obj.config.is_blocking = blocking;
    
    match hal_uart_set_blocking( obj, obj.config ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn uart_read( obj: &mut uart_t, read_data_buf: &mut [u8], len_read_data: usize ) -> Result<usize> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_uart_read( obj, read_data_buf, len_read_data ) {
        Ok(size) => Ok(size),
        Err(e) => return Err(e),
    }
}

pub fn uart_write( obj: &mut uart_t, write_data_buf: &mut [u8], len_write_data: usize ) -> Result<usize> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_uart_write( obj, write_data_buf, len_write_data ) {
        Ok(size) => Ok(size),
        Err(e) => return Err(e),
    }
}

pub fn uart_print( obj: &mut uart_t, write_data_buf: &str) -> Result<usize> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_uart_print( obj, write_data_buf) {
        Ok(size) => Ok(size),
        Err(e) => return Err(e),
    }
}

pub fn uart_println( obj: &mut uart_t, write_data_buf: &str) -> Result<usize> {
    match _acquire( obj, false ) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match hal_uart_println( obj, write_data_buf) {
        Ok(size) => Ok(size),
        Err(e) => return Err(e),
    }
}

pub fn uart_bytes_available(obj: &mut uart_t) -> Result<usize> {
    hal_uart_bytes_available(obj)
}

pub fn uart_clear(obj: &mut uart_t) -> Result<()> {
    hal_uart_clear(obj)
}

pub fn uart_close(obj: &mut uart_t) -> Result<()> {
    match hal_uart_close(obj) {
        Ok(_) => {
            unsafe{previous_initialized = uart_t::default();}
            Ok(())
        },
        Err(e) => Err(e),
    }
}