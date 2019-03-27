//! Traits for MIPS CP0 registers

macro_rules! register_basic_operations {
    () => {
        #[inline]
        pub fn bits(&self) -> u32 {
            self.bits
        }

        #[inline]
        pub fn set_bits(&mut self, val: u32) {
            self.bits = val;
        }
    };
}

macro_rules! register_r {
    ($reg_id: expr, $reg_sel: expr) => {
        #[inline]
        unsafe fn __read() -> u32 {
            let reg: u32;
            asm!("mfc0 $0, $$$1, $2"
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
            asm!("mtc0 $0, $$$1, $2"
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
            self.bits = (val & ((1 << $size) - 1)) << $offset;
        }
    };
}
