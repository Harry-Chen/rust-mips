//! MIPS specific instructions

pub unsafe fn wait() {
    asm!("wait" : : : : "volatile");
}

pub unsafe fn nop() {
    asm!("nop" : : : : "volatile");
}