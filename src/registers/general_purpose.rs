//! General purpose MIPS registers

pub trait GeneralPurposeRegister {
    const ID: u32;
}

macro_rules! define_gpr {
    ($name: ident, $id: expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name {}
        impl GeneralPurposeRegister for $name {
            const ID: u32 = $id;
        }
    };
}

define_gpr!(zero, 0);
define_gpr!(at, 1);
define_gpr!(v0, 2);
define_gpr!(v1, 3);
define_gpr!(a0, 4);
define_gpr!(a1, 5);
define_gpr!(a2, 6);
define_gpr!(a3, 7);
define_gpr!(t0, 8);
define_gpr!(t1, 9);
define_gpr!(t2, 10);
define_gpr!(t3, 11);
define_gpr!(t4, 12);
define_gpr!(t5, 13);
define_gpr!(t6, 14);
define_gpr!(t7, 15);
define_gpr!(s0, 16);
define_gpr!(s1, 17);
define_gpr!(s2, 18);
define_gpr!(s3, 19);
define_gpr!(s4, 20);
define_gpr!(s5, 21);
define_gpr!(s6, 22);
define_gpr!(s7, 23);
define_gpr!(t8, 24);
define_gpr!(t9, 25);
define_gpr!(k0, 26);
define_gpr!(k1, 27);
define_gpr!(gp, 28);
define_gpr!(sp, 29);
// s8 == fp
define_gpr!(s8, 30);
define_gpr!(fp, 30);
define_gpr!(ra, 31);

#[inline]
pub fn read<R>() -> u32 
where R: GeneralPurposeRegister {
    let value: u32;
    unsafe {
        asm!("ori $0, $$$1, 0"
            : "=r"(value)
            : "i"(R::ID)
            :: "volatile"
        );
    }
    value
}

#[inline]
pub fn write<R>(value: u32)
where R: GeneralPurposeRegister {
    unsafe {
        asm!("ori $$$1, $0, 0"
            :: "r"(value), "i"(R::ID)
            :: "volatile"
        );
    }
}
