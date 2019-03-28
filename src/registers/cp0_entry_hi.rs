//! MIPS CP0 EntryHi register

#[derive(Clone, Copy, Debug)]
pub struct CP0EntryHi {
    pub bits: u32,
}

impl CP0EntryHi {
    register_field!(get_vpn2, set_vpn2, 13, 19);
    register_field!(get_asid, set_asid, 0, 8);
}

register_rw!(10, 0);
register_struct_rw!(CP0EntryHi);

#[inline]
pub fn set_entry(vpn2: u32, asid: u32) {
    let mut reg = CP0EntryHi { bits: 0 };
    reg.set_vpn2(vpn2);
    reg.set_asid(asid);
    write(reg);
}
