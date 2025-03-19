#![no_std]

pub const GPIO_PIN_MASK_LOW: u16 = 0x00FF;
pub const GPIO_PIN_MASK_HIGH: u16 = 0xFF00;
pub const GPIO_PIN_MASK_ALL: u16 = 0xFFFF;

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