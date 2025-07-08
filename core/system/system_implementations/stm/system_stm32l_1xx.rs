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
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(const_item_mutation)]
//#![allow(unused)]

use core::arch::asm;

mod core_header;

pub use mcu_header::{RCC_TypeDef, RCC_BASE};
use mcu_header::{ RCC_CR_HSION_Pos, RCC_CR_HSEBYP_Pos, 
    FLASH_TypeDef, FLASH_R_BASE, RCC_CFGR_HPRE_Msk, RCC_CFGR_HPRE_Pos, RCC_CFGR_PPRE1_Msk, 
    RCC_CFGR_PPRE1_Pos, RCC_CFGR_PPRE2_Msk, RCC_CFGR_PPRE2_Pos, 
    FLASH_ACR_PRFTEN_Pos, FLASH_ACR_ACC64_Pos, FLASH_ACR_LATENCY_Pos, 
    RCC_CR_HSIRDY_Pos, RCC_CR_HSEON_Pos, RCC_CR_HSERDY_Pos, RCC_CR_PLLON_Pos, RCC_CR_PLLRDY_Pos, RCC_CFGR_SWS_Msk};

use common_header::*;
use core_header::*;
use system_reset::*;

const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;
const SCB_AIRCR_SYSRESETREQ : u32 = 2;

//VCORE = 1.2
const VC_1200 : u32 = 0;
//VCORE = 1.2
const VC_1500 : u32 = 1;
//VCORE = 1.8
const VC_1800 : u32 = 2;

pub struct RCC_ClocksTypeDef
{
    pub SYSCLK_Frequency    : u32,
    pub HCLK_Frequency      : u32,
    pub PCLK1_Frequency     : u32,
    pub PCLK2_Frequency     : u32,
    pub ADCCLK_Frequency    : u32,
}

impl Default for RCC_ClocksTypeDef {
    fn default() -> Self {
        Self {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0,
            ADCCLK_Frequency    : 0
        }
    }
}


const APBAHBPrescTable : [u8; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];
const PLLMulTable      : [u8; 9]  = [3, 4, 6, 8, 12, 16, 24, 32, 48];

//*****************************************************************************
//
// Resets the device.
//
// This function will perform a software reset of the entire device. The
// processor and all peripherals are reset and all device registers will
// return to their default values (with the exception of the reset cause
// register, which will maintain its current value but have the software reset
// bit set as well).
//
// \return This function does not return anything.
//
//*****************************************************************************
pub fn system_reset()
{
    // Perform a software reset request. This will cause the device to reset,
    // no further code is executed.

    reg_value_clear_set(ADDRESS_SCB_AIRCR as *mut u32, 0x05FA0000 | (1u32 << SCB_AIRCR_SYSRESETREQ));

    // The device should have reset, so this should never be reached. Just in
    // case, loop forever.

    loop{};
}

/*
   @brief Resets the RCC clock configuration to the default reset state.
   @note   The default reset state of the clock configuration is given below:
              - HSI ON and used as system clock source
              - HSE, PLL and PLLI2S OFF
              - AHB, APB1 and APB2 prescaler set to 1.
              - CSS, MCO1 and MCO2 OFF
              - All interrupts disabled (not used)
   @note   This function doesn't modify the configuration of the
              - Peripheral clocks
              - LSI, LSE and RTC clocks
   @param None
   @retval None
*/

pub fn system_clock_set_default() {
    unsafe{
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        /* Set HSION bit */
        reg_value_set_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_HSION_Pos);

        /*< Reset SW[1:0], HPRE[3:0], PPRE1[2:0], PPRE2[2:0], MCOSEL[2:0] and MCOPRE[2:0] bits */
        reg_value_clear_mask(&(*rcc_ptr).CFGR as *const u32 as *mut u32, 0xF0FFC00C);

        /*< Reset HSION, HSEON, CSSON and PLLON bits */
        reg_value_clear_mask(&(*rcc_ptr).CR as *const u32 as *mut u32, 0xEEFEFFFE);
        
        /*< Reset HSEBYP bit */
        reg_value_clear_bit(&(*rcc_ptr).CR as *const u32  as *mut u32, RCC_CR_HSEBYP_Pos);

        /*< Reset PLLSRC, PLLMUL[3:0] and PLLDIV[1:0] bits */
        reg_value_clear_mask(&(*rcc_ptr).CFGR as *const u32  as *mut u32, 0xFF02FFFF);

        /*< Disable all interrupts */
        reg_value_clear(&(*rcc_ptr).CIR as *const u32 as *mut u32);
    }
}

pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    unsafe{
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        let mut tmp: u32;
        let mut presc: u8;

        rcc_clocks.HCLK_Frequency = FOSC_KHZ_VALUE * 1000;

        /*------ Compute HCLK, PCLK1, and PCLK2 clocks frequencies ------*/

        /* Get HCLK prescaler */
        tmp = reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_HPRE_Msk;
        tmp >>= RCC_CFGR_HPRE_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        /* HCLK clock frequency */
        rcc_clocks.SYSCLK_Frequency = rcc_clocks.HCLK_Frequency << presc;

        /* Get PCLK1 prescaler */
        tmp = reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_PPRE1_Msk;
        tmp >>= RCC_CFGR_PPRE1_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        /* PCLK1 clock frequency */
        rcc_clocks.PCLK1_Frequency = rcc_clocks.HCLK_Frequency >> presc;

        /* Get PCLK2 prescaler */
        tmp = reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_PPRE2_Msk;
        tmp >>= RCC_CFGR_PPRE2_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        /* PCLK2 clock frequency */
        rcc_clocks.PCLK2_Frequency = rcc_clocks.HCLK_Frequency >> presc;
    }
}

#[unsafe(no_mangle)]
pub fn system_init() {
    unsafe{
        
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        let flash_ptr : *mut FLASH_TypeDef = FLASH_R_BASE as *mut FLASH_TypeDef;

        if VALUE_SVCORE == VC_1200 {
            if FOSC_KHZ_VALUE > 2000 {
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_ACC64_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_PRFTEN_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            } else {
                reg_value_clear_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            }
        } 
        else if VALUE_SVCORE == VC_1500 {
            if FOSC_KHZ_VALUE > 8000 {
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_ACC64_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_PRFTEN_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            } else {
                reg_value_clear_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            }
        }
        else if VALUE_SVCORE == VC_1800 {
            if FOSC_KHZ_VALUE > 16000 {
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_ACC64_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_PRFTEN_Pos);
                reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            } else {
                reg_value_clear_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_Pos);
            }
        } 

        system_clock_set_default();

        reg_value_set(&(*rcc_ptr).CFGR as *const u32 as *mut u32, VALUE_RCC_CFGR);             /* set clock configuration register */
        reg_value_set(&(*rcc_ptr).CR as *const u32 as *mut u32, VALUE_RCC_CR & 0x000FFFFF);    /* do not start PLLs yet */

        if VALUE_RCC_CR & (1 << RCC_CR_HSION_Pos) != 0 { /* if HSI enabled */
            while reg_value_get_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_HSIRDY_Pos) == 0 {
                /* Wait for HSIRDY = 1 (HSI is ready) */
            }
        }      

        if VALUE_RCC_CR & (1 << RCC_CR_HSEON_Pos) != 0 { /* if HSE enabled */
            while reg_value_get_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_HSERDY_Pos) == 0 {
                /* Wait for HSERDY = 1 (HSE is ready) */
            }
        }

        if VALUE_RCC_CR & (1 << RCC_CR_PLLON_Pos) != 0 { /* if PLL enabled */
            reg_value_set_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_PLLON_Pos); /* PLL On */
            while reg_value_get_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_PLLRDY_Pos) == 0 {
                /* Wait for PLL1RDY = 1 (PLL is ready) */
            }
        }

        /* Wait till SYSCLK is stabilized (depending on selected clock) */    
        while (reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_SWS_Msk) != ((VALUE_RCC_CFGR << 2) & RCC_CFGR_SWS_Msk) {
        }
    }
}

// ==================== DELAYS ======================

fn get_clock_value(_clock : u32) -> u32
{
    _clock / 1000
}

#[inline(never)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ramfunc")]

pub fn Delay_Cyc(mut cycle_num : u32)
{
    unsafe
    {
        asm!(
             
            "2:",
            "sub {0}, #1",
            "nop",
            "cmp {0}, #0",
            "ble 2f",
            "B 2b",
            "2:",
            inout(reg) cycle_num,
        );
    }
}

#[inline(never)]
pub fn Delay_us(time_us: u32) 
{
    /*
     * Delay for STM32L152RE - default NECTO setup
     */
    Delay_Cyc(time_us * get_clock_value(FOSC_KHZ_VALUE)  / 8 );
}


#[inline(never)]
pub fn Delay_ms(time_ms: u32) 
{

    /*
     * Delay for STM32L152RE - default NECTO setup
     */

    Delay_us(time_ms * 1000);
}

#[inline(never)]
pub fn Delay_Advanced_ms(time_ms: u32, current_fosc_kHz: u32) 
{

}

// Functions for specific delays in microseconds
pub fn delay_1us() {
    Delay_us(1);
}

pub fn delay_5us() {
    Delay_us(5);
}

pub fn delay_6us() {
    Delay_us(6);
}

pub fn delay_9us() {
    Delay_us(9);
}

pub fn delay_10us() {
    Delay_us(10);
}

pub fn delay_22us() {
    Delay_us(22);
}

pub fn delay_50us() {
    Delay_us(50);
}

pub fn delay_55us() {
    Delay_us(55);
}

pub fn delay_60us() {
    Delay_us(60);
}

pub fn delay_64us() {
    Delay_us(64);
}

pub fn delay_70us() {
    Delay_us(70);
}

pub fn delay_80us() {
    Delay_us(78);
}

pub fn delay_410us() {
    Delay_us(410);
}

pub fn delay_480us() {
    Delay_us(480);
}

pub fn delay_500us() {
    Delay_us(498);
}

pub fn delay_5500us() {
    Delay_us(5500);
}

// Functions for delays in milliseconds
pub fn delay_1ms() {
    Delay_ms(1);
}

pub fn delay_5ms() {
    Delay_ms(5);
}

pub fn delay_8ms() {
    Delay_ms(8);
}

pub fn delay_10ms() {
    Delay_ms(10);
}

pub fn delay_100ms() {
    Delay_ms(100);
}

pub fn delay_1sec() {
    Delay_ms(1000);
}

pub fn get_gpio_clock<'a>() -> &'a mut u32 {
    unsafe {
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        &mut (*rcc_ptr).AHBENR
    }
}