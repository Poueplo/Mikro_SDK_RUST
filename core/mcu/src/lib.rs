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
#![allow(non_upper_case_globals)]
//for STM32F429ZI
pub const APB1_BASE_ADDR: u32 = 0x4000_0000;
pub const APB2_BASE_ADDR: u32 = 0x4001_0000;
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

pub const PORT_SIZE: u8 = 16;

pub const ADC_MODULE_1: u8 = 1;
pub const ADC_MODULE_2: u8 = 2;
pub const ADC_MODULE_3: u8 = 3;

pub const ADC_MODULE_COUNT: u8 = 3;

pub const I2C_MODULE_1: u8 = 1;
pub const I2C_MODULE_2: u8 = 2;
pub const I2C_MODULE_3: u8 = 3;

pub const I2C_MODULE_COUNT: u8 = 3;

pub const SPI_MODULE_1: u8 = 1;
pub const SPI_MODULE_2: u8 = 2;
pub const SPI_MODULE_3: u8 = 3;
pub const SPI_MODULE_4: u8 = 4;
pub const SPI_MODULE_5: u8 = 5;
pub const SPI_MODULE_6: u8 = 6;

pub const SPI_MODULE_COUNT: u8 = 6;

pub const TIM_MODULE_1: u8 = 1;
pub const TIM_MODULE_2: u8 = 2;
pub const TIM_MODULE_3: u8 = 3;
pub const TIM_MODULE_4: u8 = 4;
pub const TIM_MODULE_5: u8 = 5;
pub const TIM_MODULE_8: u8 = 6;
pub const TIM_MODULE_9: u8 = 7;
pub const TIM_MODULE_10: u8 = 8;
pub const TIM_MODULE_11: u8 = 9;
pub const TIM_MODULE_12: u8 = 10;
pub const TIM_MODULE_13: u8 = 11;
pub const TIM_MODULE_14: u8 = 12;

pub const TIM1_BUS: u8 = 2;
pub const TIM2_BUS: u8 = 1;
pub const TIM3_BUS: u8 = 1;
pub const TIM4_BUS: u8 = 1;
pub const TIM5_BUS: u8 = 1;
pub const TIM8_BUS: u8 = 2;
pub const TIM9_BUS: u8 = 2;
pub const TIM10_BUS: u8 = 2;
pub const TIM11_BUS: u8 = 2;
pub const TIM12_BUS: u8 = 1;
pub const TIM13_BUS: u8 = 1;
pub const TIM14_BUS: u8 = 1;

pub const TIM_MODULE_COUNT: u8 = 12;

//present in hal_ll_adc_pin_map
pub const ADC_BASE_ADDR: u32 = APB2_BASE_ADDR + 0x2000;
pub const ADC1_BASE_ADDR: u32 = ADC_BASE_ADDR;
pub const ADC2_BASE_ADDR: u32 = ADC_BASE_ADDR + 0x100;
pub const ADC3_BASE_ADDR: u32 = ADC_BASE_ADDR + 0x200;

//present in hal_ll_i2c_pin_map
pub const I2C_BASE_ADDR: u32 = APB1_BASE_ADDR + 0x5400;
pub const I2C1_BASE_ADDR: u32 = I2C_BASE_ADDR;
pub const I2C2_BASE_ADDR: u32 = I2C_BASE_ADDR + 0x400;
pub const I2C3_BASE_ADDR: u32 = I2C_BASE_ADDR + 0x800;

//present in hal_ll_spi_master_pin_map
pub const SPI1_MASTER_BASE_ADDR : u32 = 0x4001_3000;
pub const SPI2_MASTER_BASE_ADDR : u32 = 0x4000_3800;
pub const SPI3_MASTER_BASE_ADDR : u32 = 0x4000_3C00;
pub const SPI4_MASTER_BASE_ADDR : u32 = 0x4001_3400;
pub const SPI5_MASTER_BASE_ADDR : u32 = 0x4001_5000;
pub const SPI6_MASTER_BASE_ADDR : u32 = 0x4001_5400;

//prsent in hal_ll_tim_pin_map
pub const TIM1_BASE_ADDR : u32 = 0x4001_0000;
pub const TIM2_BASE_ADDR : u32 = 0x4000_0000;
pub const TIM3_BASE_ADDR : u32 = 0x4000_0400;
pub const TIM4_BASE_ADDR : u32 = 0x4000_0800;
pub const TIM5_BASE_ADDR : u32 = 0x4000_0C00;
pub const TIM8_BASE_ADDR : u32 = 0x4001_0400;
pub const TIM9_BASE_ADDR : u32 = 0x4001_4000;
pub const TIM10_BASE_ADDR : u32 = 0x4001_4400;
pub const TIM11_BASE_ADDR : u32 = 0x4001_4800;
pub const TIM12_BASE_ADDR : u32 = 0x4000_1800;
pub const TIM13_BASE_ADDR : u32 = 0x4000_1C00;
pub const TIM14_BASE_ADDR : u32 = 0x4000_2000;

//present in core_header.h
pub const ADDRESS_RCC_CR         : u32 = 0x40023800;
pub const VALUE_RCC_CR           : u32 = 0x1000081;
pub const ADDRESS_RCC_PLLCFGR    : u32 = 0x40023804;
pub const VALUE_RCC_PLLCFGR      : u32 = 0x29005A10;
pub const ADDRESS_RCC_CFGR       : u32 = 0x40023808;
pub const VALUE_RCC_CFGR         : u32 = 0x609402;
pub const ADDRESS_SVRANGE        : u32 = 0x400FE070;
pub const VALUE_SVRANGE          : u32 = 0x3;
pub const ADDRESS_RCC_PLLSAICFGR : u32 = 0x40023888;
pub const VALUE_RCC_PLLSAICFGR   : u32 = 0x24003000;
pub const FOSC_KHZ_VALUE         : u32 = 180000;


//prsent in mcu_header
pub const RCC_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x3800;
pub const PWR_BASE_ADDR: u32 = APB1_BASE_ADDR + 0x7000;

// =========== RCC REGISTERS =========== 

pub const RCC_CR          : u32 = RCC_BASE_ADDR;
pub const RCC_PLLCFGR     : u32 = RCC_BASE_ADDR + 0x04;
pub const RCC_CFGR        : u32 = RCC_BASE_ADDR + 0x08;
pub const RCC_CIR         : u32 = RCC_BASE_ADDR + 0x0C;
//present in hal_ll_rcc.h
pub const RCC_AHB1ENR     : u32 = RCC_BASE_ADDR + 0x30;
//
pub const RCC_APB1ENR     : u32 = RCC_BASE_ADDR + 0x40;
pub const RCC_APB2ENR     : u32 = RCC_BASE_ADDR + 0x44;

// =========== PWR REGISTERS =========== 

pub const PWR_CR          : u32 = PWR_BASE_ADDR + 0x00;
pub const PWR_CSR         : u32 = PWR_BASE_ADDR + 0x04;

// =====================================

/***** Bit definition for RCC_CR register  *****/

pub const RCC_CR_HSION_Pos   : u32   =  0;
pub const RCC_CR_HSEBYP_Pos  : u32   = 18;
pub const RCC_CR_HSIRDY_Pos  : u32   =  1;
pub const RCC_CR_HSEON_Pos   : u32   = 16;
pub const RCC_CR_HSERDY_Pos  : u32   = 17;
pub const RCC_CR_PLLON_Pos   : u32   = 24;
pub const RCC_CR_PLLRDY_Pos  : u32   = 25;

/***** Bit definition for PWR_CR register  *****/

pub const RCC_APB1ENR_PWREN_Pos : u32 = 28;

pub const PWR_CR_ODEN_Pos       : u32 = 16;
pub const PWR_CSR_ODRDY_Pos     : u32 = 16;

pub const PWR_CR_ODSWEN_Pos     : u32 = 17;
pub const PWR_CSR_ODSWRDY_Pos   : u32 = 17;

/***** Bit definition for RCC_CFGR register  *****/

/* SWS configuration */
pub const RCC_CFGR_SWS_Pos    : u32 = 2;
pub const RCC_CFGR_SWS_Msk    : u32 = 0x3 << RCC_CFGR_SWS_Pos;

/* HPRE configuration */
pub const RCC_CFGR_HPRE_Pos   : u32 = 4;  
pub const RCC_CFGR_HPRE_Msk   : u32 = 0xF << RCC_CFGR_HPRE_Pos;

/* PPRE1 configuration */
pub const RCC_CFGR_PPRE1_Pos  : u32 = 10;                              
pub const RCC_CFGR_PPRE1_Msk  : u32 = 0x7 << RCC_CFGR_PPRE1_Pos;

/* PPRE2 configuration */
pub const RCC_CFGR_PPRE2_Pos  : u32 = 13;
pub const RCC_CFGR_PPRE2_Msk  : u32 = 0x7 << RCC_CFGR_PPRE2_Pos;

/*************************************************/

// =========== FLASH REGISTERS =========== 

pub const FLASH_BASE              : u32 = 0x40023C00;

pub const FLASH_ACR               : u32 = FLASH_BASE;

pub const FLASH_ACR_LATENCY_1WS   : u32 = 0x00000001;
pub const FLASH_ACR_LATENCY_2WS   : u32 = 0x00000002;
pub const FLASH_ACR_LATENCY_3WS   : u32 = 0x00000003;
pub const FLASH_ACR_LATENCY_4WS   : u32 = 0x00000004;
pub const FLASH_ACR_LATENCY_5WS   : u32 = 0x00000005;
pub const FLASH_ACR_LATENCY_6WS   : u32 = 0x00000006;
pub const FLASH_ACR_LATENCY_7WS   : u32 = 0x00000007;
pub const FLASH_ACR_LATENCY_8WS   : u32 = 0x00000008;

// =====================================

/***** Bit definition for FLASH_ACR register  *****/

pub const FLASH_ACR_PRFTEN_Pos  : u32 = 8;
pub const FLASH_ACR_ICEN_Pos    : u32 = 9;

pub const FLASH_ACR_LATENCY_Pos : u32 = 0;
pub const FLASH_ACR_LATENCY_Msk : u32 = 0xF << FLASH_ACR_LATENCY_Pos;

/*************************************************/