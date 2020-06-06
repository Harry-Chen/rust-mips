//! multi-level page table implementation

use super::frame_alloc::*;
use super::page_table::{PageTableFlags as F, *};
use crate::addr::*;
use crate::tlb::*;

pub trait Mapper {
    /// Creates a new mapping in the page table.
    ///
    /// This function might need additional physical frames to create new page tables. These
    /// frames are allocated from the `allocator` argument. At most three frames are required.
    fn map_to(
        &mut self,
        page: Page,
        frame: Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator,
    ) -> Result<MapperFlush, MapToError>;

    /// Removes a mapping from the page table and returns the frame that used to be mapped.
    ///
    /// Note that no page tables or pages are deallocated.
    fn unmap(&mut self, page: Page) -> Result<(Frame, MapperFlush), UnmapError>;

    /// Get the reference of the specified `page` entry
    fn ref_entry(&mut self, page: Page) -> Result<&mut PageTableEntry, FlagUpdateError>;

    /// Updates the flags of an existing mapping.
    fn update_flags(
        &mut self,
        page: Page,
        flags: PageTableFlags,
    ) -> Result<MapperFlush, FlagUpdateError> {
        self.ref_entry(page).map(|e| {
            *e.flags_mut() = flags;
            MapperFlush::new(page)
        })
    }

    /// Return the frame that the specified page is mapped to.
    fn translate_page(&mut self, page: Page) -> Option<Frame> {
        match self.ref_entry(page) {
            Ok(e) => {
                if e.is_unused() {
                    None
                } else {
                    Some(e.frame())
                }
            }
            Err(_) => None,
        }
    }

    /// Maps the given frame to the virtual page with the same address.
    fn identity_map(
        &mut self,
        frame: Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator,
    ) -> Result<MapperFlush, MapToError> {
        let page = Page::of_addr(VirtAddr::new(frame.start_address().as_usize()));
        self.map_to(page, frame, flags, allocator)
    }
}

#[must_use = "Page Table changes must be flushed or ignored."]
pub struct MapperFlush(Page);

impl MapperFlush {
    /// Create a new flush promise
    fn new(page: Page) -> Self {
        MapperFlush(page)
    }

    /// Flush the page from the TLB to ensure that the newest mapping is used.
    pub fn flush(self) {
        TLBEntry::clear_all();
    }

    /// Don't flush the TLB and silence the “must be used” warning.
    pub fn ignore(self) {}
}

/// This error is returned from `map_to` and similar methods.
#[derive(Debug)]
pub enum MapToError {
    /// An additional frame was needed for the mapping process, but the frame allocator
    /// returned `None`.
    FrameAllocationFailed,
    /// An upper level page table entry has the `HUGE_PAGE` flag set, which means that the
    /// given page is part of an already mapped huge page.
    ParentEntryHugePage,
    /// The given page is already mapped to a physical frame.
    PageAlreadyMapped,
}

/// An error indicating that an `unmap` call failed.
#[derive(Debug)]
pub enum UnmapError {
    /// An upper level page table entry has the `HUGE_PAGE` flag set, which means that the
    /// given page is part of a huge page and can't be freed individually.
    ParentEntryHugePage,
    /// The given page is not mapped to a physical frame.
    PageNotMapped,
    /// The page table entry for the given page points to an invalid physical address.
    InvalidFrameAddress(PhysAddr),
}

/// An error indicating that an `update_flags` call failed.
#[derive(Debug)]
pub enum FlagUpdateError {
    /// The given page is not mapped to a physical frame.
    PageNotMapped,
}

/// This struct is a two level page table with `Mapper` trait implemented.
pub struct TwoLevelPageTable<'a> {
    root_table: &'a mut PageTable,
}

impl<'a> TwoLevelPageTable<'a> {
    pub fn new(table: &'a mut PageTable) -> Self {
        TwoLevelPageTable { root_table: table }
    }

    fn create_p1_if_not_exist(
        &mut self,
        p2_index: usize,
        allocator: &mut impl FrameAllocator,
    ) -> Result<&mut PageTable, MapToError> {
        if self.root_table[p2_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            self.root_table[p2_index].set(frame, F::VALID);
            let p1_table: &mut PageTable = unsafe { frame.to_kernel_unmapped().as_mut() };
            p1_table.zero();
            Ok(p1_table)
        } else {
            let frame = self.root_table[p2_index].frame();
            let p1_table: &mut PageTable = unsafe { frame.to_kernel_unmapped().as_mut() };
            Ok(p1_table)
        }
    }
}

impl<'a> Mapper for TwoLevelPageTable<'a> {
    fn map_to(
        &mut self,
        page: Page,
        frame: Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator,
    ) -> Result<MapperFlush, MapToError> {
        let p1_table = self.create_p1_if_not_exist(page.p2_index(), allocator)?;
        if !p1_table[page.p1_index()].is_unused() {
            return Err(MapToError::PageAlreadyMapped);
        }
        p1_table[page.p1_index()].set(frame, flags);
        Ok(MapperFlush::new(page))
    }

    fn unmap(&mut self, page: Page) -> Result<(Frame, MapperFlush), UnmapError> {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame();
        let p1_table: &mut PageTable = unsafe { p1_frame.to_kernel_unmapped().as_mut() };
        let p1_entry = &mut p1_table[page.p1_index()];
        if !p1_entry.flags().contains(F::VALID) {
            return Err(UnmapError::PageNotMapped);
        }
        let frame = p1_entry.frame();
        p1_entry.set_unused();
        Ok((frame, MapperFlush::new(page)))
    }

    fn ref_entry(&mut self, page: Page) -> Result<&mut PageTableEntry, FlagUpdateError> {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame();
        let p1_table: &mut PageTable = unsafe { p1_frame.to_kernel_unmapped().as_mut() };
        Ok(&mut p1_table[page.p1_index()])
    }
}
