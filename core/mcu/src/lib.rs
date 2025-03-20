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
//for STM32F429ZI

pub const AHB1_BASE_ADDR: u32 = 0x4002_0000;

//present in hal_ll_gpio_port
pub const GPIOA_BASE_ADDR: u32 = AHB1_BASE_ADDR;
pub const GPIOB_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x0400;
pub const GPIOC_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x0800;
pub const GPIOD_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x0C00;
pub const GPIOE_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x1000;
pub const GPIOF_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x1400;
pub const GPIOG_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x1800;
pub const GPIOH_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x1C00;
pub const GPIOI_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x2000;
pub const GPIOJ_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x2400;
pub const GPIOK_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x2800;


//present in core_header.h
pub const RCC_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x3800;

//present in hal_ll_rcc.h
pub const RCC_AHB1ENR:   u32 = RCC_BASE_ADDR + 0x0030;

pub const PORT_SIZE : u8 = 16;
