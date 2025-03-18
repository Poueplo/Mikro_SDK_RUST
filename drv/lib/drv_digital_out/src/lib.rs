#![no_std]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use core::{fmt, ptr::null};
use hal_gpio::*;
use drv_name::*;

type Result<T> = core::result::Result<T, DIGITAL_OUT_ERROR>;

#[derive(Debug, Clone)]
pub struct DIGITAL_OUT_ERROR;

impl fmt::Display for DIGITAL_OUT_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DIGITAL_OUT_ERROR occurred")
    }
}

pub struct digital_out_t
{
    pin: hal_gpio_pin_t, /*< Structure containing port address and port name. */
}

impl Default for digital_out_t {
    fn default() -> Self {
        Self { pin: hal_gpio_pin_t::default()}
    }
}

pub fn digital_out_init(out_pin: &mut digital_out_t, name: pin_name_t) -> Result<()>
{
    if name != HAL_PIN_NC {
        hal_gpio_configure_pin(&mut out_pin.pin as *mut _, name, hal_gpio_direction_t::from(gpio_direction_t::GPIO_DIGITAL_OUTPUT));
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_high(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_set_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_low(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_clear_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_toggle(out_pin: &mut digital_out_t) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_toggle_pin_output(&mut out_pin.pin as *mut _);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}

pub fn digital_out_write(out_pin: &mut digital_out_t, value: u8) -> Result<()>
{
    if out_pin.pin.base != 0 {
        hal_gpio_write_pin_output(&mut out_pin.pin as *mut _, value);
        Ok(())
    } else {
        Err(DIGITAL_OUT_ERROR)
    }
}