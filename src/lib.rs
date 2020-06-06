//! Rust library for low-level abstraction of MIPS32 processors

#![feature(llvm_asm)]
#![no_std]
#![deny(warnings)]
#![cfg_attr(feature = "inline-asm", feature(llvm_asm))]

#[macro_use]
extern crate bitflags;

pub mod instructions;
pub mod registers;
pub mod interrupts;
pub mod tlb;
pub mod addr;
pub mod paging;
