use aligned::{Aligned, A16};

use crate::{bsp::memory::map::mmio::*, println};

pub mod mmio {
    pub mod tags {
        pub const MBOX_TAG_SETPOWER: usize = 0x28001;
        pub const MBOX_TAG_SETCLKRATE: usize = 0x38002;
        pub const MBOX_TAG_SETPHYWH: usize = 0x48003;
        pub const MBOX_TAG_SETVIRTWH: usize = 0x48004;
        pub const MBOX_TAG_SETVIRTOFF: usize = 0x48009;
        pub const MBOX_TAG_SETDEPTH: usize = 0x48005;
        pub const MBOX_TAG_SETPXLORDR: usize = 0x48006;
        pub const MBOX_TAG_GETFB: usize = 0x40001;
        pub const MBOX_TAG_GETPITCH: usize = 0x40008;
        pub const MBOX_TAG_LAST: usize = 0;
    }

    pub mod ch {
        pub const MBOX_CH_POWER: usize = 0x0;
        pub const MBOX_CH_FB: usize = 0x1;
        pub const MBOX_CH_VUART: usize = 0x2;
        pub const MBOX_CH_VCHIQ: usize = 0x3;
        pub const MBOX_CH_LEDS: usize = 0x4;
        pub const MBOX_CH_BTNS: usize = 0x5;
        pub const MBOX_CH_TOUCH: usize = 0x6;
        pub const MBOX_CH_COUNT: usize = 0x7;
        pub const MBOX_CH_PROP: usize = 0x8; // Request from ARM for response by VideoCore
    }

    pub const MBOX_REQUEST: usize = 0x0;
}

pub static mut MBOX: Aligned<A16, [usize; 36]> = Aligned([0usize; 36]);

unsafe fn mmio_read(src: *const usize) -> usize {
    use core::ptr::read_volatile;

    read_volatile(src)
}

unsafe fn mmio_write(src: usize, dest: *mut usize) {
    use core::ptr::write_volatile;

    write_volatile(dest, src);
}

type MboxPtr = *const Aligned<A16, [usize; 36]>;

pub unsafe fn mbox_call(val: char) -> bool {
    // 28-bit address (MSB) and 4-bit value (LSB)
    let mut mbox_ref = (&MBOX as MboxPtr as usize) & !0xF | (val as usize) & 0xF;

    // Wait until we can write
    while mmio_read(&MBOX_STATUS) & MBOX_FULL != 0 {
        println!("Unable to write");
    }

    // Write the address of our buffer to the mailbox with the channel appended
    mmio_write(mbox_ref, MBOX_WRITE as *mut usize);

    loop {
        // Is there a reply?
        while mmio_read(&MBOX_STATUS) & MBOX_EMPTY != 0 {
            println!("No reply");
        }

        println!("{}\n{}", mbox_ref, mmio_read(&MBOX_READ));

        // Is it a reply to our message?
        if mbox_ref == mmio_read(&MBOX_READ) {
            return MBOX[1] == MBOX_RESPONSE;
        }
    }
}
