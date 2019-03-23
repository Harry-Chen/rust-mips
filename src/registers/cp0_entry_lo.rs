//! MIPS CP0 Count register

use crate::registers::cp0_general::*;

pub trait CP0EntryLoFlags {
    const DIRTY:     u32 = 0b00100;
    const VALID:     u32 = 0b00010;
    const GLOBAL:    u32 = 0b00001;
    const UNCACHED:  u32 = 0b10000;
    const CACHEABLE:  u32 = 0b11000;
    const FLAG_MASK: u32 = 0b11111;
}

macro_rules! generate_entry_lo {
    ($name: ident, $id: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name {
            bits: u32,
        }

        impl CP0EntryLoFlags for $name { }
        impl CP0RegisterTrait for $name {
            const REG_ID : u8 = $id;
        }

        impl $name {
            register_basic_operations!();
            register_flags!();
            register_field!(get_pfn, set_pfn, 6, 24);
        }
    };
}

generate_entry_lo!(CP0EntryLo0, 2);
generate_entry_lo!(CP0EntryLo1, 3);
