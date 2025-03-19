#![no_std]
#![allow(non_camel_case_types)]

use hal_ll_gpio::*;
use hal_target::*;
use hal_target::pin_names::*;

#[derive(PartialEq)]
pub enum hal_gpio_direction_t
{
    HAL_GPIO_DIGITAL_INPUT = 0,
    HAL_GPIO_DIGITAL_OUTPUT = 1
}

pub type hal_gpio_base_t = handle_t;
pub type hal_gpio_mask_t = hal_ll_gpio_mask_t;

pub struct hal_gpio_t
{
    pub base: hal_gpio_base_t,
    pub mask: hal_gpio_mask_t
}

impl Default for hal_gpio_t {
    fn default() -> Self {
        Self { base: 0, mask: 0}
    }
}

impl From<hal_gpio_direction_t> for hal_ll_gpio_direction_t {
    fn from(direction: hal_gpio_direction_t) -> Self {
        match direction {
            hal_gpio_direction_t::HAL_GPIO_DIGITAL_INPUT => hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT,
            _ => hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT,
        }
    }
}

pub type hal_gpio_pin_t = hal_gpio_t;
pub type hal_gpio_port_t = hal_gpio_t;

pub fn hal_gpio_configure_pin( pin: *mut hal_gpio_pin_t, name: hal_pin_name_t,
        direction: hal_gpio_direction_t)
{
    hal_ll_gpio_configure_pin( pin as *mut hal_ll_gpio_pin_t, name, hal_ll_gpio_direction_t::from(direction) );
}

pub fn hal_gpio_read_pin_input(pin: *mut hal_gpio_pin_t) -> u8
{
    hal_ll_gpio_read_pin_input(pin as *mut hal_ll_gpio_pin_t)
}

pub fn hal_gpio_read_pin_output(pin: *mut hal_gpio_pin_t) -> u8
{
    hal_ll_gpio_read_pin_output(pin as *mut hal_ll_gpio_pin_t)
}

pub fn hal_gpio_write_pin_output(pin: *mut hal_gpio_pin_t, value: u8)
{
    hal_ll_gpio_write_pin_output(pin as *mut hal_ll_gpio_pin_t, value);
}

pub fn hal_gpio_toggle_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_toggle_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_set_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_set_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_clear_pin_output(pin: *mut hal_gpio_pin_t)
{
    hal_ll_gpio_clear_pin_output(pin as *mut hal_ll_gpio_pin_t);
}

pub fn hal_gpio_configure_port( port: *mut hal_gpio_port_t, name: hal_port_name_t,
    mask: hal_gpio_mask_t, direction: hal_gpio_direction_t)
{
    hal_ll_gpio_configure_port( port as *mut hal_ll_gpio_port_t, name, mask, hal_ll_gpio_direction_t::from(direction) );
}

pub fn hal_gpio_read_port_input(port: *mut hal_gpio_port_t) -> hal_port_size_t
{
    hal_ll_gpio_read_port_input(port as *mut hal_ll_gpio_port_t)
}

pub fn hal_gpio_read_port_output(port: *mut hal_gpio_port_t) -> hal_port_size_t
{
    hal_ll_gpio_read_port_output(port as *mut hal_ll_gpio_port_t)
}

pub fn hal_gpio_write_port_output(port: *mut hal_gpio_port_t, value: hal_port_size_t)
{
    hal_ll_gpio_write_port_output(port as *mut hal_ll_gpio_port_t, value as hal_ll_port_size_t);
}