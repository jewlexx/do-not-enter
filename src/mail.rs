use aligned::{Aligned, A16};

use crate::bsp::memory::map::mmio::*;

pub static mut MBOX: Aligned<A16, [u32; 36]> = Aligned([0_u32; 36]);

unsafe fn mmio_read(src: *const usize) -> usize {
    use core::ptr::read_volatile;

    read_volatile(src)
}

unsafe fn mmio_write(src: usize, dest: *mut usize) {
    use core::ptr::write_volatile;

    write_volatile(dest, src);
}

type MboxPtr = *const Aligned<A16, [u32; 36]>;

unsafe fn mbox_call(val: char) {
    let mbox_ref = (&MBOX as MboxPtr as usize) & !0xF | (val as usize) & 0xF;

    while mmio_read(&MBOX_STATUS) & MBOX_FULL != 0 {}
}
