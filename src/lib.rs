//! Rust library for low-level abstraction of MIPS32 processors

#![feature(asm)]
#![no_std]
#![feature(nll)]
#![cfg_attr(feature = "inline-asm", feature(asm))]

#[macro_use]
extern crate bitflags;

pub mod instructions;
pub mod registers;
pub mod interrupts;
pub mod tlb;
pub mod addr;
pub mod paging;
