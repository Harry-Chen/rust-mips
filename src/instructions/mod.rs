//! MIPS specific instructions

pub unsafe fn wait() {
    asm!("wait" : : : : "volatile");
}

pub unsafe fn nop() {
    asm!("nop" : : : : "volatile");
}

pub unsafe fn tlbr() {
    asm!("tlbr" : : : : "volatile");
}

pub unsafe fn tlbp() {
    asm!("tlbp" : : : : "volatile");
}

pub unsafe fn tlbwr() {
    asm!("tlbwr" : : : : "volatile");
}

pub unsafe fn tlbwi() {
    asm!("tlbwi" : : : : "volatile");
}

pub unsafe fn syscall() {
    asm!("syscall" : : : : "volatile");
}

pub unsafe fn breakpoint() {
    asm!("break" : : : : "volatile");
}

pub unsafe fn exception_return() {
    asm!("eret" : : : : "volatile");
}
