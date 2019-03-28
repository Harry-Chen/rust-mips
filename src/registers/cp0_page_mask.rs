//! MIPS CP0 PageMask register

#[derive(Clone, Copy, Debug)]
pub struct CP0PageMask {
    pub bits: u32,
}

impl CP0PageMask {
    register_field!(get_mask, set_mask, 13, 16);
}

register_rw!(5, 0);
register_struct_rw!(CP0PageMask);

#[inline]
pub fn set_mask(mask: u32) {
    let mut reg = CP0PageMask { bits: 0 };
    reg.set_mask(mask);
    write(reg);
}
