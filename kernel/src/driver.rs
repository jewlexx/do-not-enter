//! Driver Support

/// Driver interfaces.
pub mod interface {
    /// Device Driver functions.
    pub trait DeviceDriver {
        /// Return a compatibility string for identifying the driver.
        fn compatible(&self) -> &'static str;

        /// Called by the kernel to bring up the device.
        ///
        /// # Safety
        ///
        /// - During init, drivers might do stuff with system-wide impact.
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    /// Device driver management functions.
    ///
    /// The `BSP` is supposed to supply one global instance.
    pub trait DriverManager {
        /// Return a slice of references to all `BSP`-instantiated drivers.
        ///
        /// # Safety
        ///
        /// - The order of devices is the order in which `DeviceDriver::init()` is called.
        fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

        /// Initialization code that runs after driver init.
        ///
        /// For example, device driver code that depends on other drivers already being online.
        fn post_device_driver_init(&self);

        /// Minimal code needed to bring up the console in QEMU (for testing only). This is often
        /// less steps than on real hardware due to QEMU's abstractions.
        #[cfg(feature = "test_build")]
        fn qemu_bring_up_console(&self);
    }
}
