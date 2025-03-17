#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

//to change place at some point
//==========================================================//
pub extern crate mcu;

pub const GPIO_PIN_MASK_LOW: u16 = 0x00FF;
pub const GPIO_PIN_MASK_HIGH: u16 = 0xFF00;
pub const GPIO_PIN_MASK_ALL: u16 = 0xFFFF;
pub const HAL_LL_NIBBLE_HIGH_32BIT : u32 = 0xFFFF_0000;
pub const HAL_LL_NIBBLE_LOW_32BIT : u32 = 0xFFFF;

pub const GPIO_OSPEEDER_OSPEEDR0 : u32 = 0x03;
pub const GPIO_MODER_MODER0 : u32 = 0x03;
pub const GPIO_OTYPER_OT_0 : u32 = 0x01;
pub const GPIO_PUPDR_PUPDR0 : u32 = 0x03;


pub const GPIO_CFG_MODE_INPUT : u32 = 0x2;
pub const GPIO_CFG_MODE_OUTPUT : u32 = 0x4;
pub const GPIO_CFG_OTYPE_PP : u32 = 0x10;
pub const GPIO_CFG_PULL_NO : u32 = 0x40;
pub const GPIO_CFG_SPEED_MAX : u32 = 0x80000;

pub const GPIO_CFG_DIGITAL_OUTPUT : u32 = GPIO_CFG_MODE_OUTPUT | GPIO_CFG_SPEED_MAX | GPIO_CFG_OTYPE_PP;
pub const GPIO_CFG_DIGITAL_INPUT : u32 = GPIO_CFG_MODE_INPUT | GPIO_CFG_PULL_NO;

//==========================================================//

pub const RESET_PINS_OFFSET: u8 = 16;

pub type hal_ll_gpio_base_t = mcu::handle_t;

pub struct hal_ll_gpio_t
{
    pub base: hal_ll_gpio_base_t,
    pub mask: mcu::hal_ll_gpio_mask_t
}

impl Default for hal_ll_gpio_port_t {
    fn default() -> Self {
        Self { base: 0, mask: 0}
    }
}

#[derive(PartialEq)]
pub enum hal_ll_gpio_direction_t
{
    HAL_LL_GPIO_DIGITAL_INPUT = 0,
    HAL_LL_GPIO_DIGITAL_OUTPUT = 1
}

pub type hal_ll_gpio_port_t = hal_ll_gpio_t;

const _hal_ll_gpio_port_base: [u32; 11 ] =
[
    mcu::GPIOA_BASE_ADDR,
    mcu::GPIOB_BASE_ADDR,
    mcu::GPIOC_BASE_ADDR,
    mcu::GPIOD_BASE_ADDR,
    mcu::GPIOE_BASE_ADDR,
    mcu::GPIOF_BASE_ADDR,
    mcu::GPIOG_BASE_ADDR,
    mcu::GPIOH_BASE_ADDR,
    mcu::GPIOI_BASE_ADDR,
    mcu::GPIOJ_BASE_ADDR,
    mcu::GPIOK_BASE_ADDR
];

#[repr(C)]
pub struct hal_ll_gpio_base_handle_t 
{
    pub moder:   u32,
    pub otyper:  u32,
    pub ospeedr: u32,
    pub pupdr:   u32,
    pub idr:     u32,
    pub odr:     u32,
    pub bsrr:    u32,
    pub lckr:    u32,
    pub afrl:    u32,
    pub afrh:    u32,
}

const RCC_GPIOCLOCK: u32 = mcu::RCC_AHB1ENR;

pub fn hal_ll_gpio_port_base( name: mcu::hal_ll_port_name_t ) -> u32
{
    return _hal_ll_gpio_port_base[ name as usize ];
}

pub fn hal_ll_gpio_digital_input(port: u32, pin_mask: u16) 
{
    hal_ll_gpio_config(port, pin_mask, GPIO_CFG_DIGITAL_INPUT);
}

pub fn hal_ll_gpio_digital_output(port: u32, pin_mask: u16) 
{
    hal_ll_gpio_config(port, pin_mask, GPIO_CFG_DIGITAL_OUTPUT);
}

//TODO : optimize per MCU
fn hal_ll_gpio_clock_enable(port: u32) {
    let mut pos = 0;

    unsafe 
    {

        match port & 0xFFFFFC00 {
            mcu::GPIOA_BASE_ADDR => {pos = 0x1;},
            mcu::GPIOB_BASE_ADDR => {pos = 0x2;},
            mcu::GPIOC_BASE_ADDR => {pos = 0x4;},
            mcu::GPIOD_BASE_ADDR => {pos = 0x8;},
            mcu::GPIOE_BASE_ADDR => {pos = 0x10;},
            mcu::GPIOF_BASE_ADDR => {pos = 0x20;},
            mcu::GPIOG_BASE_ADDR => {pos = 0x40;},
            _ => {}
        }

        *(RCC_GPIOCLOCK as *mut u32) |= pos;
    }
}

//TODO : optimize per MCU, extra config tunnings
//only treated conf digital I/O any mask
fn hal_ll_gpio_config(port: u32, pin_mask: u16, config: u32) 
{
    let mut pos         : u32   = 0;
    let mut current_pin : u32   = 0;

    let mut mode        : u32   = 0;
    let mut speed       : u32   = 0;
    let mut otype       : u32   = 0;
    let mut pull        : u32   = 0;

    let port_ptr : *mut hal_ll_gpio_base_handle_t = port as *mut hal_ll_gpio_base_handle_t;
    
    hal_ll_gpio_clock_enable(port);
    

    if pin_mask == GPIO_PIN_MASK_LOW as u16 
    {
        unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_HIGH_32BIT; }
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { (*port_ptr).moder |= 0x0000_5555; }
            unsafe { (*port_ptr).otyper &= 0xFFFF_FF00; }
            unsafe { (*port_ptr).ospeedr |= HAL_LL_NIBBLE_LOW_32BIT; }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }

    } else if pin_mask == GPIO_PIN_MASK_HIGH as u16
    {
        unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_LOW_32BIT; }
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { (*port_ptr).moder |= 0x5555_0000; }
            unsafe { (*port_ptr).otyper &= 0xFFFF_00FF; }
            unsafe { (*port_ptr).ospeedr |= HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_LOW_32BIT; }
            return;
        }

    }

    if pin_mask == GPIO_PIN_MASK_ALL as u16
    {
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { (*port_ptr).moder = 0x5555_5555; }
            unsafe { (*port_ptr).otyper = 0; }
            unsafe { (*port_ptr).ospeedr = HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder = 0; }
            return;
        }
    }

    if config & GPIO_CFG_MODE_OUTPUT != 0
    {
        mode = 1;
    }
    else
    {
        mode = 0;
    }

    for pin_pos in 0x00 .. 0x10 {
        pos = 0x01 << pin_pos;
        current_pin = (pin_mask) as u32 & pos;

        if current_pin == pos
        {
            unsafe{
                (*port_ptr).moder &= !( GPIO_MODER_MODER0 << ( pin_pos * 2 ) );
                (*port_ptr).moder |=  ( mode ) << ( pin_pos * 2 );
            }

            if config & ( GPIO_CFG_MODE_OUTPUT /*| GPIO_CFG_MODE_ALT_FUNCTION*/) != 0
            {
                unsafe {                
                    (*port_ptr).ospeedr &= !( GPIO_OSPEEDER_OSPEEDR0 << ( pin_pos * 2 ) );
                    (*port_ptr).ospeedr |=  speed  << ( pin_pos * 2 );
                    
                    (*port_ptr).otyper &= !( GPIO_OTYPER_OT_0 << ( pin_pos ) as u16 ) ;
                    (*port_ptr).otyper |= otype << pin_pos;
                }
            }

            unsafe{
                (*port_ptr).pupdr &= !( GPIO_PUPDR_PUPDR0 << ( pin_pos * 2 ) );
                (*port_ptr).pupdr |=  pull  << ( pin_pos * 2 );
            }

        }

    }
}