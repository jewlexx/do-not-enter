#![feature(asm_const)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;

// Panic if not building for aarch64
const _: () = if !cfg!(target_arch = "aarch64") {
    panic!();
};

unsafe fn kernel_init() -> ! {
    println!();
    panic!()
}
