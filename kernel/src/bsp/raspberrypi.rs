//! Top-level BSP file for the Raspberry Pi 3 and 4.

pub mod console;
pub mod cpu;
pub mod driver;
pub mod memory;

/// Board identification.
pub const fn board_name() -> &'static str {
    if cfg!(feature = "bsp_rpi3") {
        "Raspberry Pi 3"
    } else if cfg!(feature = "bsp_rpi4") {
        "Raspberry Pi 4"
    } else {
        "Unknown"
    }
}
