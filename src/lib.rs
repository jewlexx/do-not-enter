#![allow(clippy::upper_case_acronyms)]
#![allow(incomplete_features)]
#![warn(missing_docs)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(asm_const)]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_std]
// Testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

extern crate alloc;

mod panic_wait;

pub mod bsp;
pub mod colorize;
pub mod console;
pub mod cpu;
pub mod driver;
pub mod exception;
pub mod framebuffer;
pub mod mail;
pub mod memory;
pub mod print;
pub mod time;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Version string.
pub fn version() -> &'static str {
    concat!(
        env!("CARGO_PKG_NAME"),
        " version ",
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(not(test))]
extern "Rust" {
    fn kernel_init() -> !;
}
