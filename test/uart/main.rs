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

use drv_name::*;
use drv_uart::*;
use system::*;

const tx_pin: pin_name_t = GPIO_C6;
const rx_pin: pin_name_t = GPIO_C7;
const tx_pin1: pin_name_t = GPIO_D8;
const rx_pin1: pin_name_t = GPIO_D9;

#[unsafe(no_mangle)]
fn main() -> ! {
    let mut uart : uart_t = uart_t::default();
    let mut uart1 : uart_t = uart_t::default();

    let mut uart_config : uart_config_t = uart_config_t::default();
    let mut uart1_config : uart_config_t = uart_config_t::default();


    uart_config.rx = rx_pin;
    uart_config.tx = tx_pin;
    uart1_config.rx = rx_pin1;
    uart1_config.tx = tx_pin1;

    uart_open(&mut uart, uart_config);
    uart_open(&mut uart1, uart1_config);

    uart_set_baud(&mut uart, 115200);
    uart_set_baud(&mut uart1, 9600);

    uart_set_blocking(&mut uart, false);

    let mut buffer : [u8; 255] = [0; 255];
    let mut write_buff: [u8; 11] = [0x63, 0x6F, 0x64, 0x65, 0x20, 0x6C, 0x79, 0x6F, 0x6B, 0x6F, 13];
    let mut read_data_size: usize = 0;

    match uart_read(&mut uart, &mut buffer, 10) {
            Ok(data_len) => read_data_size = data_len,
            Err(_) => read_data_size = 0,
        }

    loop {
        match uart_read(&mut uart, &mut buffer, 255) {
            Ok(data_len) => read_data_size = data_len,
            Err(_) => read_data_size = 0,
        }
        //hal_uart_write(&mut uart, &mut write_buff, 10);
        uart_print(&mut uart1, "code lyoko");
        Delay_ms(1000);
        uart_println(&mut uart1, " - code xana");
        Delay_ms(1000);
        uart_write(&mut uart1, &mut buffer, read_data_size);
    }
}