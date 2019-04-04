//! MIPS CP0 EntryLo register

bitflags! {
    pub struct Flags : u32 {
        const DIRTY      = 0b000100;
        const VALID      = 0b000010;
        const GLOBAL     = 0b000001;
        const UNCACHED   = 0b010000;
        const CACHEABLE  = 0b011000;
        const FLAG_MASK  = 0b111111;
        const CACHE_MASK = 0b111000;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EntryLo {
    pub bits: u32,
}

impl EntryLo {
    register_flags!();
    register_field!(get_pfn, set_pfn, 6, 24);
    register_struct_bit_accessor!(
        dirty, set_dirty, reset_dirty, 2);
    register_struct_bit_accessor!(
        valid, set_valid, reset_valid, 1);
    register_struct_bit_accessor!(
        global, set_global, reset_global, 0);
    register_struct_block_setter!(
        set_uncached, Flags::UNCACHED.bits(), Flags::CACHE_MASK.bits());
    register_struct_block_setter!(
        set_cacheable, Flags::CACHEABLE.bits(), Flags::CACHE_MASK.bits());
}

pub mod __entry_lo0 {
    register_rw!(2, 0);
}

pub mod __entry_lo1 {
    register_rw!(3, 0);
}

#[inline]
pub fn read0_u32() -> u32 {
    __entry_lo0::read_u32()
}

#[inline]
pub fn read1_u32() -> u32 {
    __entry_lo1::read_u32()
}

#[inline]
pub fn write0_u32(v: u32) {
    __entry_lo0::write_u32(v);
}

#[inline]
pub fn write1_u32(v: u32) {
    __entry_lo1::write_u32(v);
}

#[inline]
pub fn read0() -> EntryLo {
    EntryLo { bits: read0_u32() }
}

#[inline]
pub fn read1() -> EntryLo {
    EntryLo { bits: read1_u32() }
}

#[inline]
pub fn write0(reg: EntryLo) {
    write0_u32(reg.bits);
}

#[inline]
pub fn write1(reg: EntryLo) {
    write1_u32(reg.bits);
}
