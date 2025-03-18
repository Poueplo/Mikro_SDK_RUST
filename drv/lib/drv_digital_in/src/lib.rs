#![no_std]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use core::{fmt, ptr::null};
use hal_gpio::*;
use drv_name::*;

type Result<T> = core::result::Result<T, DIGITAL_IN_ERROR>;

#[derive(Debug, Clone)]
pub struct DIGITAL_IN_ERROR;

impl fmt::Display for DIGITAL_IN_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DIGITAL_IN_ERROR occurred")
    }
}

pub struct digital_in_t
{
    pin: hal_gpio_pin_t, /*< Structure containing port address and port name. */
}

impl Default for digital_in_t {
    fn default() -> Self {
        Self { pin: hal_gpio_pin_t::default()}
    }
}

pub fn digital_in_init(in_pin: &mut digital_in_t, name: pin_name_t) -> Result<()>
{
    if name != HAL_PIN_NC {
        hal_gpio_configure_pin(&mut in_pin.pin as *mut _, name, hal_gpio_direction_t::from(gpio_direction_t::GPIO_DIGITAL_INPUT));
        Ok(())
    } else {
        Err(DIGITAL_IN_ERROR)
    }
}

pub fn digital_in_read(in_pin: &mut digital_in_t) -> Result<u8>
{
    if in_pin.pin.base != 0 {
        Ok(hal_gpio_read_pin_input(&mut in_pin.pin as *mut _))
    } else {
        Err(DIGITAL_IN_ERROR)
    }
}