//! Traits for MIPS CP0 registers

macro_rules! register_r {
    ($reg_id: expr, $reg_sel: expr) => {
        #[inline]
        unsafe fn __read() -> u32 {
            let reg: u32;
            llvm_asm!("mfc0 $0, $$$1, $2"
                 : "=r"(reg)
                 : "i"($reg_id), "i"($reg_sel)
            );
            reg
        }

        #[inline]
        pub fn read_u32() -> u32 {
            unsafe { __read() }
        }
    };
}

macro_rules! register_w {
    ($reg_id: expr, $reg_sel: expr) => {
        #[inline]
        unsafe fn __write(reg: u32) {
            llvm_asm!("mtc0 $0, $$$1, $2"
                 :
                 : "r"(reg), "i"($reg_id), "i"($reg_sel)
                 :
                 : "volatile"
            );
        }

        #[inline]
        pub fn write_u32(reg: u32) {
            unsafe { __write(reg); }
        }
    };
}

macro_rules! register_rw {
    ($reg_id: expr, $reg_sel: expr) => {
        register_r!($reg_id, $reg_sel);
        register_w!($reg_id, $reg_sel);
    };
}

macro_rules! register_struct_rw {
    ($ident: ident) => {
        #[inline]
        pub fn read() -> $ident {
            $ident { bits: read_u32() }
        }

        #[inline]
        pub fn write(reg: $ident) {
            write_u32(reg.bits);
        }
    };
}

macro_rules! register_set_bit {
    ($setter: ident, $bit: expr) => {
        #[inline]
        pub fn $setter() {
            write_u32(read_u32() | (1u32 << $bit));
        }
    };
}

macro_rules! register_reset_bit {
    ($resetter: ident, $bit: expr) => {
        #[inline]
        pub fn $resetter() {
            write_u32(read_u32() & !(1u32 << $bit));
        }
    };
}

macro_rules! register_set_reset_bit {
    ($setter: ident, $resetter: ident, $bit: expr) => {
        register_set_bit!($setter, $bit);
        register_reset_bit!($resetter, $bit);
    };
}

macro_rules! register_field {
    ($getter: ident, $setter: ident, $offset: expr, $size: expr) => {
        pub fn $getter(&self) -> u32 {
            (self.bits >> $offset) & ((1 << $size) - 1)
        }

        pub fn $setter(&mut self, val: u32) {
            let mask: u32 = ((1 << $size) - 1) << $offset;
            self.bits = (self.bits & !mask) | ((val << $offset) & mask);
        }
    };
}

macro_rules! register_flags {
    () => {
        #[inline]
        pub fn set_flags(&mut self, flags: Flags) {
            self.bits = (self.bits & !Flags::FLAG_MASK.bits()) | (flags & Flags::FLAG_MASK).bits();
        }

        #[inline]
        pub fn get_flags(&self) -> Flags {
            Flags {
                bits: self.bits & Flags::FLAG_MASK.bits(),
            }
        }
    };
}

macro_rules! register_struct_block_setter {
    ($setter: ident, $val: expr, $mask: expr) => {
        #[inline]
        pub fn $setter(&mut self) {
            self.bits = (self.bits & !$mask) | $val;
        }
    };
}

macro_rules! register_struct_bit_getter {
    ($getter: ident, $bit: expr) => {
        #[inline]
        pub fn $getter(&self) -> bool {
            ((self.bits >> $bit) & 1) != 0
        }
    };
}

macro_rules! register_struct_bit_setter {
    ($setter: ident, $resetter: ident, $bit: expr) => {
        #[inline]
        pub fn $setter(&mut self) {
            self.bits = self.bits | (1 << $bit);
        }

        #[inline]
        pub fn $resetter(&mut self) {
            self.bits = self.bits & !(1 << $bit);
        }
    };
}

macro_rules! register_struct_bit_accessor {
    ($getter: ident, $setter: ident, $resetter: ident, $bit: expr) => {
        register_struct_bit_getter!($getter, $bit);
        register_struct_bit_setter!($setter, $resetter, $bit);
    };
}
