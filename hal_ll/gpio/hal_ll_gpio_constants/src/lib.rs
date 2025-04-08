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

pub const GPIO_PIN_MASK_LOW: u16 = 0x00FF;
pub const GPIO_PIN_MASK_HIGH: u16 = 0xFF00;
pub const GPIO_PIN_MASK_ALL: u16 = 0xFFFF;

pub const GPIO_AF_MASK: u32 = 0x0F;
pub const GPIO_PIN_NAME_MASK: u32 = 0xFF;

pub const GPIO_OSPEEDER_OSPEEDR0 : u32 = 0x03;
pub const GPIO_MODER_MODER0 : u32 = 0x03;
pub const GPIO_OTYPER_OT_0 : u32 = 0x01;
pub const GPIO_PUPDR_PUPDR0 : u32 = 0x03;

pub const GPIO_CFG_MODE_ANALOG : u32 = 0x1;
pub const GPIO_CFG_MODE_INPUT : u32 = 0x2;
pub const GPIO_CFG_MODE_OUTPUT : u32 = 0x4;
pub const GPIO_CFG_MODE_ALT_FUNCTION : u32 = 0x8;
pub const GPIO_CFG_OTYPE_PP : u32 = 0x10;
pub const GPIO_CFG_OTYPE_OD : u32 = 0x20;
pub const GPIO_CFG_PULL_NO : u32 = 0x40;
pub const GPIO_CFG_PULL_UP : u32 = 0x80;
pub const GPIO_CFG_PULL_DOWN : u32 = 0x100;

//TODO adapt per CPU
pub const GPIO_CFG_SPEED_LOW : u32 = 0x0;
pub const GPIO_CFG_SPEED_MEDIUM : u32 = 0x200;
pub const GPIO_CFG_SPEED_HIGH : u32 = 0x400;
pub const GPIO_CFG_SPEED_VERY_HIGH : u32 = 0x800;

pub const GPIO_CFG_SPEED_MAX : u32 = 0x80000;



pub const GPIO_CFG_DIGITAL_OUTPUT : u32 = GPIO_CFG_MODE_OUTPUT | GPIO_CFG_SPEED_MAX | GPIO_CFG_OTYPE_PP;
pub const GPIO_CFG_DIGITAL_INPUT : u32 = GPIO_CFG_MODE_INPUT | GPIO_CFG_PULL_NO;
pub const GPIO_CFG_ANALOG_INPUT : u32 = GPIO_CFG_MODE_ANALOG | GPIO_CFG_PULL_NO;