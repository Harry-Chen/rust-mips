//! control the interrupt processing

use crate::registers::cp0;

/// disable all interrupts
pub fn disable() {
    cp0::status::disable_interrupt();
}

/// enable all interrupts 
pub fn enable() {
    cp0::status::enable_interrupt();
}

/// execute a closure in critical section, which means it will not be interrupted  
/// only works when there is only one processor
pub fn critical_section(f: impl FnOnce()) {
    disable();
    f();
    enable();
}
