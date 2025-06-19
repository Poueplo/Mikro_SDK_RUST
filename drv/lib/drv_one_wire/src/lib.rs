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

use hal_one_wire::*;

pub use hal_one_wire::hal_one_wire_t as one_wire_t;
pub use hal_one_wire::HAL_ONE_WIRE_ERROR as ONE_WIRE_ERROR;
pub use hal_one_wire::hal_one_wire_rom_address_t as one_wire_rom_address_t;


type Result<T> = core::result::Result<T, HAL_ONE_WIRE_ERROR>;


pub fn one_wire_open(obj : &mut one_wire_t ) -> Result<()> {
    hal_one_wire_open( obj )
}

pub fn one_wire_reset(obj : &mut one_wire_t ) -> Result<()> {
        hal_one_wire_reset(obj)

}

pub fn one_wire_read_rom(obj : &mut one_wire_t , device_rom_address : &mut one_wire_rom_address_t) -> Result<()> {
    hal_one_wire_read_rom(obj, device_rom_address) 
}

pub fn one_wire_skip_rom(obj : &mut one_wire_t) -> Result<()> {
    hal_one_wire_skip_rom(obj)
}

pub fn one_wire_match_rom(obj : &mut one_wire_t , device_rom_address : &mut one_wire_rom_address_t) -> Result<()> {
   hal_one_wire_match_rom(obj, device_rom_address)
}

pub fn one_wire_search_first_device(obj : &mut one_wire_t , one_wire_device_list : &mut one_wire_rom_address_t) -> Result<()> {
    hal_one_wire_search_first_device(obj, one_wire_device_list)
}

pub fn one_wire_search_next_device(obj : &mut one_wire_t , one_wire_device_list : &mut one_wire_rom_address_t) -> Result<()> {
    hal_one_wire_search_first_device(obj, one_wire_device_list)
}

pub fn one_wire_write_byte(obj : &mut one_wire_t , write_data_buffer : &[u8], write_data_length : usize) -> Result<()> {
        hal_one_wire_write_byte(obj, write_data_buffer, write_data_length)
}

pub fn one_wire_read_byte(obj : &mut one_wire_t , read_data_buffer : &mut[u8], read_data_length : usize) -> Result<()> {
        hal_one_wire_read_byte(obj, read_data_buffer, read_data_length)
}