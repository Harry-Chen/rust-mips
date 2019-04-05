use crate::addr::*;
use crate::registers::cp0::entry_lo::*;
use core::ops::{Index, IndexMut};
use core::fmt::{Debug, Formatter, Error};

pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    /// Clears all entries.
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Debug for PageTable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_map()
            .entries(self.entries.iter().enumerate()
                .filter(|p| !p.1.is_unused()))
            .finish()
    }
}

// MIPS32 4K page with 1K 4-bytes entries.
const ENTRY_COUNT: usize = 1 << 10;

bitflags! {
    /// Possible flags for a page table entry.
    /// VALID      = cp0.entry_lo.valid
    /// WRITABLE   = cp0.entry_lo.dirty
    /// GLOBAL     = cp0.entry_lo.global
    /// CACHEABLE  = cp0.entry_lo.cacheable/uncached
    /// ACCESSED and DIRTY are set by software and used in swap algorithms.
    pub struct PageTableFlags: usize {
        const GLOBAL =      1 << 0;
        const VALID =       1 << 1;
        const WRITABLE =    1 << 2;
//        const READABLE =    1 << 3;
//        const EXECUTABLE =  1 << 4;
        const RESERVED1 =   1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const CACHEABLE =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}

pub type EF = PageTableFlags;

/// Page table entry
///
/// +----------------+-------------------+
/// | PPN ( 24bits ) | flags ( 10 bits ) |
/// +----------------+-------------------+
///
#[derive(Copy, Clone)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }
    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0)
    }
    pub fn ppn(&self) -> usize {
        self.0 >> 10
    }
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.ppn() << 12)
    }
    pub fn frame(&self) -> Frame {
        Frame::of_addr(self.addr())
    }
    pub fn entrylo(&self) -> EntryLo {
        let mut entry = EntryLo {
            bits: (self.0 & 0b111) as u32
        };

        if self.flags().contains(EF::CACHEABLE) {
            entry.set_cacheable();
        } else {
            entry.set_uncached();
        }
        entry.set_pfn(self.ppn() as u32);
        entry
    }
    pub fn set(&mut self, frame: Frame, mut flags: PageTableFlags) {
        flags |= EF::ACCESSED | EF::DIRTY;
        self.0 = (frame.number() << 10) | flags.bits();
    }
    pub fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("PageTableEntry")
            .field("frame", &self.frame())
            .field("flags", &self.flags())
            .finish()
    }
}
