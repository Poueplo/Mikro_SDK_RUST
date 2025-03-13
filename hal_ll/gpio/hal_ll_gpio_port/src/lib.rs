#![no_std]

//to change place at some point
//==========================================================//
pub extern crate mcu;

pub const GPIO_PIN_MASK_LOW: u16 = 0x00FF;
pub const GPIO_PIN_MASK_HIGH: u16 = 0xFF00;
pub const GPIO_PIN_MASK_ALL: u16 = 0xFFFF;
pub const HAL_LL_NIBBLE_HIGH_32BIT : u32 = 0xFFFF_0000;
pub const HAL_LL_NIBBLE_LOW_32BIT : u32 = 0xFFFF;

pub const GPIO_CFG_MODE_OUTPUT : u32 = 0x4;
pub const GPIO_CFG_OTYPE_PP : u32 = 0x10;
pub const GPIO_CFG_SPEED_MAX : u32 = 0x80000;

pub const  GPIO_CFG_DIGITAL_OUTPUT : u32 = ( GPIO_CFG_MODE_OUTPUT | GPIO_CFG_SPEED_MAX | GPIO_CFG_OTYPE_PP );


//==========================================================//

pub const RESET_PINS_OFFSET: u8 = 16;


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

const RCC_GPIOCLOCK: u32 = mcu::RCC_AHB1ENR;


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
            mcu::GPIOA_BASE_ADDR => {pos = 0x1;},
            mcu::GPIOB_BASE_ADDR => {pos = 0x2;},
            mcu::GPIOC_BASE_ADDR => {pos = 0x4;},
            mcu::GPIOD_BASE_ADDR => {pos = 0x8;},
            mcu::GPIOE_BASE_ADDR => {pos = 0x10;},
            mcu::GPIOF_BASE_ADDR => {pos = 0x20;},
            mcu::GPIOG_BASE_ADDR => {pos = 0x40;},
            _ => {}
        }

        *(RCC_GPIOCLOCK as *mut u32) |= pos;
    }
}

//TODO : optimize per MCU
//only conf output treated
fn hal_ll_gpio_config(port: u32, pin_mask: u16, config: u32) 
{

    let mut pin_pos     : u32   = 0;
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
            unsafe { (*port_ptr).moder |= 0x0000_5555; }
            unsafe { (*port_ptr).otyper &= 0xFFFFFF00; }
            unsafe { (*port_ptr).ospeedr |= HAL_LL_NIBBLE_LOW_32BIT; }
            return;
        }

        // if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_INPUT 
        // {
        //     unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
        //     return;
        // }
    } else if pin_mask == GPIO_PIN_MASK_HIGH as u16
    {
        unsafe { (*port_ptr).moder &= HAL_LL_NIBBLE_LOW_32BIT; }
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { (*port_ptr).moder |= 0x5555_0000; }
            unsafe { (*port_ptr).otyper &= 0xFFFF00FF; }
            unsafe { (*port_ptr).ospeedr |= HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }

    }

    if pin_mask == GPIO_PIN_MASK_ALL as u16
    {
        if config == GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { (*port_ptr).moder = 0x5555_5555; }
            unsafe { (*port_ptr).otyper = 0; }
            unsafe { (*port_ptr).ospeedr = HAL_LL_NIBBLE_HIGH_32BIT; }
            return;
        }
        /*if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            unsafe { *(port_ptr.crh as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            return;
        }*/
    }

//TODO 
/*
    if pin_mask == hal_ll_gpio_constants::GPIO_PIN_MASK_LOW as u16 
    {
        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_OUTPUT;}
            return;
        }

        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            return;
        }
    } 
    else if pin_mask == hal_ll_gpio_constants::GPIO_PIN_MASK_HIGH as u16 
    {
        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { *(port_ptr.crh as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_OUTPUT; }
            return;
        }
        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { *(port_ptr.crh as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            return;
        }
    }

    if pin_mask == hal_ll_gpio_constants::GPIO_PIN_MASK_ALL as u16
    {
        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_OUTPUT 
        {
            unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_OUTPUT; }
            unsafe { *(port_ptr.crh as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_OUTPUT; }
            return;
        }
        if config == hal_ll_gpio_constants::GPIO_CFG_DIGITAL_INPUT 
        {
            unsafe { *(port_ptr.crl as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            unsafe { *(port_ptr.crh as *mut u32) = hal_ll_gpio_constants::GPIO_SET_ALL_INPUT; }
            return;
        }
    }

    if config & (hal_ll_gpio_constants::GPIO_CFG_MODE_ANALOG) != 0 
    {
        mode = 0;
    } 
    else if config & hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT != 0 
    {
        if config & (hal_ll_gpio_constants::GPIO_CFG_PULL_DOWN | hal_ll_gpio_constants::GPIO_CFG_PULL_UP) != 0 
        {
            mode = 0x8;
        } 
        else 
        {
            mode = 0x4;
        }
    } 
    else if config & hal_ll_gpio_constants::GPIO_CFG_MODE_OUTPUT != 0 
    {
        if config & hal_ll_gpio_constants::GPIO_CFG_OTYPE_OD != 0 
        {
            mode = 0x4;
        } 
        else 
        {
            mode = 0;
        }
    } 
    else if config & hal_ll_gpio_constants::GPIO_CFG_MODE_ALT_FUNCTION != 0 
    {
        if config & hal_ll_gpio_constants::GPIO_CFG_OTYPE_OD != 0 
        {
            mode = 0xC;
        } 
        else 
        {
            mode = 0x8;
        }
    }

    if config & (hal_ll_gpio_constants::GPIO_CFG_SPEED_MAX | hal_ll_gpio_constants::GPIO_CFG_SPEED_50MHZ) != 0 
    {
        speed = 3;
    } 
    else if config & hal_ll_gpio_constants::GPIO_CFG_SPEED_2MHZ != 0 
    {
        speed = 2;
    } 
    else if config & hal_ll_gpio_constants::GPIO_CFG_SPEED_10MHZ != 0 
    {
        speed = 1;
    }

    if config & (hal_ll_gpio_constants::GPIO_CFG_MODE_OUTPUT | hal_ll_gpio_constants::GPIO_CFG_MODE_ALT_FUNCTION) != 0 
    {
        /* Output mode */
        mode |= speed;
    }

    if (pin_mask & 0xFF) != 0 
    {
        tmpreg = unsafe { *(port_ptr.crl as *mut u32) }; // CRL register

        for pin_pos in 0..8 
        {
            pos = 1 << pin_pos;

            /* Get the port pins position */
            current_pin = pin_mask as u32 & pos;

            if current_pin == pos 
            {
                pos = pin_pos << 2;

                /* Clear the corresponding low control register bits */
                tmp_pinmask = 0x0F << pos;
                tmpreg &= !tmp_pinmask;

                /* Write the mode configuration in the corresponding bits */
                tmpreg |= mode << pos;

                /* Reset the corresponding ODR bit */
                if (config & (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_DOWN)) == 
                                (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_DOWN) 
                {
                    unsafe { *(port_ptr.brr as *mut u32) = 1 << pin_pos }; // write to BRR register
                }
                
                /* Set the corresponding ODR bit */
                if (config & (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_UP)) == 
                                    (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_UP) 
                {
                    unsafe { *(port_ptr.bsrr as *mut u32) = 1 << pin_pos };
                }
            }
        }
        unsafe { *(port_ptr.crl as *mut u32) = tmpreg; } // write to CRL register
    }




    /*---------------------------- GPIO CRH Configuration ------------------------*/
    
    /* Configure the eight high port pins */
    use hal_ll_target::hal_ll_bit_control;

    if pin_mask > hal_ll_bit_control::HAL_LL_NIBBLE_LOW_16BIT 
    {

        unsafe { tmpreg = port_ptr.crh; } // GPIOx->CRH;
        
        for pin_pos in 0..8 
        {
            pos = 1 << (pin_pos + 8);

            /* Get the port pins position */
            current_pin = pin_mask as u32 & pos;

            if current_pin == pos 
            {
                pos = pin_pos << 2;

            /* Clear the corresponding high control register bits */
                tmp_pinmask = 0x0F << pos;
                tmpreg &= !tmp_pinmask;

            
            /* Write the mode configuration in the corresponding bits */
                tmpreg |= mode << pos;


            /* Reset the corresponding ODR bit */
                if (config & (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_DOWN)) == 
                        (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_DOWN) 
                {
                    unsafe { *(port_ptr.brr as *mut u32) = 1 << (pin_pos + 8); }
                }

            /* Set the corresponding ODR bit */
                if (config & (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_UP)) == 
                        (hal_ll_gpio_constants::GPIO_CFG_MODE_INPUT | hal_ll_gpio_constants::GPIO_CFG_PULL_UP) 
                {
                    unsafe { *(port_ptr.bsrr as *mut u32) = 1 << (pin_pos + 8); }
                }
            }
        }

        unsafe { *(port_ptr.crh as *mut u32) = tmpreg; }
    } */

}