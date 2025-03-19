#![no_std]
#![allow(non_camel_case_types)]

use hal_gpio::hal_gpio_direction_t;
use hal_target::{hal_port_name_t, hal_port_size_t,hal_pin_name_t};
pub use hal_target::{HAL_PORT_NC,HAL_PIN_NC};
pub use hal_target::pin_names::*;

#[derive(PartialEq)]
pub enum gpio_direction_t
{
    GPIO_DIGITAL_INPUT = 0, /*< GPIO Digital input. */
    GPIO_DIGITAL_OUTPUT = 1 /*< GPIO Digital output. */
}

impl From<gpio_direction_t> for hal_gpio_direction_t {
    fn from(direction: gpio_direction_t) -> Self {
        match direction {
            gpio_direction_t::GPIO_DIGITAL_INPUT => hal_gpio_direction_t::HAL_GPIO_DIGITAL_INPUT,
            _ =>  hal_gpio_direction_t::HAL_GPIO_DIGITAL_OUTPUT,
        }
    }
}

pub type pin_name_t = hal_pin_name_t; /*< GPIO pin name. */

pub type port_name_t = hal_port_name_t; /*< GPIO port name.*/

pub type port_size_t = hal_port_size_t; /*< GPIO port size. */