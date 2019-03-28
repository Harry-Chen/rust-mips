
use crate::registers::cp0;

pub fn critical_section(f: &Fn() -> ()) {
    cp0::status::disable_interrupt();
    f();
    cp0::status::enable_interrupt();
}
