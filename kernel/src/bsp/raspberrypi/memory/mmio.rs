//! BSP MMIO

/// The start of the mmio
pub const MMIO_START: usize = 0x3F000000;

/// Write to a point in mmio
pub fn write_at_offset(data: u32, offset: usize) {
    unsafe {
        core::ptr::write_volatile((MMIO_START + offset) as *mut u32, data);
    }
}

/// Read from a point in mmio
pub fn read_at_offset(offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile((MMIO_START + offset) as *const u32) }
}
