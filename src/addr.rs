//! abstraction for virtual and physical addresses

use bit_field::BitField;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(usize);


impl VirtAddr {
    pub fn new(addr: usize) -> VirtAddr {
        VirtAddr(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn p2_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    pub fn p1_index(&self) -> usize {
        self.0.get_bits(12..22)
    }

    pub fn page_number(&self) -> usize {
        self.0.get_bits(12..32)
    }

    pub fn vpn2(&self) -> usize {
        self.0.get_bits(13..32)
    }

    pub fn page_offset(&self) -> usize {
        self.0.get_bits(0..12)
    }

    pub fn to_4k_aligned(&self) -> Self {
        VirtAddr((self.0 >> 12) << 12)
    }

    pub fn from_page_table_indices(p2_index: usize,
                                   p1_index: usize,
                                   offset: usize) -> Self
    {
        assert!(p2_index.get_bits(10..32) == 0, "p2_index exceeding 10 bits");
        assert!(p1_index.get_bits(10..32) == 0, "p1_index exceeding 10 bits");
        assert!(offset.get_bits(12..32) == 0, "offset exceeding 12 bits");
        VirtAddr::new((p2_index << 22) | (p1_index << 12) | offset)
    }

    pub(crate) unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn new(addr: usize) -> PhysAddr {
        PhysAddr(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn p2_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    pub fn p1_index(&self) -> usize {
        self.0.get_bits(12..22)
    }

    pub fn page_number(&self) -> usize {
        self.0.get_bits(12..32)
    }

    pub fn page_offset(&self) -> usize {
        self.0.get_bits(0..12)
    }

    pub fn to_4k_aligned(&self) -> Self {
        PhysAddr((self.0 >> 12) << 12)
    }

    pub fn in_kernel_unmapped(&self) -> bool {
        (self.0 & 0x7fff_ffff) <= 0x1fff_ffff 
    }

    pub fn to_kernel_unmapped(&self) -> VirtAddr {
        assert!(self.in_kernel_unmapped(), "not in kernel unmapped segments");
        VirtAddr(self.0 | 0x8000_0000)
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(VirtAddr);


impl Page {
    pub fn of_addr(addr: VirtAddr) -> Self {
        Page(addr.to_4k_aligned())
    }

    pub fn of_vpn(vpn: usize) -> Self {
        Page(VirtAddr::new(vpn << 12))
    }

    pub fn start_address(&self) -> VirtAddr { self.0.clone() }

    pub fn p2_index(&self) -> usize { self.0.p2_index() }

    pub fn p1_index(&self) -> usize { self.0.p1_index() }

    pub fn number(&self) -> usize { self.0.page_number() }

    pub fn from_page_table_indices(p2_index: usize, p1_index: usize) -> Self {
        use bit_field::BitField;
        let mut addr: usize = 0;
        addr.set_bits(22..32, p2_index);
        addr.set_bits(12..22, p1_index);
        Page::of_addr(VirtAddr::new(addr))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(PhysAddr);


impl Frame {
    pub fn of_addr(addr: PhysAddr) -> Self {
        Frame(addr.to_4k_aligned())
    }

    #[inline(always)]
    pub fn of_ppn(ppn: usize) -> Self {
        Frame(PhysAddr::new(ppn << 12))
    }

    pub fn in_kernel_unmapped(&self) -> bool {
        self.0.in_kernel_unmapped()
    }

    pub fn to_kernel_unmapped(&self) -> VirtAddr {
        self.0.to_kernel_unmapped()
    }

    pub fn start_address(&self) -> PhysAddr { self.0.clone() }

    pub fn p2_index(&self) -> usize { self.0.p2_index() }

    pub fn p1_index(&self) -> usize { self.0.p1_index() }

    pub fn number(&self) -> usize { self.0.page_number() }
}
