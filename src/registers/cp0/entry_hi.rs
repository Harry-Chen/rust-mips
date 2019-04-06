//! MIPS CP0 EntryHi register

#[derive(Clone, Copy, Debug)]
pub struct EntryHi {
    pub bits: u32,
}

impl EntryHi {
    register_field!(get_vpn2, set_vpn2, 13, 19);
    register_field!(get_asid, set_asid, 0, 8);
}

register_rw!(10, 0);
register_struct_rw!(EntryHi);

#[inline]
pub fn set_entry(vpn2: u32, asid: u32) {
    write(new_entry(vpn2, asid));
}

#[inline]
pub fn new_entry(vpn2: u32, asid: u32) -> EntryHi {
    let mut reg = EntryHi { bits: 0 };
    reg.set_vpn2(vpn2);
    reg.set_asid(asid);
    reg
}
