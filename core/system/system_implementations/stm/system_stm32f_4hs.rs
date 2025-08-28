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
#![allow(unused)]

use core::arch::asm;

mod core_header;

pub use mcu_header::{RCC_TypeDef, RCC_BASE};
use mcu_header::{ RCC_CR_HSION_Pos, RCC_CR_HSEBYP_Pos, PWR_TypeDef, PWR_BASE, 
    FLASH_TypeDef, FLASH_R_BASE, RCC_CFGR_HPRE_Msk, RCC_CFGR_HPRE_Pos, RCC_CFGR_PPRE1_Msk, 
    RCC_CFGR_PPRE1_Pos, RCC_CFGR_PPRE2_Msk, RCC_CFGR_PPRE2_Pos, RCC_APB1ENR_PWREN_Pos, 
    PWR_CR_ODEN_Pos, PWR_CSR_ODRDY_Pos, PWR_CR_ODSWEN_Pos, PWR_CSR_ODSWRDY_Pos, FLASH_ACR_PRFTEN_Pos, 
    FLASH_ACR_ICEN_Pos, FLASH_ACR_LATENCY_8WS, FLASH_ACR_LATENCY_7WS, FLASH_ACR_LATENCY_6WS, FLASH_ACR_LATENCY_5WS, 
    FLASH_ACR_LATENCY_4WS, FLASH_ACR_LATENCY_3WS, FLASH_ACR_LATENCY_2WS, FLASH_ACR_LATENCY_1WS, FLASH_ACR_LATENCY_Msk,
    RCC_CR_HSIRDY_Pos, RCC_CR_HSEON_Pos, RCC_CR_HSERDY_Pos, RCC_CR_PLLON_Pos, RCC_CR_PLLRDY_Pos, RCC_CFGR_SWS_Msk};

use common_header::*;
use core_header::*;
use system_reset::*;


const ADDRESS_SCB_AIRCR     : u32 = 0xE000ED0C;
const SCB_AIRCR_SYSRESETREQ : u8  = 2;

// Voltage range
// 2.7 to 3.6 V
const VR_2700_3600: u32 = 3;
// Voltage range
// 2.4 to 2.7 V
const VR_2400_2700: u32 = 2;
// Voltage range
// 2.1 to 2.4 V
const VR_2100_2400: u32 = 1;
// Voltage range
// 1.8 to 2.1 V
const VR_1800_2100: u32 = 0;
pub struct RCC_ClocksTypeDef
{
    pub SYSCLK_Frequency    : u32,
    pub HCLK_Frequency      : u32,
    pub PCLK1_Frequency     : u32,
    pub PCLK2_Frequency     : u32,
}

impl Default for RCC_ClocksTypeDef {
    fn default() -> Self {
        Self {
            SYSCLK_Frequency    : 0,
            HCLK_Frequency      : 0,
            PCLK1_Frequency     : 0,
            PCLK2_Frequency     : 0
        }
    }
}

const APBAHBPrescTable : [u8; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];

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

        /* Reset CFGR register */
        reg_value_clear_mask(&(*rcc_ptr).CFGR as *const u32 as *mut u32, 0x007F0000);

        /* Reset HSEON, CSSON and PLLON bits */
        reg_value_clear_mask(&(*rcc_ptr).CR as *const u32 as *mut u32, 0xFEF6FFFF);

        /* Reset PLLSRC, PLLXTPRE, PLLM and USBPRE bits */
        reg_value_clear_mask(&(*rcc_ptr).CFGR as *const u32 as *mut u32, 0x24003010);

        /* Reset HSEBYP bit */
        reg_value_clear_bit(&(*rcc_ptr).CR as *const u32 as *mut u32, RCC_CR_HSEBYP_Pos);

        // Disable all interrupts and clear pending bits
        reg_value_clear(&(*rcc_ptr).CIR as *const u32 as *mut u32);
    }
    
}

pub fn rcc_get_clocks_frequency(rcc_clocks: &mut RCC_ClocksTypeDef) {
    let mut tmp: u32;
    let mut presc: u8;
    unsafe{
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        rcc_clocks.HCLK_Frequency = FOSC_KHZ_VALUE * 1000;

        /*------ Compute HCLK, PCLK1, and PCLK2 clocks frequencies ------*/

        /* Get HCLK prescaler */
        tmp = (reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_HPRE_Msk) >> RCC_CFGR_HPRE_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        /* HCLK clock frequency */
        rcc_clocks.SYSCLK_Frequency = rcc_clocks.HCLK_Frequency << presc;

        /* Get PCLK1 prescaler */
        tmp = (reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_PPRE1_Msk) >> RCC_CFGR_PPRE1_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        /* PCLK1 clock frequency */
        rcc_clocks.PCLK1_Frequency = rcc_clocks.HCLK_Frequency >> presc;

        // Get PCLK2 prescaler
        tmp = reg_value_get(&(*rcc_ptr).CFGR as *const u32 as *mut u32) & RCC_CFGR_PPRE2_Msk;
        tmp = tmp >> RCC_CFGR_PPRE2_Pos;
        presc = APBAHBPrescTable[tmp as usize];

        // PCLK2 clock frequency
        rcc_clocks.PCLK2_Frequency = rcc_clocks.HCLK_Frequency >> presc;
    }
}

/**
   @brief Sets the overdrive mode.
   @note Used to get max mcu frequency.
   @param None
   @retval None
  */

pub fn enable_overdrive_mode() {
    unsafe{
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        let pwr_ptr : *mut PWR_TypeDef = PWR_BASE as *mut PWR_TypeDef;
        // Enable power clock
        reg_value_set_bit(&(*rcc_ptr).APB1ENR as *const u32 as *mut u32, RCC_APB1ENR_PWREN_Pos);

        // Set overdrive bit
        reg_value_set_bit(&(*pwr_ptr).CR as *const u32 as *mut u32, PWR_CR_ODEN_Pos); // ODEN

        // Wait for overdrive ready flag to be set
        while(reg_value_get_bit(&(*pwr_ptr).CSR as *const u32 as *mut u32, PWR_CSR_ODRDY_Pos) == 0) // ODRDY
        {   }

        // Enable overdrive switching
        reg_value_set_bit(&(*pwr_ptr).CR as *const u32 as *mut u32, PWR_CR_ODSWEN_Pos); //ODSWEN
        
        // Wait for overdrive switch ready flag to be set
        while(reg_value_get_bit(&(*pwr_ptr).CSR as *const u32 as *mut u32, PWR_CSR_ODSWRDY_Pos) == 0) //ODSWRDY
        {   }
    }
}

#[unsafe(no_mangle)]
pub fn system_init() {
    system_clock_set_default();

    unsafe{
        
        let rcc_ptr : *mut RCC_TypeDef = RCC_BASE as *mut RCC_TypeDef;
        let flash_ptr : *mut FLASH_TypeDef = FLASH_R_BASE as *mut FLASH_TypeDef;
  

    reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_PRFTEN_Pos);    // PRFTEN Prefetch enable
    reg_value_set_bit(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_ICEN_Pos);      // ICEN Instruction cache enable

    if VALUE_SVRANGE == VR_2700_3600 {
        if FOSC_KHZ_VALUE > 150000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 120000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 90000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 60000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 30000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(&(*flash_ptr).ACR as *const u32 as *mut u32, !FLASH_ACR_LATENCY_Msk);
        }
    } else if VALUE_SVRANGE == VR_2400_2700 {
        if FOSC_KHZ_VALUE > 168000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 144000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 120000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 96000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 72000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 48000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 24000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(&(*flash_ptr).ACR as *const u32 as *mut u32, !FLASH_ACR_LATENCY_Msk);
        }
    } else if VALUE_SVRANGE == VR_2100_2400 {
        if FOSC_KHZ_VALUE > 176000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_8WS);
        } else if FOSC_KHZ_VALUE > 154000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 132000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 110000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 88000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 66000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 44000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 22000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(&(*flash_ptr).ACR as *const u32 as *mut u32, !FLASH_ACR_LATENCY_Msk);
        }
    } else if VALUE_SVRANGE == VR_1800_2100 {
        if FOSC_KHZ_VALUE > 160000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_8WS);
        } else if FOSC_KHZ_VALUE > 140000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_7WS);
        } else if FOSC_KHZ_VALUE > 120000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_6WS);
        } else if FOSC_KHZ_VALUE > 100000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_5WS);
        } else if FOSC_KHZ_VALUE > 80000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_4WS);
        } else if FOSC_KHZ_VALUE > 60000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_3WS);
        } else if FOSC_KHZ_VALUE > 40000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_2WS);
        } else if FOSC_KHZ_VALUE > 20000 {
            reg_value_set(&(*flash_ptr).ACR as *const u32 as *mut u32, FLASH_ACR_LATENCY_1WS);
        } else {
            reg_value_clear_mask(&(*flash_ptr).ACR as *const u32 as *mut u32, !FLASH_ACR_LATENCY_Msk);
        }
    }

    enable_overdrive_mode();

    reg_value_clear_set(&(*rcc_ptr).PLLCFGR as *const u32 as *mut u32, VALUE_RCC_PLLCFGR);         /* set clock configuration register */
    reg_value_clear_set(&(*rcc_ptr).CFGR as *const u32 as *mut u32, VALUE_RCC_CFGR);               /* set clock configuration register 2 */
    reg_value_clear_set(&(*rcc_ptr).CR as *const u32 as *mut u32, VALUE_RCC_CR & 0x000FFFFF);      /* do not start PLLs yet */

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

    // FPU enabled by default by cortex_m_rt crate
    }
}

// ==================== DELAYS ======================

fn get_clock_value(_clock : u32) -> u32
{
    _clock / 1000 / 4
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
     * Delay for STM32F469II - default NECTO setup
     */
    Delay_Cyc(time_us * get_clock_value(FOSC_KHZ_VALUE) * 2 / 4);
}


#[inline(never)]
pub fn Delay_ms(time_ms: u32) 
{
    /*
     * Delay for STM32F469II - default NECTO setup
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
        &mut (*rcc_ptr).AHB1ENR
    }
}