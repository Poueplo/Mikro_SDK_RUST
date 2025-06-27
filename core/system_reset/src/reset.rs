#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unreachable_code)]


use core::arch::global_asm;

global_asm!(include_str!("startup_assembly.s"));

#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    unsafe extern "Rust" {
        unsafe fn main() -> !;
        unsafe fn system_init();
    }

    unsafe{
        system_init();
        main(); 
    }
    
    loop{} //this is a security as Reset must not return
}