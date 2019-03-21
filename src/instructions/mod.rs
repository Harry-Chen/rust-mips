//! MIPS specific instructions

/// Purpose: Enter Standby Mode
/// Wait for Event
pub unsafe fn wait() {
    asm!("wait" : : : : "volatile");
}

/// Purpose: No Operation
/// To perform no operation.
pub unsafe fn nop() {
    asm!("nop" : : : : "volatile");
}