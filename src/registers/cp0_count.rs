//! MIPS CP0 Count register

use crate::registers::cp0_general::*;

#[derive(Clone, Copy, Debug)]
pub struct CP0Count {
    bits: u32
}

impl CP0RegisterTrait for CP0Count {
    const REG_ID : u8 = 9;
}

impl CP0Count {
    register_basic_operations!();
}
