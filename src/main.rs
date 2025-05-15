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
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

// The runtime
use panic_halt;

use drv_port::*;
use drv_name::*;
use system::*;
use interrupt::interrupt_helper::*;
use ring::ring_buf8_t;
use hal_ll_uart::*;

const port_out: port_name_t = GPIO_PORT_E;
const port_interrupt: port_name_t = GPIO_PORT_B;
const tx_pin: pin_name_t = GPIO_C6;
const rx_pin: pin_name_t = GPIO_C7;
const tx_pin1: pin_name_t = GPIO_B6;
const rx_pin1: pin_name_t = GPIO_B7;

static mut toggle : bool = false;
static mut ring_buffer_rx : ring_buf8_t<255> = ring_buf8_t::<255>::init();
static mut ring_buffer_tx : ring_buf8_t<255> = ring_buf8_t::<255>::init();

#[unsafe(no_mangle)]
fn main() -> ! {

    let mut output1: port_t = port_t::default();
    let mut output2: port_t = port_t::default();
    port_init(&mut output1 , port_out, 0xFFFF, gpio_direction_t::GPIO_DIGITAL_OUTPUT);
    port_init(&mut output2 , port_interrupt, 0xFFFF, gpio_direction_t::GPIO_DIGITAL_OUTPUT);

    let mut value : u16;

    let mut uart : hal_ll_uart_handle_register_t;
    let mut uart1 : hal_ll_uart_handle_register_t;
    let mut module_index : u8 = 0;

    uart1 = hal_ll_uart_register_handle(tx_pin1, rx_pin1, &mut module_index).ok().unwrap();
    uart = hal_ll_uart_register_handle(tx_pin, rx_pin, &mut module_index).ok().unwrap();

    hal_ll_uart_register_irq_handler(interruption_handler);
    
    hal_ll_module_configure_uart(&mut uart);
    hal_ll_module_configure_uart(&mut uart1);
    
    
    hal_ll_uart_set_baud(&mut uart1, 9600);
    //hal_ll_uart_set_parity(&mut uart, hal_ll_uart_parity_t::HAL_LL_UART_PARITY_ODD);

    hal_ll_core_disable_interrupts();
    hal_ll_uart_irq_enable(&mut uart, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX);
    hal_ll_core_enable_interrupts();

    hal_ll_uart_close(&mut uart1);

    let mut buffer : [u8; 255] = [0; 255];

    loop {
        hal_ll_core_disable_interrupts();
        let mut data_length: u8 = 0;
        unsafe {
            {
                let buffer_rx = &raw mut ring_buffer_rx;
                let mut index: usize = 0;
                while!(*buffer_rx).is_empty() && index < buffer.len() {
                    buffer[index] = (*buffer_rx).pop().ok().unwrap() as u8;
                    index += 1;
                    data_length += 1;
                }
            }

            {
                if data_length > 0 {
                    let mut index: usize = 0;
                    let buffer_tx = &raw mut ring_buffer_tx;

                    while !(*buffer_tx).is_full() && index < data_length as usize {
                        (*buffer_tx).push(buffer[index]);
                        index += 1;
                    }

                    if index > 0 {
                        hal_ll_uart_irq_enable(&mut uart, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX);
                
                        hal_ll_uart_write(&mut uart, (*buffer_tx).pop().ok().unwrap());
                    }
                }
            }
        }
        hal_ll_core_enable_interrupts();
        //Delay_ms(100);
    }
}

fn interruption_handler(handle : &mut hal_ll_uart_handle_register_t, event : hal_ll_uart_irq_t) {
    unsafe {
        {
            let buffer_rx = &raw mut ring_buffer_rx;
            if event == hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX
            {
                let rd_data : u8;
                if (*buffer_rx).is_full()
                {
                    hal_ll_uart_irq_disable( handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_RX );
                    return;
                }

                rd_data = hal_ll_uart_read( handle );
                (*buffer_rx).push(rd_data );
            }
        }

        {
            // If TX interrupt triggered
            let buffer_tx = &raw mut ring_buffer_tx;
            if event == hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX
            {
                let wr_data : u8;
               
                match (*buffer_tx).pop()
                {
                    Ok(data) => wr_data = data,
                    Err(_) => {
                        hal_ll_uart_irq_disable( handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX );
                        return;
                    },
                }
                hal_ll_uart_write(handle, wr_data);

                if (*buffer_tx).is_empty()
                {
                    hal_ll_uart_irq_disable( handle, hal_ll_uart_irq_t::HAL_LL_UART_IRQ_TX );
                }
            }
        }
    }
}