#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
//for STM32F429ZI

pub const AHB1_BASE_ADDR: u32 = 0x4002_0000;

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

pub const RCC_BASE_ADDR: u32 = AHB1_BASE_ADDR + 0x3800;
pub const RCC_AHB1ENR:   u32 = RCC_BASE_ADDR + 0x0030;


///to move somewhere else
pub type handle_t = u32;
pub type hal_ll_gpio_mask_t = u16;
pub type hal_ll_port_size_t = u16;
pub type hal_ll_port_name_t = u8;
pub const PORT_A: hal_ll_port_name_t = 0x00;
pub const PORT_B: hal_ll_port_name_t = 0x01;
pub const PORT_C: hal_ll_port_name_t = 0x02;
pub const PORT_D: hal_ll_port_name_t = 0x03;
pub const PORT_E: hal_ll_port_name_t = 0x04;
pub const PORT_F: hal_ll_port_name_t = 0x05;
pub const PORT_G: hal_ll_port_name_t = 0x06;
pub const PORT_H: hal_ll_port_name_t = 0x07;
pub const PORT_I: hal_ll_port_name_t = 0x08;
pub const PORT_J: hal_ll_port_name_t = 0x09;
pub const PORT_K: hal_ll_port_name_t = 0x0A;