//! A panic handler that infinitely waits.

use alloc::boxed::Box;
use spin::Mutex;

use crate::{cpu, println};
use core::panic::PanicInfo;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

type PanicHook = Box<dyn Fn(&PanicInfo<'_>) + Sync + Send + 'static>;

static PANIC_HOOK: Mutex<Option<PanicHook>> = Mutex::new(None);

/// Stop immediately if called a second time.
///
/// # Note
///
/// Using atomics here relieves us from needing to use `unsafe` for the static variable.
///
/// On `AArch64`, which is the only implemented architecture at the time of writing this,
/// [`AtomicBool::load`] and [`AtomicBool::store`] are lowered to ordinary load and store
/// instructions. They are therefore safe to use even with MMU + caching deactivated.
///
/// [`AtomicBool::load`]: core::sync::atomic::AtomicBool::load
/// [`AtomicBool::store`]: core::sync::atomic::AtomicBool::store
fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    cpu::wait_forever()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::time::interface::TimeManager;

    // Protect against panic infinite loops if any of the following code panics itself.
    panic_prevent_reenter();

    let timestamp = crate::time::time_manager().uptime();
    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    println!(
        "\n\n[  {:>3}.{:06}] Kernel panic!\n\n
        Panic location:\n      File '{location}', line {line}, column {column}\n\n
        {}",
        timestamp.as_secs(),
        timestamp.subsec_micros(),
        info.message().unwrap_or(&format_args!("")),
    );

    cpu::wait_forever()
}
