//! MIPS specific instructions

macro_rules! define_instruction {
    // specify a different function name
    ($inst: expr, $fun: ident) => {
        #[doc = "invoke `"]
        #[doc = $inst]
        #[doc = "` instruction"]
        pub unsafe fn $fun() {
            asm!($inst : : : : "volatile");
        }
    };

    // directly use instruction name as function name
    ($inst: ident) => {
        define_instruction!(stringify!($inst), $inst);
    }
}

define_instruction!(wait);
define_instruction!(nop);
define_instruction!(tlbr);
define_instruction!(tlbp);
define_instruction!(tlbwr);
define_instruction!(tlbwi);
define_instruction!(syscall);
define_instruction!("break", breakpoint);
define_instruction!("eret", exception_return);
