#![no_std]
#![allow(non_camel_case_types)]

use mcu::{hal_ll_pin_name_t, hal_ll_port_name_t, hal_ll_port_size_t};

pub const HAL_PORT_NC : hal_port_name_t = 0xFF;
pub const HAL_PIN_NC : hal_ll_pin_name_t = 0xFF;

pub type hal_pin_name_t = hal_ll_pin_name_t;
pub type hal_port_name_t = hal_ll_port_name_t;
pub type hal_port_size_t = hal_ll_port_size_t;