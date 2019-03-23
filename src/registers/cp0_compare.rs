//! MIPS CP0 Compare register

use crate::registers::cp0_general::*;

#[derive(Clone, Copy, Debug)]
pub struct CP0Compare {
    bits: u32
}

impl CP0RegisterTrait for CP0Compare {
    const REG_ID : u8 = 11;
}

impl CP0Compare {
    register_basic_operations!();
}
