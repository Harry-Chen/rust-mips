//! MIPS CP0 Count register

use crate::registers::cp0_traits::*;

#[derive(Clone, Copy, Debug)]
pub struct CP0Count {
    bits: u32
}

impl CP0Count {
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }

    pub unsafe fn read() -> Self {
        Self { bits: cp0_register_read::<Self>() }
    }

    pub unsafe fn write(&mut self) {
        cp0_register_write::<Self>(self.bits);
    }
}

pub unsafe fn read_cp0_count() -> u32 {
    cp0_register_read::<CP0Count>()
}

pub unsafe fn write_cp0_count(val: u32) {
    cp0_register_write::<CP0Count>(val);
}

impl CP0Register for CP0Count {
    const REG_ID : u8 = 9;
}
