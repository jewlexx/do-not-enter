/// The board's physical memory map.
#[rustfmt::skip]
pub mod map {

    pub const GPIO_OFFSET:         usize = 0x0020_0000;
    pub const UART_OFFSET:         usize = 0x0020_1000;

    /// Physical devices.
    #[cfg(feature = "bsp_rpi3")]
    pub mod mmio {
        use super::*;

        pub const START:            usize =         0x3F00_0000;
        pub const GPIO_START:       usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;

        pub const VIDEOCORE_MBOX:   usize = START + 0x0000B880;
        pub const MBOX_READ:        usize = VIDEOCORE_MBOX;
        pub const MBOX_POLL:        usize = VIDEOCORE_MBOX + 0x10;
        pub const MBOX_SENDER:      usize = VIDEOCORE_MBOX + 0x14;
        pub const MBOX_STATUS:      usize = VIDEOCORE_MBOX + 0x18;
        pub const MBOX_CONFIG:      usize = VIDEOCORE_MBOX + 0x1C;
        pub const MBOX_WRITE:       usize = VIDEOCORE_MBOX + 0x20;
        pub const MBOX_RESPONSE:    usize = 0x80000000;
        pub const MBOX_FULL:        usize = 0x80000000;
        pub const MBOX_EMPTY:       usize = 0x40000000;
    }

    /// Physical devices.
    #[cfg(feature = "bsp_rpi4")]
    pub mod mmio {
        use super::*;

        pub const START:            usize =         0xFE00_0000;
        pub const GPIO_START:       usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;

        pub const VIDEOCORE_MBOX:   usize = START + 0x0000B880;
        pub const MBOX_READ:        usize = VIDEOCORE_MBOX;
        pub const MBOX_POLL:        usize = VIDEOCORE_MBOX + 0x10;
        pub const MBOX_SENDER:      usize = VIDEOCORE_MBOX + 0x14;
        pub const MBOX_STATUS:      usize = VIDEOCORE_MBOX + 0x18;
        pub const MBOX_CONFIG:      usize = VIDEOCORE_MBOX + 0x1C;
        pub const MBOX_WRITE:       usize = VIDEOCORE_MBOX + 0x20;
        pub const MBOX_RESPONSE:    usize = 0x80000000;
        pub const MBOX_FULL:        usize = 0x80000000;
        pub const MBOX_EMPTY:       usize = 0x40000000;
    }
}
