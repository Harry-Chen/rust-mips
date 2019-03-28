//! MIPS CP0 EntryLo register

pub trait Flags {
    const DIRTY:     u32 = 0b000100;
    const VALID:     u32 = 0b000010;
    const GLOBAL:    u32 = 0b000001;
    const UNCACHED:  u32 = 0b010000;
    const CACHEABLE:  u32 = 0b011000;
    const FLAG_MASK: u32 = 0b111111;
    const CACHE_MASK: u32 = 0b111000;
}

#[derive(Clone, Copy, Debug)]
pub struct CP0EntryLo {
    pub bits: u32,
}

impl Flags for CP0EntryLo { }

impl CP0EntryLo {
    register_flags!();
    register_field!(get_pfn, set_pfn, 6, 24);
    register_struct_bit_accessor!(
        dirty, set_dirty, reset_dirty, 2);
    register_struct_bit_accessor!(
        valid, set_valid, reset_valid, 1);
    register_struct_bit_accessor!(
        global, set_global, reset_global, 0);
    register_struct_block_setter!(
        set_uncached, Self::UNCACHED, Self::CACHE_MASK);
    register_struct_block_setter!(
        set_cacheable, Self::CACHEABLE, Self::CACHE_MASK);
}

pub mod __cp0_entry_lo0 {
    register_rw!(2, 0);
}

pub mod __cp0_entry_lo1 {
    register_rw!(3, 0);
}

#[inline]
pub fn read0_u32() -> u32 {
    __cp0_entry_lo0::read_u32()
}

#[inline]
pub fn read1_u32() -> u32 {
    __cp0_entry_lo1::read_u32()
}

#[inline]
pub fn write0_u32(v: u32) {
    __cp0_entry_lo0::write_u32(v);
}

#[inline]
pub fn write1_u32(v: u32) {
    __cp0_entry_lo1::write_u32(v);
}

#[inline]
pub fn read0() -> CP0EntryLo {
    CP0EntryLo { bits: read0_u32() }
}

#[inline]
pub fn read1() -> CP0EntryLo {
    CP0EntryLo { bits: read1_u32() }
}

#[inline]
pub fn write0(reg: CP0EntryLo) {
    write0_u32(reg.bits);
}

#[inline]
pub fn write1(reg: CP0EntryLo) {
    write1_u32(reg.bits);
}
