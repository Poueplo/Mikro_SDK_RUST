#![no_std]

extern crate hal_ll_gpio_port;

pub fn hal_ll_gpio_configure_port(port: *mut hal_ll_gpio_port::hal_ll_gpio_port_t, name: mcu::hal_ll_port_name_t , mask: mcu::hal_ll_gpio_mask_t, direction: hal_ll_gpio_port::hal_ll_gpio_direction_t)
{
    unsafe{
        (*port).base = hal_ll_gpio_port::hal_ll_gpio_port_base(name);
        (*port).mask = mask;

        if direction == hal_ll_gpio_port::hal_ll_gpio_direction_t::HAL_LL_GPIO_DIGITAL_INPUT
        {
            hal_ll_gpio_port::hal_ll_gpio_digital_input((*port).base, (*port).mask);
        }
        else
        {
            hal_ll_gpio_port::hal_ll_gpio_digital_output((*port).base, (*port).mask);
        }
    }
}

pub fn hal_ll_gpio_read_port_input(port: *mut hal_ll_gpio_port::hal_ll_gpio_port_t) -> mcu::hal_ll_port_size_t
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t;

        ((*base_reg).idr & ((*port).mask) as u32)as u16
    }
}

pub fn hal_ll_gpio_read_port_ouput(port: *mut hal_ll_gpio_port::hal_ll_gpio_port_t) -> mcu::hal_ll_port_size_t
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t;

        ((*base_reg).odr & ((*port).mask) as u32)as u16
    }
}

pub fn hal_ll_gpio_write_port_output(port: *mut hal_ll_gpio_port::hal_ll_gpio_port_t, value: mcu::hal_ll_port_size_t)
{
    unsafe {
        let base_reg : *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t = 
                    (*port).base as *mut hal_ll_gpio_port::hal_ll_gpio_base_handle_t;

        (*base_reg).bsrr = (value as u32 & (*port).mask as u32) | (((!value & (*port).mask) as u32) << (hal_ll_gpio_port::RESET_PINS_OFFSET as u32));
    }
}