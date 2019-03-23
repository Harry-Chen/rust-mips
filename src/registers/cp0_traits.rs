//! Traits for MIPS CP0 registers

pub trait CP0RegisterTrait {
    const REG_ID : u8;
    const REG_SEL : u8 = 0;
}

macro_rules! generate_register_info {
    () => {
        #[inline]
        pub fn bits(&self) -> u32 {
            self.bits
        }

        pub unsafe fn read(&mut self) {
            asm!("mfc0 $0, $$$1, $2"
                 : "=r"(self.bits)
                 : "i"(Self::REG_ID), "i"(Self::REG_SEL)
            );
        }

        pub unsafe fn write(&self) {
            asm!("mtc0 $0, $$$1, $2"
                 :
                 : "r"(self.bits), "i"(Self::REG_ID), "i"(Self::REG_SEL)
                 :
                 : "volatile"
            );
        }
    };
}
