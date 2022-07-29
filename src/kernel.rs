#![feature(format_args_nl)]
#![no_main]
#![no_std]

//! Basic Kernel for Raspberry Pi 3/4

extern crate alloc;

use libkernel::*;

use spin::Mutex;

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

/// States whether or not we can allocate memory
pub static CAN_ALLOC: Mutex<bool> = Mutex::new(false);

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order
#[no_mangle]
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;
    use memory::mmu::interface::MMU;

    exception::handling_init();

    if let Err(string) = memory::mmu::mmu().enable_mmu_and_caching() {
        panic!("MMU: {}", string);
    }

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Can now use String, Vec, Box, etc.
    memory::alloc::kernel_init_heap_allocator();
    *CAN_ALLOC.lock() = true;

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
    use driver::interface::DriverManager;

    for line in TITLE_TEXT.lines() {
        info!("{}", line);
    }

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!("MMU online. Special regions:");
    bsp::memory::mmu::virt_mem_layout().print_layout();

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

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

    info!("Initializing framebuffer");

    {
        use ruspiro_mailbox::*;

        let batch = MailboxBatch::empty()
            .with_tag(PhysicalSizeSet::new(128, 64))
            .with_tag(VirtualSizeSet::new(128, 64))
            .with_tag(DepthSet::new(16))
            .with_tag(PixelOrderSet::new(1))
            .with_tag(VirtualOffsetSet::new(0, 0))
            .with_tag(PitchGet::new())
            .with_tag(FramebufferAllocate::new(4));

        let mut mb = Mailbox::new();
        match mb.send_batch(batch) {
            Ok(batch_result) => {
                let fb_base_address = batch_result
                    .get_tag::<FramebufferAllocate, _>()
                    .response()
                    .base_address;
                let fb_pitch = batch_result.get_tag::<PitchGet, _>().response().pitch as isize;

                info!("Drawing to framebuffer");

                for x in 100..200 {
                    for y in 100..200 {
                        let ptr = fb_base_address as *mut u16;
                        unsafe {
                            core::ptr::write_volatile(ptr.offset(x + y * fb_pitch / 16), 0xff);
                        }
                    }
                }
            }
            Err(e) => {
                panic!("Error sending batch: {:?}", e);
            }
        }
    }

    console::enter_echo();
}
