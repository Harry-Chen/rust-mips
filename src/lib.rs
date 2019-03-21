#![feature(asm)]
#![no_std]

pub unsafe fn nop() {
    asm!("nop" : : : : "volatile");
}