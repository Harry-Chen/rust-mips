//! Traits for MIPS CP0 registers

pub trait CP0Register {
    const REG_ID : u8;
    const REG_SEL : u8 = 0;
}

pub unsafe fn cp0_register_read<T: CP0Register>() -> usize {
    let x: usize;
    asm!("mfc0 $0, $$$1, $2"
         : "=r"(x)
         : "i"(T::REG_ID), "i"(T::REG_SEL)
    );
    x
}

pub unsafe fn cp0_register_write<T: CP0Register>(val: usize) {
    asm!("mtc0 $0, $$$1, $2"
         :
         : "r"(val), "i"(T::REG_ID), "i"(T::REG_SEL)
         :
         : "volatile"
    );
}