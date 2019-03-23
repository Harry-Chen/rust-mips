//! MIPS CP0 Compare register

use crate::registers::cp0_traits::*;

#[derive(Clone, Copy, Debug)]
pub struct CP0Compare {
    bits: u32
}

impl CP0Compare {
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

pub unsafe fn read_cp0_compare() -> u32 {
    cp0_register_read::<CP0Compare>()
}

pub unsafe fn write_cp0_compare(val: u32) {
    cp0_register_write::<CP0Compare>(val);
}

impl CP0Register for CP0Compare {
    const REG_ID : u8 = 11;
}
