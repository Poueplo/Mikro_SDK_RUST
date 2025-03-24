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

/* Low nibble macros */

pub const HAL_LL_NIBBLE_LOW_8BIT: u8 = 0xF;
pub const HAL_LL_NIBBLE_LOW_16BIT: u16 = 0xFF;
pub const HAL_LL_NIBBLE_LOW_32BIT: u32 = 0xFFFF;

/* High nibble macros */

pub const HAL_LL_NIBBLE_HIGH_8BIT: u8 = 0xF0;
pub const HAL_LL_NIBBLE_HIGH_16BIT: u16 = 0xFF00;
pub const HAL_LL_NIBBLE_HIGH_32BIT: u32 = 0xFFFF0000;

/// Clears one bit in a register
pub fn clear_reg_bit(reg: &mut u32, bit: u32) 
{
    *reg &= !(1 << bit);
}

/// Sets one bit in a register
pub fn set_reg_bit(reg: &mut u32, bit: u32)
{
    *reg |= 1 << bit;
}

/// Returns value of one bit in a register
pub fn check_reg_bit(reg: u32, bit: u32) -> u32 
{
    reg & (1 << bit)
}