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
//#![allow(unused_imports)]

// The runtime
use panic_halt;


use drv_spi_master::*;
use drv_name::*;
use system::*;

const pin_sck: pin_name_t = GPIO_C10;
const pin_miso: pin_name_t = GPIO_C11;
const pin_mosi: pin_name_t = GPIO_C12;
const pin_cs: pin_name_t = GPIO_C13;


#[unsafe(no_mangle)]
fn main() -> ! {

    let mut data_buff: [u8; 16] = [0; 16];
    let mut spi_write_buff: [u8; 14] = [0x02, 0x00, 0x00, 0x00, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x6C, 0x79, 0x6F, 0x6B, 0x6F];
    let mut spi_read_order: [u8; 4] = [0x03, 0x00, 0x00, 0x00];

    let mut spi : spi_master_t = spi_master_t::default(); 
    let mut spi_config : spi_master_config_t = spi_master_config_t::default();

    spi_config.sck = pin_sck;
    spi_config.miso = pin_miso;
    spi_config.mosi = pin_mosi;

    spi_master_set_chip_select_polarity(spi_master_chip_select_polarity_t::SPI_MASTER_CHIP_SELECT_POLARITY_ACTIVE_LOW);
    spi_master_open(&mut spi, spi_config);
    //hal_spi_master_set_mode(&mut spi, spi_config);

    spi_config.default_write_data = 0x55;
    //hal_spi_master_set_default_write_data(&mut spi, spi_config);

    
    spi_master_select_device(pin_cs);
    spi_master_write(&mut spi, &mut spi_write_buff, 14);
    spi_master_deselect_device(pin_cs);
    delay_1ms();
    spi_master_select_device(pin_cs);
    spi_master_write(&mut spi, &mut spi_read_order, 4);
    spi_master_read(&mut spi, &mut data_buff, 10);
    spi_master_deselect_device(pin_cs);
    delay_1ms();
    spi_master_select_device(pin_cs);
    spi_master_write_then_read(&mut spi, &mut spi_read_order, 4, &mut data_buff, 10);
    spi_master_deselect_device(pin_cs);

    spi_master_close(&mut spi);

   loop{}
}