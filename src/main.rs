#![warn(missing_docs)]
#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

//! Basic Kernel for Raspberry Pi 3/4

// mod alloc;
// mod colorize;

use crate::{console::enter_echo, framebuffer::FrameBuffer};

mod bsp;
mod console;
mod cpu;
mod driver;
mod framebuffer;
mod mail;
mod panic_wait;
mod print;

// Panic if not building for aarch64
const _: () = if !cfg!(target_arch = "aarch64") {
    panic!("Must build for aarch64");
};

const _: () = if cfg!(feature = "bsp_rpi3") && cfg!(feature = "bsp_rpi4") {
    panic!("Cannot build for multiple targets");
};

const _: () = if !cfg!(any(feature = "bsp_rpi3", feature = "bsp_rpi4")) {
    panic!("Must build for either rpi3 or rpi4");
};

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use driver::interface::DriverManager;

    let console = console::console();

    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("[1] Booting on: {}", bsp::board_name());

    println!("[2] Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        println!("      {}. {}", i + 1, driver.compatible());
    }

    println!("[3] Chars written: {}", console.chars_written());

    let fb = unsafe { FrameBuffer::new() }.unwrap();

    fb.draw_rect(150, 150, 400, 400, 0x03 as char, false);

    println!("[4] Echoing input now");

    enter_echo();
}
