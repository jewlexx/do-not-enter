use aligned::{Aligned, A16};
use spin::Mutex;

use crate::{bsp::memory::map::mmio::*, debug};

// All const definitions so unused is fine
#[allow(dead_code)]
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

pub static MBOX: Mutex<Aligned<A16, [usize; 36]>> = Mutex::new(Aligned([0usize; 36]));

unsafe fn mmio_read(src: *const usize) -> usize {
    core::ptr::read_volatile(src)
}

unsafe fn mmio_write<T: core::fmt::Debug>(src: T, dest: *mut T) {
    debug!("Setting {:?} to {:?}", dest, src);
    core::ptr::write_volatile(dest, src);
}

type MboxPtr = *const Aligned<A16, [usize; 36]>;

pub unsafe fn mbox_call(val: usize) -> bool {
    // 28-bit address (MSB) and 4-bit value (LSB)
    let mbox_ref = ((&*MBOX.lock()) as MboxPtr as usize) & !0xF | val & 0xF;

    // Wait until we can write
    while mmio_read(MBOX_STATUS as *const usize) & MBOX_FULL != 0 {
        debug!("Unable to write");
    }
    debug!("About to write");

    // Write the address of our buffer to the mailbox with the channel appended
    mmio_write(mbox_ref, MBOX_WRITE as *mut usize);

    debug!("Wrote");

    loop {
        // Is there a reply?
        while (mmio_read(MBOX_STATUS as *const usize) & MBOX_EMPTY) != 0 {
            debug!("No reply");
        }

        // Is it a reply to our message?
        if mbox_ref == mmio_read(MBOX_READ as *const usize) {
            debug!("Got reply");
            return MBOX.lock()[1] == MBOX_RESPONSE;
        }
    }
}
