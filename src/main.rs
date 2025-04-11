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


use drv_digital_in::*;
use drv_digital_out::*;
use drv_i2c_master::*;
use drv_name::*;
use system::*;

use hal_ll_spi_master::*;

const pin_scl: pin_name_t = GPIO_B8;
const pin_sda: pin_name_t = GPIO_B9;

const pin_sck: pin_name_t = GPIO_C10;
const pin_miso: pin_name_t = GPIO_C11;
const pin_mosi: pin_name_t = GPIO_C12;
const pin_cs: pin_name_t = GPIO_C13;
const pin_hld: pin_name_t = GPIO_E14;


#[entry]
fn main() -> ! {

    system_init();

    let mut index: u8 = 0;

    let mut data_buff: [u8; 16] = [0; 16];
    let mut write_buf: [u8; 5] = [0x00, 0x55, 0x56, 0x57, 0x58];
    let mut write_buf2: [u8; 5] = [0x00, 0x65, 0x66, 0x67, 0x68];
    let mut spi_write_buff: [u8; 14] = [0x02, 0x00, 0x00, 0x00, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x6C, 0x79, 0x6F, 0x6B, 0x6F];
    let mut spi_read_order: [u8; 4] = [0x03, 0x00, 0x00, 0x00];

    let mut sc: digital_out_t = digital_out_t::default();
    let mut hld: digital_out_t = digital_out_t::default();
    digital_out_init(&mut sc , pin_cs );
    digital_out_init(&mut hld , pin_cs );

    digital_out_high(&mut hld);
    digital_out_high(&mut sc);

    let mut spi : hal_ll_spi_master_handle_register_t = hal_ll_spi_master_register_handle(pin_sck, pin_miso, pin_mosi, &mut index).ok().unwrap();

    hal_ll_spi_master_set_default_write_data(&mut spi, 0);
    hal_ll_spi_master_set_mode(&mut spi, hal_ll_spi_master_mode_t::SPI_MASTER_MODE_0);
    hal_ll_spi_master_set_speed(&mut spi, 100000);


    digital_out_low(&mut sc);
    hal_ll_spi_master_write(&mut spi, &mut spi_write_buff, 14);
    digital_out_high(&mut sc);
    delay_1ms();
    digital_out_low(&mut sc);
    hal_ll_spi_master_write(&mut spi, &mut spi_read_order, 4);
    hal_ll_spi_master_read(&mut spi, &mut data_buff, 10);
    digital_out_high(&mut sc);
    delay_1ms();
    digital_out_low(&mut sc);
    hal_ll_spi_master_write_then_read(&mut spi, &mut spi_read_order, 4, &mut data_buff, 10);
    digital_out_high(&mut sc);


    let mut scl: digital_out_t = digital_out_t::default();
    let mut sda: digital_in_t = digital_in_t::default();

    digital_out_init(&mut scl , pin_scl );
    digital_in_init(&mut sda , pin_sda);
    digital_out_high(&mut scl );
    while digital_in_read(&mut sda ).ok().unwrap() == 0
    {
        digital_out_low(&mut scl);
        Delay_ms(1);
        digital_out_high(&mut scl);
        Delay_ms(1);
    }


    let mut i2c: i2c_master_t = i2c_master_t::default();
    let mut i2c_config: i2c_master_config_t = i2c_master_config_t::default();

    i2c_config.sda = pin_sda;
    i2c_config.scl = pin_scl;

    i2c_master_open(&mut i2c, i2c_config);

    i2c_master_set_timeout(&mut i2c, 0);
    i2c_master_set_slave_address(&mut i2c, 0x50);
    i2c_master_set_speed(&mut i2c, i2c_master_speed_t::I2C_MASTER_SPEED_400K);


    hal_ll_spi_master_close(&mut spi);
    

    i2c_master_write(&mut i2c, &mut write_buf, 5);
    Delay_ms(10);

    i2c_master_write_then_read(&mut i2c, &mut write_buf, 1, &mut data_buff, 3);
    i2c_master_read(&mut i2c, &mut data_buff, 1);

    i2c_master_write(&mut i2c, &mut write_buf2, 5);
    Delay_ms(10);

    i2c_master_write_then_read(&mut i2c, &mut write_buf2, 1, &mut data_buff, 1);
    i2c_master_read(&mut i2c, &mut data_buff, 3);


    i2c_master_close(&mut i2c);

    loop {}
}
