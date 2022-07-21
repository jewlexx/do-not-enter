use aligned::{Aligned, A16};

use crate::bsp::memory::map::mmio::*;

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

unsafe fn mbox_call(val: char) -> bool {
    // 28-bit address (MSB) and 4-bit value (LSB)
    let mut mbox_ref = (&MBOX as MboxPtr as usize) & !0xF | (val as usize) & 0xF;

    // Wait until we can write
    while mmio_read(&MBOX_STATUS) & MBOX_FULL != 0 {}

    // Write the address of our buffer to the mailbox with the channel appended
    mmio_write(MBOX_WRITE, &mut mbox_ref);

    loop {
        // Is there a reply?
        while mmio_read(&MBOX_STATUS) & MBOX_EMPTY != 0 {}

        // Is it a reply to our message?
        if mbox_ref == mmio_read(&MBOX_READ) {
            return MBOX[1] == MBOX_RESPONSE;
        }
    }
}
