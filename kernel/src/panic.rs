//! A panic handler that infinitely waits.

use core::panic::PanicInfo;

use crate::{cpu, println};

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

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

    _panic_exit()
}

/// The point of exit for `libkernel`.
///
/// It is linked weakly, so that the integration tests can overload its standard behavior.
#[linkage = "weak"]
#[no_mangle]
fn _panic_exit() -> ! {
    #[cfg(not(feature = "test_build"))]
    {
        cpu::wait_forever()
    }

    #[cfg(feature = "test_build")]
    {
        cpu::qemu_exit_failure()
    }
}

#[allow(unused)]
pub mod hook {
    //! Interact with panic hooks

    use alloc::boxed::Box;
    use core::panic::PanicInfo;

    use spin::Mutex;

    pub type PanicHook = Box<dyn Fn(&PanicInfo<'_>) + Sync + Send + 'static>;

    static PANIC_HOOK: Mutex<Option<PanicHook>> = Mutex::new(None);

    #[macro_export]
    /// Set the panic hook from a raw function
    macro_rules! set_panic_hook {
        ($hook:expr) => {
            set_panic_hook(alloc::boxed::Box::new($hook));
        };
    }

    /// Set the panic hook from a boxed function
    pub fn set_panic_hook(hook: PanicHook) {
        *PANIC_HOOK.lock() = Some(hook)
    }

    /// Take the current panic hook and remove it
    pub fn take_panic_hook() -> Option<PanicHook> {
        let mut lock = PANIC_HOOK.lock();

        let old_hook = lock.take();

        *lock = None;

        old_hook
    }
}
