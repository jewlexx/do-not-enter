#![warn(missing_docs)]
#![allow(clippy::upper_case_acronyms)]
#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(default_alloc_error_handler)]
#![no_main]
#![no_std]

//! Basic Kernel for Raspberry Pi 3/4

extern crate alloc;

use crate::console::enter_echo;

mod bsp;
mod colorize;
mod console;
mod cpu;
mod driver;
mod framebuffer;
mod mail;
mod memory;
mod panic_wait;
mod print;
mod sync;
mod time;

cfg_if::cfg_if! {
    // Panic if not building for aarch64
    if #[cfg(not(target_arch = "aarch64"))] {
        compile_error!("Must build for aarch64");
    } else if #[cfg(all(feature = "bsp_rpi3", feature = "bsp_rpi4"))] {
        compile_error!("Cannot build for multiple targets");
    } else if #[cfg(not(any(feature = "bsp_rpi3", feature = "bsp_rpi4")))] {
        compile_error!("Must build for Raspberry Pi 3 or 4");
    }
}

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

    // Can now use String, Vec, Box, etc.
    memory::alloc::kernel_init_heap_allocator();

    // Transition from unsafe to safe.
    kernel_main()
}

const TITLE_TEXT: &str = r#"
_____          _   _       _     ______       _
|  __ \        | \ | |     | |   |  ____|     | |
| |  | | ___   |  \| | ___ | |_  | |__   _ __ | |_ ___ _ __
| |  | |/ _ \  | . ` |/ _ \| __| |  __| | '_ \| __/ _ \ '__|
| |__| | (_) | | |\  | (_) | |_  | |____| | | | ||  __/ |
|_____/ \___/  |_| \_|\___/ \__| |______|_| |_|\__\___|_|
"#;

/// The main function running after the early init.
fn kernel_main() -> ! {
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;

    info!("{}", TITLE_TEXT);

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        info!("      {}. {}", i + 1, driver.compatible());
    }

    info!("Spinning for 5 seconds before initializing framebuffer");
    time::time_manager().spin_for(Duration::from_secs(5));

    let fb = framebuffer::FrameBuffer::new(1920, 1080).unwrap();

    fb.draw_rect(150, 150, 400, 400, 0x03 as char, false);

    enter_echo();
}
