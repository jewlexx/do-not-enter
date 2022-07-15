#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

// mod alloc;
// mod colorize;

mod bsp;
mod console;
mod cpu;
mod io;
mod panic_wait;
mod print;
mod vga;

// Panic if not building for aarch64
const _: () = if !cfg!(target_arch = "aarch64") {
    panic!();
};

unsafe fn kernel_init() -> ! {
    use console::console;

    println!("[0] Hello from Rust!");

    println!("[1] Chars written: {}", console().chars_written());

    println!("[2] Stopping here.");

    cpu::wait_forever()
}
