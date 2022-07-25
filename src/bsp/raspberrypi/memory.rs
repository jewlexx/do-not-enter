// All const definitions so unused is fine
#[allow(dead_code)]
/// The board's physical memory map.
pub mod map {
    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    cfg_if::cfg_if! {
        if #[cfg(feature = "bsp_rpi3")] {
            pub const START: usize = 0x3F00_0000;
            pub const END_INCLUSIVE: usize = 0x4000_FFFF;
        } else if #[cfg(feature = "bsp_rpi4")] {
            pub const START: usize = 0xFE00_0000;
            pub const END_INCLUSIVE: usize = 0xFF84_FFFF;
        }
    }

    /// Physical devices.
    pub mod mmio {
        use super::*;

        pub use super::{END_INCLUSIVE, START};

        pub const GPIO_START: usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;

        pub const VIDEOCORE_MBOX: usize = START + 0x0000B880;
        pub const MBOX_READ: usize = VIDEOCORE_MBOX;
        pub const MBOX_POLL: usize = VIDEOCORE_MBOX + 0x10;
        pub const MBOX_SENDER: usize = VIDEOCORE_MBOX + 0x14;
        pub const MBOX_STATUS: usize = VIDEOCORE_MBOX + 0x18;
        pub const MBOX_CONFIG: usize = VIDEOCORE_MBOX + 0x1C;
        pub const MBOX_WRITE: usize = VIDEOCORE_MBOX + 0x20;
        pub const MBOX_RESPONSE: usize = 0x80000000;
        pub const MBOX_FULL: usize = 0x80000000;
        pub const MBOX_EMPTY: usize = 0x40000000;
    }
}

pub mod mmu;

use core::cell::UnsafeCell;

// Symbols from the linker script.
extern "Rust" {
    static __code_start: UnsafeCell<()>;
    static __code_end_exclusive: UnsafeCell<()>;
}

/// Start page address of the code segment.
///
/// # Safety
///
/// - Value is provided by the linker script and must be trusted as-is.
#[inline(always)]
fn code_start() -> usize {
    unsafe { __code_start.get() as usize }
}

/// Exclusive end page address of the code segment.
/// # Safety
///
/// - Value is provided by the linker script and must be trusted as-is.
#[inline(always)]
fn code_end_exclusive() -> usize {
    unsafe { __code_end_exclusive.get() as usize }
}
