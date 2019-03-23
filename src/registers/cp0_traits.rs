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

        #[inline]
        pub fn set_bits(&mut self, val: u32) {
            self.bits = val;
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

macro_rules! register_flags {
    () => {
        #[inline]
        pub fn set_flags(&mut self, flags: u32) {
            self.bits = self.bits | (flags & Self::FLAG_MASK);
        }

        #[inline]
        pub fn reset_flags(&mut self, flags: u32) {
            self.bits = self.bits & !(flags & Self::FLAG_MASK);
        }
    };
}

macro_rules! register_fleld {
    ($getter: ident, $setter: ident, $offset: expr, $size: expr) => {
        pub fn $getter(&self) -> u32 {
            (self.bits >> $offset) & ((1 << $size) - 1)
        }

        pub fn $setter(&mut self, val: u32) {
            self.bits = (val & ((1 << $size) - 1)) << $offset;
        }
    };
}
