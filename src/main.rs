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

use cortex_m_rt::entry;
// The runtime
use panic_halt;


//use drv_digital_in::*;
use drv_digital_out::*;
use drv_port::*;
use drv_name::*;
use system::*;

use hal_spi_master::*;

const pin_sck: pin_name_t = GPIO_A5;
const pin_miso: pin_name_t = GPIO_A6;
const pin_mosi: pin_name_t = GPIO_B5;
const pin_cs: pin_name_t = GPIO_B4;
const pin_hld: pin_name_t = GPIO_E13;

const error_port: port_name_t = GPIO_PORT_D;


#[entry]
fn main() -> ! {

    system_init();

    let mut error: port_t = port_t::default();
    let mut error_output : u16 = 0;
    let mut data_buff: [u8; 16] = [0; 16];
    let mut spi_write_buff: [u8; 14] = [0x02, 0x00, 0x00, 0x00, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x6C, 0x79, 0x6F, 0x6B, 0x6F];
    let mut spi_read_order: [u8; 4] = [0x03, 0x00, 0x00, 0x00];

    let mut hld: digital_out_t = digital_out_t::default();
    digital_out_init(&mut hld , pin_hld );

    digital_out_high(&mut hld);

    port_init(&mut error, error_port, 0xFFFF, gpio_direction_t::GPIO_DIGITAL_OUTPUT);

    let mut spi : hal_spi_master_t = hal_spi_master_t::default(); 
    let mut spi_config : hal_spi_master_config_t = hal_spi_master_config_t::default();

    spi.config.sck = pin_sck;
    spi.config.miso = pin_miso;
    spi.config.mosi = pin_mosi;

    hal_spi_master_set_chip_select_polarity(hal_spi_master_chip_select_polarity_t::SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_LOW);
    hal_spi_master_open(&mut spi, true);
    //hal_spi_master_set_mode(&mut spi, spi_config);

    spi_config.default_write_data = 0x55;
    //hal_spi_master_set_default_write_data(&mut spi, spi_config);

    
    hal_spi_master_select_device(pin_cs);
    hal_spi_master_write(&mut spi, &mut spi_write_buff, 14);
    hal_spi_master_deselect_device(pin_cs);
    delay_1ms();
    hal_spi_master_select_device(pin_cs);
    hal_spi_master_write(&mut spi, &mut spi_read_order, 4);
    hal_spi_master_read(&mut spi, &mut data_buff, 10);
    hal_spi_master_deselect_device(pin_cs);
    delay_1ms();
    hal_spi_master_select_device(pin_cs);
    hal_spi_master_write_then_read(&mut spi, &mut spi_read_order, 4, &mut data_buff, 10);
    hal_spi_master_deselect_device(pin_cs);

    hal_spi_master_close(&mut spi);

    loop {}
}
