unsafe fn mmio_read(src: *const usize) -> usize {
    use core::ptr::read_volatile;

    read_volatile(src)
}

unsafe fn mmio_write(src: usize, dest: *mut usize) {
    use core::ptr::write_volatile;

    write_volatile(dest, src);
}

unsafe fn mbox_call(val: char) {
    while (mmio_read(MBOX_STATUS) & MBOX_FULL) {}
}
