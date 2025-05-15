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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum hal_ll_core_irq_priority_levels
{
    HAL_LL_IVT_PRIORITY_LEVEL_0,
    HAL_LL_IVT_PRIORITY_LEVEL_1,
    HAL_LL_IVT_PRIORITY_LEVEL_2,
    HAL_LL_IVT_PRIORITY_LEVEL_3,
    HAL_LL_IVT_PRIORITY_LEVEL_4,
    HAL_LL_IVT_PRIORITY_LEVEL_5,
    HAL_LL_IVT_PRIORITY_LEVEL_6,
    HAL_LL_IVT_PRIORITY_LEVEL_7,
    HAL_LL_IVT_PRIORITY_LEVEL_8,
    HAL_LL_IVT_PRIORITY_LEVEL_9,
    HAL_LL_IVT_PRIORITY_LEVEL_10,
    HAL_LL_IVT_PRIORITY_LEVEL_11,
    HAL_LL_IVT_PRIORITY_LEVEL_12,
    HAL_LL_IVT_PRIORITY_LEVEL_13,
    HAL_LL_IVT_PRIORITY_LEVEL_14,
    HAL_LL_IVT_PRIORITY_LEVEL_15
}

#[inline(always)]
pub fn hal_ll_core_irq(irq_val : u8) -> u8 { irq_val - 16 }

pub const HAL_LL_CORE_IRQ_MASK : u8 = 0x1F;
pub const HAL_LL_CORE_LOW_NIBBLE : u8 = 0xF;
pub const HAL_LL_CORE_HIGH_NIBBLE : u8 = 0xF0;
pub const HAL_LL_CORE_IVT_INT_MEM_MANAGE : u8 = 4;
pub const HAL_LL_CORE_IVT_INT_BUS_FAULT : u8 = 5;
pub const HAL_LL_CORE_IVT_INT_USAGE_FAULT : u8 = 6;
pub const HAL_LL_CORE_IVT_INT_SYS_TICK : u8 = 15;
pub const HAL_LL_CORE_IVT_TICKINT_BIT : u8 = 1;
pub const HAL_LL_CORE_IVT_MEMFAULTENA_BIT : u8 = 16;
pub const HAL_LL_CORE_IVT_BUSFAULTENA_BIT : u8 = 17;
pub const HAL_LL_CORE_IVT_USGFAULTENA_BIT : u8 = 18;
pub const HAL_LL_CORE_SCB_SHCRS : u32 = 0xE000ED24;
pub const HAL_LL_CORE_STK_CTRL : u32 = 0xE000E010;
pub const HAL_LL_CORE_NVIC_ISER_0 : u32 = 0xE000E100;
pub const HAL_LL_CORE_NVIC_ISER_1 : u32 = 0xE000E104;
pub const HAL_LL_CORE_NVIC_ISER_2 : u32 = 0xE000E108;
pub const HAL_LL_CORE_NVIC_ICER_0 : u32 = 0xE000E180;
pub const HAL_LL_CORE_NVIC_ICER_1 : u32 = 0xE000E184;
pub const HAL_LL_CORE_NVIC_ICER_2 : u32 = 0xE000E188;
pub const HAL_LL_CORE_NVIC_IPR_0 : u32 = 0xE000E400;
pub const HAL_LL_CORE_NVIC_SCB_SHPR1 : u32 = 0xE000ED18;
pub const HAL_LL_CORE_NVIC_SCB_SHPR2 : u32 = 0xE000ED1C;
pub const HAL_LL_CORE_NVIC_SCB_SHPR3 : u32 = 0xE000ED20;
