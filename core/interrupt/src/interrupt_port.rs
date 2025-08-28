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

#![allow(non_snake_case)]



use bit_control::*;
use crate::interrupt_defines::*;

#[allow(unused_comparisons)]
pub fn hal_ll_core_port_nvic_enable_irq(IRQn : u8)
{
    match IRQn
    {
        HAL_LL_CORE_IVT_INT_MEM_MANAGE  => {
                set_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_MEMFAULTENA_BIT as u32 );
            },
        HAL_LL_CORE_IVT_INT_BUS_FAULT => {
                set_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_BUSFAULTENA_BIT as u32 );
            },
        HAL_LL_CORE_IVT_INT_USAGE_FAULT => {
                set_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_USGFAULTENA_BIT as u32 );
            },
        HAL_LL_CORE_IVT_INT_SYS_TICK => {
                set_reg_bit( HAL_LL_CORE_STK_CTRL, HAL_LL_CORE_IVT_TICKINT_BIT as u32 );
            },
        _ => (), // If none of the above, exit switch
        }

    // General exceptions
    if IRQn >= 80
    {
        set_reg_bit( HAL_LL_CORE_NVIC_ISER_2, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    } else if IRQn >= 48 {
        set_reg_bit( HAL_LL_CORE_NVIC_ISER_1, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    } else if IRQn >= 16 {
        set_reg_bit( HAL_LL_CORE_NVIC_ISER_0, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    }
}

#[allow(unused_comparisons)]
pub fn hal_ll_core_port_nvic_disable_irq(IRQn : u8)
{
    match IRQn
    {
        HAL_LL_CORE_IVT_INT_MEM_MANAGE => {
                clear_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_MEMFAULTENA_BIT as u32 );
            },
            
        HAL_LL_CORE_IVT_INT_BUS_FAULT => {
                clear_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_BUSFAULTENA_BIT as u32 );
            },
            
        HAL_LL_CORE_IVT_INT_USAGE_FAULT => {
                clear_reg_bit( HAL_LL_CORE_SCB_SHCRS, HAL_LL_CORE_IVT_USGFAULTENA_BIT as u32 );
            },
            
        HAL_LL_CORE_IVT_INT_SYS_TICK => {
                clear_reg_bit( HAL_LL_CORE_STK_CTRL, HAL_LL_CORE_IVT_TICKINT_BIT as u32 );
            },
            
        _ => (), // If none of the above, exit switch
    }

    // General exceptions
    if IRQn >= 80 
    {
        set_reg_bit( HAL_LL_CORE_NVIC_ICER_2, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    } else if IRQn >= 48 {
        set_reg_bit( HAL_LL_CORE_NVIC_ICER_1, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    } else if IRQn >= 16 {
        set_reg_bit( HAL_LL_CORE_NVIC_ICER_0, ( /*( hal_ll_core_irq( IRQn ) )*/ IRQn & HAL_LL_CORE_IRQ_MASK ) as u32 );
    }
}


#[allow(unused_assignments)]
pub fn hal_ll_core_port_nvic_set_priority_irq(IRQn_in : u8, IRQn_priority : hal_ll_core_irq_priority_levels )
{
    let reg : *mut u32;
    let mut tmp_shift: u32 = 0;
    let IRQn: u8 = IRQn_in + 16;

    if IRQn > 15
    {
        reg = (HAL_LL_CORE_NVIC_IPR_0 + ( ( hal_ll_core_irq( IRQn ) ) as u32 >> 2 )) as *mut u32;
        tmp_shift = ( ( ( hal_ll_core_irq( IRQn ) ) as u32 % 4 )  << 3 ) + 4;
    } else if ( IRQn > 3 ) & ( IRQn <= 15 ) {
        reg = (HAL_LL_CORE_NVIC_SCB_SHPR1 + ( IRQn as u32 / 4 ) - 1) as *mut u32;
        tmp_shift = ( ( IRQn as u32 % 4 ) << 3 ) + 4;
    } else {
        return;
    }

    unsafe{
        if (IRQn_priority as u8 & HAL_LL_CORE_LOW_NIBBLE) > 0 {
            *reg &= !(( HAL_LL_CORE_LOW_NIBBLE as u32) << tmp_shift);
            *reg |= (IRQn_priority as u32) << tmp_shift;
        } else {
            *reg &= !(( HAL_LL_CORE_LOW_NIBBLE as u32) << tmp_shift );
            *reg |= (IRQn_priority as u32) << ( tmp_shift - 4 );
        }
    }
    
}