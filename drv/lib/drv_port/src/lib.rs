#![no_std]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use core::{fmt, ptr::null};
use hal_gpio::*;
use drv_name::*;

type Result<T> = core::result::Result<T, PORT_ERROR>;

#[derive(Debug, Clone)]
pub struct PORT_ERROR;

impl fmt::Display for PORT_ERROR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PORT_ERROR occurred")
    }
}

pub struct port_t
{
    port: hal_gpio_port_t, /*< Structure containing port address and port name. */
    name: port_name_t     /*< Port name. */
}

impl Default for port_t {
    fn default() -> Self {
        Self { port: hal_gpio_port_t::default(), name: 0}
    }
}


pub fn port_init(port: &mut port_t, name: port_name_t, mask: port_size_t, direction: gpio_direction_t) -> Result<()>
{
    if (direction != gpio_direction_t::GPIO_DIGITAL_INPUT) && (direction != gpio_direction_t::GPIO_DIGITAL_OUTPUT) {
        return Err(PORT_ERROR);
    }
    if name != HAL_PORT_NC {
        port.name = name;
        hal_gpio_configure_port( &mut port.port as *mut _, name, mask, hal_gpio_direction_t::from(direction));
        Ok(())
    } else {
        Err(PORT_ERROR)
    }
}

pub fn port_write(port: &mut port_t, value: port_size_t) -> Result<()>
{
    if port.port.base != 0 {
        hal_gpio_write_port_output(&mut port.port as *mut _, value);
        Ok(())
    } else {
        Err(PORT_ERROR)
    }
    
}

pub fn port_read_input(port: &mut port_t) -> Result<port_size_t>
{
    if port.port.base != 0 {
        Ok(hal_gpio_read_port_input(&mut port.port as *mut _))
    } else {
        Err(PORT_ERROR)
    }
}

pub fn port_read_output(port: &mut port_t) -> Result<port_size_t>
{
    if port.port.base != 0 {
        Ok(hal_gpio_read_port_output(&mut port.port as *mut _))
    } else {
        Err(PORT_ERROR)
    }
}