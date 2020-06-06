//! MIPS CP0 Config register

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub config0: u32,
    pub config1: u32,
    pub config2: u32,
    pub config3: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum EndianMode {
    LittleEndian,
    BigEndian,
}

#[derive(Clone, Copy, Debug)]
pub enum MMUType {
    NoMMU,
    StandardTLB,
    StandardBAT,
    StandardFixed,
    Unknown,
}

impl Config {
    pub fn endian(&self) -> EndianMode {
        if ((self.config0 >> 15) & 1) == 1 {
            EndianMode::BigEndian
        } else {
            EndianMode::LittleEndian
        }
    }

    pub fn mmu_type(&self) -> MMUType {
        match (self.config0 >> 7) & 7 {
            0 => MMUType::NoMMU,
            1 => MMUType::StandardTLB,
            2 => MMUType::StandardBAT,
            3 => MMUType::StandardFixed,
            _ => MMUType::Unknown,
        }
    }

    pub fn mmu_size(&self) -> u32 {
        (self.config1 >> 25) & 0b111111
    }

    pub fn icache_per_way(&self) -> u32 {
        64 << ((self.config1 >> 22) & 0b111)
    }

    pub fn icache_line_size(&self) -> u32 {
        let il: u32 = (self.config1 >> 19) & 0b111;
        if il == 0 {
            0
        } else {
            2 << il
        }
    }

    pub fn icache_associativity(&self) -> u32 {
        let is: u32 = (self.config1 >> 16) & 0b111;
        if is == 0 {
            0
        } else {
            is + 1
        }
    }

    pub fn dcache_per_way(&self) -> u32 {
        64 << ((self.config1 >> 13) & 0b111)
    }

    pub fn dcache_line_size(&self) -> u32 {
        let dl: u32 = (self.config1 >> 10) & 0b111;
        if dl == 0 {
            0
        } else {
            2 << dl
        }
    }

    pub fn dcache_associativity(&self) -> u32 {
        let ds: u32 = (self.config1 >> 7) & 0b111;
        if ds == 0 {
            0
        } else {
            ds + 1
        }
    }

    pub fn has_cp2(&self) -> bool {
        ((self.config1 >> 6) & 1) == 1
    }

    pub fn has_performance_counter(&self) -> bool {
        ((self.config1 >> 4) & 1) == 1
    }

    pub fn has_watch_regs(&self) -> bool {
        ((self.config1 >> 3) & 1) == 1
    }

    pub fn has_mips16(&self) -> bool {
        ((self.config1 >> 2) & 1) == 1
    }

    pub fn has_ejtag(&self) -> bool {
        ((self.config1 >> 1) & 1) == 1
    }

    pub fn has_fpu(&self) -> bool {
        (self.config1 & 1) == 1
    }
}

pub mod __config0 {
    register_r!(16, 0);
}
pub mod __config1 {
    register_r!(16, 1);
}
pub mod __config2 {
    register_r!(16, 2);
}
pub mod __config3 {
    register_r!(16, 3);
}

pub fn read() -> Config {
    Config {
        config0: __config0::read_u32(),
        config1: __config1::read_u32(),
        config2: __config2::read_u32(),
        config3: __config3::read_u32(),
    }
}

pub fn mmu_size() -> u32 {
    (__config1::read_u32() >> 25) & 0b111111
}
