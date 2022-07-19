unsafe fn mmio_read(src: *const usize) -> usize {
    use core::ptr::read_volatile;

    read_volatile(src)
}

unsafe fn mmio_write(src: usize, dest: *mut usize) {
    use core::ptr::write_volatile;

    write_volatile(dest, src);
}

const PERIPHERAL_BASE: usize = 0xFE000000;
const VIDEOCORE_MBOX: usize = PERIPHERAL_BASE + 0x0000B880;
const MBOX_READ: usize = VIDEOCORE_MBOX;
const MBOX_POLL: usize = VIDEOCORE_MBOX + 0x10;
const MBOX_SENDER: usize = VIDEOCORE_MBOX + 0x14;
const MBOX_STATUS: usize = VIDEOCORE_MBOX + 0x18;
const MBOX_CONFIG: usize = VIDEOCORE_MBOX + 0x1C;
const MBOX_WRITE: usize = VIDEOCORE_MBOX + 0x20;
const MBOX_RESPONSE: usize = 0x80000000;
const MBOX_FULL: usize = 0x80000000;
const MBOX_EMPTY: usize = 0x40000000;
