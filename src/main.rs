#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

mod alloc;
mod bsp;
mod colorize;
mod console;
mod cpu;
mod panic_wait;
mod print;

use colorize::Colorize;

use crate::{alloc::init_alloc, colorize::Colors};

// Panic if not building for aarch64
const _: () = if !cfg!(target_arch = "aarch64") {
    panic!();
};

unsafe fn kernel_init() -> ! {
    init_alloc();

    use console::console;

    println!("[0] Hello from Rust!");

    println!("[1] Chars written: {}", console().chars_written());

    println!("[2] Stopping here. {}", "yo".colorize(Colors::Red));

    cpu::wait_forever()
}
