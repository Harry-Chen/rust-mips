//! MIPS CP0 Cause register

use crate::registers::cp0_general::*;

#[derive(Clone, Copy, Debug)]
pub struct CP0Cause {
    pub bits: u32
}

register_rw!(13, 0);
register_struct_rw!(CP0Cause);
register_set_reset_bit!(
    set_soft_int0,
    reset_soft_int0,
    8
);

register_set_reset_bit!(
    set_soft_int1,
    reset_soft_int1,
    9
);

register_set_reset_bit!(
    set_iv,
    reset_iv,
    23
);

#[derive(Clone, Copy, Debug)]
pub enum Exception {
    Interrupt,
    TLBModification,
    TLBLoadMiss,
    TLBStoreMiss,
    AddressLoadError,
    AddressStoreError,
    BusLoadError,
    BusStoreError,
    Syscall,
    Breakpoint,
    ReservedInstruction,
    CoprocessorUnusable,
    Overflow,
    TrapException,
    MSAFloatingPoint,
    FloatingPoint,
    Reserved1,
    Reserved2,
    Coprocessor2,
    TLBReadInhibit,
    TLBExecutionInhibit,
    MSADisabled,
    MDMX,
    Watch,
    MachineCheck,
    ThreadException,
    DSPDisabled,
    VirtualizedGuest,
    Reserved3,
    Reserved4,
    CacheError,
    Reserved5,
    Unknown
}

#[derive(Clone, Copy, Debug)]
pub enum SoftwareInterrupt {
    SoftInt0,
    SoftInt1
}

impl Exception {
    pub fn from(exccode: u32) -> Self {
        match exccode {
            0 => Exception::Interrupt,
            1 => Exception::TLBModification,
            2 => Exception::TLBLoadMiss,
            3 => Exception::TLBStoreMiss,
            4 => Exception::AddressLoadError,
            5 => Exception::AddressStoreError,
            6 => Exception::BusLoadError,
            7 => Exception::BusStoreError,
            8 => Exception::Syscall,
            9 => Exception::Breakpoint,
            10 => Exception::ReservedInstruction,
            11 => Exception::CoprocessorUnusable,
            12 => Exception::Overflow,
            13 => Exception::TrapException,
            14 => Exception::MSAFloatingPoint,
            15 => Exception::FloatingPoint,
            16 => Exception::Reserved1,
            17 => Exception::Reserved2,
            18 => Exception::Coprocessor2,
            19 => Exception::TLBReadInhibit,
            20 => Exception::TLBExecutionInhibit,
            21 => Exception::MSADisabled,
            22 => Exception::MDMX,
            23 => Exception::Watch,
            24 => Exception::MachineCheck,
            25 => Exception::ThreadException,
            26 => Exception::DSPDisabled,
            27 => Exception::VirtualizedGuest,
            28 => Exception::Reserved3,
            29 => Exception::Reserved4,
            30 => Exception::CacheError,
            31 => Exception::Reserved5,
            _ => Exception::Unknown,
        }
    }
}

impl CP0Cause {
    #[inline]
    pub fn cause(&self) -> Exception {
        // exc_code = cause_reg[6..2]
        Exception::from((self.bits >> 2) & 0x1f)
    }

    #[inline]
    pub fn pending_interrupt(&self) -> u32 {
        // IP = cause_reg[15..11, 9..8]
        let soft_int = (self.bits >> 8) & 0b11;
        let hard_int = (self.bits >> 11) & 0b11111;
        soft_int | (hard_int << 2)
    }
}
