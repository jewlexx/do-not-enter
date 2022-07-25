//! Timer primitives.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/time.rs"]
mod arch_time;

pub use arch_time::time_manager;

/// Timekeeping interfaces.
pub mod interface {
    use core::time::Duration;

    /// Time management functions.
    pub trait TimeManager {
        /// The timer's resolution.
        fn resolution(&self) -> Duration;

        /// The uptime since power-on of the device.
        ///
        /// This includes time consumed by firmware and bootloaders.
        fn uptime(&self) -> Duration;

        /// Spin for a given duration.
        fn spin_for(&self, duration: Duration);
    }
}

#[macro_export]
/// Spins for specified number of seconds, or one if left blank
macro_rules! spin_for_secs {
    ($time:tt) => {
        $crate::time::time_manager().spin_for(core::time::Duration::from_secs($time))
    };

    () => {
        spin_for_secs!(1)
    };
}
