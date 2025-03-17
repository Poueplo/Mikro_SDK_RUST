#![no_std]
#![no_main]
#![allow(unused)]
#![allow(non_upper_case_globals)]

use core::default;

use cortex_m_rt::entry; // The runtime

#[allow(unused_imports)]
use panic_halt;

const port_out: mcu::hal_ll_port_name_t = mcu::PORT_D;
const port_out1: mcu::hal_ll_port_name_t = mcu::PORT_E;
const port_out2: mcu::hal_ll_port_name_t = mcu::PORT_F;
const port_out3: mcu::hal_ll_port_name_t = mcu::PORT_G;
const port_in: mcu::hal_ll_port_name_t = mcu::PORT_B;




#[entry]
fn main() -> ! {
    let mut output:hal_ll_gpio_port::hal_ll_gpio_t = hal_ll_gpio_port::hal_ll_gpio_t::default();
    let mut output1:hal_ll_gpio_port::hal_ll_gpio_t = hal_ll_gpio_port::hal_ll_gpio_t::default();
    let mut output2:hal_ll_gpio_port::hal_ll_gpio_t = hal_ll_gpio_port::hal_ll_gpio_t::default();
    let mut output3:hal_ll_gpio_port::hal_ll_gpio_t = hal_ll_gpio_port::hal_ll_gpio_t::default();
    let mut input:hal_ll_gpio_port::hal_ll_gpio_t = hal_ll_gpio_port::hal_ll_gpio_t::default();

    let mut value : mcu::hal_ll_port_size_t;

    hal_ll_gpio::hal_ll_gpio_configure_port(&mut output as *mut _, port_out, 0x5555, hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT);
    hal_ll_gpio::hal_ll_gpio_configure_port(&mut output1 as *mut _, port_out1, hal_ll_gpio_port::GPIO_PIN_MASK_LOW, hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT);
    hal_ll_gpio::hal_ll_gpio_configure_port(&mut output2 as *mut _, port_out2, hal_ll_gpio_port::GPIO_PIN_MASK_HIGH, hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT);
    hal_ll_gpio::hal_ll_gpio_configure_port(&mut output3 as *mut _, port_out3, hal_ll_gpio_port::GPIO_PIN_MASK_ALL, hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_OUTPUT);
    hal_ll_gpio::hal_ll_gpio_write_port_output(&mut output as *mut _, 0xFFFF);
    hal_ll_gpio::hal_ll_gpio_write_port_output(&mut output1 as *mut _, 0xFFFF);
    hal_ll_gpio::hal_ll_gpio_write_port_output(&mut output2 as *mut _, 0xFFFF);
    hal_ll_gpio::hal_ll_gpio_write_port_output(&mut output3 as *mut _, 0xFFFF);

    hal_ll_gpio::hal_ll_gpio_configure_port(&mut input as *mut _, port_in, hal_ll_gpio_port::GPIO_PIN_MASK_ALL, hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT);

    value = hal_ll_gpio::hal_ll_gpio_read_port_ouput(&mut output as *mut _);
    value = hal_ll_gpio::hal_ll_gpio_read_port_input(&mut input as *mut _);
    
    loop {}
}
