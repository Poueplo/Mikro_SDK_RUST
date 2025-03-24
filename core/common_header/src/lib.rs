#![no_std]

/* Get the value of a register */
pub fn reg_value_get(reg: *mut u32) -> u32 
{   
    unsafe { *reg }
}

/* Get the value of a specific bit in a register */
pub fn reg_value_get_bit(reg: *mut u32, bit: u32) -> u32 
{
    unsafe { *reg & (1u32 << bit) }
}

/* Set a register with a specific value */
pub fn reg_value_set(reg: *mut u32, value: u32) 
{
    unsafe { *reg |= value };
}

/* Set a specific bit in a register */
pub fn reg_value_set_bit(reg: *mut u32, bit: u32) 
{
    unsafe { *reg |= 1u32 << bit };
}

/* Clear a register (set to 0) */
pub fn reg_value_clear(reg: *mut u32) 
{
    unsafe { *reg = 0 };
}

/* Clear a register and set it to a specific value */
pub fn reg_value_clear_set(reg: *mut u32, value: u32) 
{
    unsafe { *reg = value };
}

/* Clear a specific bit in a register */
pub fn reg_value_clear_bit(reg: *mut u32, bit: u32) 
{
    unsafe { *reg &= !(1u32 << bit) };
}

/* Clear specific bits in a register using a mask */
pub fn reg_value_clear_mask(reg: *mut u32, mask: u32) 
{
    unsafe { *reg &= mask };
}


