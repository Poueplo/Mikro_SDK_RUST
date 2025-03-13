#![no_std]
#![no_main]

use core::default;

use cortex_m_rt::entry; // The runtime

extern crate mcu;
extern crate hal_ll_gpio_port;

#[allow(unused_imports)]
use panic_halt; 

const port: u32 = mcu::GPIOD_BASE_ADDR;

fn hal_ll_gpio_write_port_output(port1: u32, value: u16, mask: u16)
{
    let base_reg : *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t = 
                                    port1 as *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t;

    unsafe {
        (*base_reg).bsrr = (value as u32 & mask as u32) 
                        | (((!value & mask) as u32) << (hal_ll_gpio_port::RESET_PINS_OFFSET as u32));
    }
}

#[entry]
fn main() -> ! {
    hal_ll_gpio_port::hal_ll_gpio_digital_output(port, hal_ll_gpio_port::GPIO_PIN_MASK_LOW);
    hal_ll_gpio_write_port_output(port, 0x5555, hal_ll_gpio_port::GPIO_PIN_MASK_LOW);
    hal_ll_gpio_port::hal_ll_gpio_digital_output(port, hal_ll_gpio_port::GPIO_PIN_MASK_HIGH);
    hal_ll_gpio_write_port_output(port, 0x5555, hal_ll_gpio_port::GPIO_PIN_MASK_HIGH);
    hal_ll_gpio_port::hal_ll_gpio_digital_output(port, hal_ll_gpio_port::GPIO_PIN_MASK_ALL);
    hal_ll_gpio_write_port_output(port, 0xFFFF, hal_ll_gpio_port::GPIO_PIN_MASK_ALL);
    
    loop {}
}
