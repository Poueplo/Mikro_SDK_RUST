#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

//to change place at some point
//==========================================================//
pub use hal_ll_target::pin_names;
use hal_ll_target::*;
use hal_ll_gpio_constants::*;

//move to hal_ll_bit_control
pub const HAL_LL_NIBBLE_HIGH_32BIT : u32 = 0xFFFF_0000;
pub const HAL_LL_NIBBLE_LOW_32BIT : u32 = 0xFFFF;

//==========================================================//

pub const RESET_PINS_OFFSET: u8 = 16;

pub type hal_ll_gpio_base_t = handle_t;

pub struct hal_ll_gpio_t
{
    pub base: hal_ll_gpio_base_t,
    pub mask: hal_ll_gpio_mask_t
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

pub type hal_ll_gpio_pin_t = hal_ll_gpio_t;
pub type hal_ll_gpio_port_t = hal_ll_gpio_t;

const _hal_ll_gpio_port_base: [u32; 11 ] =
[
    GPIOA_BASE_ADDR,
    GPIOB_BASE_ADDR,
    GPIOC_BASE_ADDR,
    GPIOD_BASE_ADDR,
    GPIOE_BASE_ADDR,
    GPIOF_BASE_ADDR,
    GPIOG_BASE_ADDR,
    GPIOH_BASE_ADDR,
    GPIOI_BASE_ADDR,
    GPIOJ_BASE_ADDR,
    GPIOK_BASE_ADDR
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

const RCC_GPIOCLOCK: u32 = RCC_AHB1ENR;

fn hal_ll_gpio_pin_index(name: hal_ll_pin_name_t) -> u8
{
    name % PORT_SIZE
}

pub fn hal_ll_gpio_port_index(name: hal_ll_pin_name_t) -> u8
{
    name / PORT_SIZE
}

pub fn hal_ll_gpio_pin_mask(name: hal_ll_pin_name_t) -> u16
{
    0x01 << hal_ll_gpio_pin_index( name )
}

pub fn hal_ll_gpio_port_base( name: hal_ll_port_name_t ) -> u32
{
    _hal_ll_gpio_port_base[ name as usize ]
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
            GPIOA_BASE_ADDR => {pos = 0x1;},
            GPIOB_BASE_ADDR => {pos = 0x2;},
            GPIOC_BASE_ADDR => {pos = 0x4;},
            GPIOD_BASE_ADDR => {pos = 0x8;},
            GPIOE_BASE_ADDR => {pos = 0x10;},
            GPIOF_BASE_ADDR => {pos = 0x20;},
            GPIOG_BASE_ADDR => {pos = 0x40;},
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
            unsafe {
                (*port_ptr).moder |= 0x0000_5555;
                (*port_ptr).otyper &= 0xFFFF_FF00;
                (*port_ptr).ospeedr |= HAL_LL_NIBBLE_LOW_32BIT;
            }
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
            unsafe {
                (*port_ptr).moder |= 0x5555_0000;
                (*port_ptr).otyper &= 0xFFFF_00FF;
                (*port_ptr).ospeedr |= HAL_LL_NIBBLE_HIGH_32BIT; 
            }
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
            unsafe {
                (*port_ptr).moder = 0x5555_5555;
                (*port_ptr).otyper = 0;
                (*port_ptr).ospeedr = HAL_LL_NIBBLE_HIGH_32BIT;
            }
            return;
        }

        if config == GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { (*port_ptr).moder = 0; }
            return;
        }
    }


    if config & GPIO_CFG_MODE_ANALOG != 0 {
        mode = 3;
    } else if config & GPIO_CFG_MODE_ALT_FUNCTION != 0 {
        mode = 2;
    } else if config & GPIO_CFG_MODE_OUTPUT != 0 {
        mode = 1;
    } else {
        mode = 0;
    }

    //TODO adapt per cpu
    if config & (GPIO_CFG_SPEED_VERY_HIGH | GPIO_CFG_SPEED_MAX) != 0 {
        speed = 3;
    } else if config & GPIO_CFG_SPEED_HIGH != 0 {
        speed = 2;
    } else if config & GPIO_CFG_SPEED_MEDIUM != 0 {
        speed = 1;
    } else {
        speed = 0;
    }

    if config & GPIO_CFG_OTYPE_OD != 0 {
        otype = 1;
    } else {
        otype = 0;
    }

    if config & GPIO_CFG_PULL_DOWN != 0 {
        pull = 2;
    } else if config & GPIO_CFG_PULL_UP != 0 {
        pull = 1;
    } else {
        pull = 0;
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

            if config & ( GPIO_CFG_MODE_OUTPUT | GPIO_CFG_MODE_ALT_FUNCTION) != 0
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