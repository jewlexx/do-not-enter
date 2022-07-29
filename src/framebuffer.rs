//! Interaction with the framebuffer

use alloc::boxed::Box;

use crate::debug;

static VGAPAL: [u32; 16] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xAA5500, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF,
];

/// The structure for interaction with the framebuffer
pub struct FrameBuffer {
    /// Display width
    pub width: usize,
    /// Display height
    pub height: usize,
    pitch: isize,
    fb: *mut u16,
}

impl FrameBuffer {
    /// Initialize a new framebuffer
    pub fn new(width: usize, height: usize) -> Result<Self, Box<dyn ruspiro_error::Error + Send>> {
        debug!("Initializing framebuffer");
        use ruspiro_mailbox::*;

        let batch = MailboxBatch::empty()
            .with_tag(PhysicalSizeSet::new(128, 64))
            .with_tag(VirtualSizeSet::new(128, 64))
            .with_tag(DepthSet::new(16))
            .with_tag(PixelOrderSet::new(1))
            .with_tag(VirtualOffsetSet::new(0, 0))
            .with_tag(PitchGet::new())
            .with_tag(FramebufferAllocate::new(4));

        let mut mb = Mailbox::new();

        let batch_result = mb.send_batch(batch)?;

        let fb_base_address = batch_result
            .get_tag::<FramebufferAllocate, _>()
            .response()
            .base_address;
        let fb_pitch = batch_result.get_tag::<PitchGet, _>().response().pitch as isize;

        Ok(Self {
            width,
            height,
            pitch: fb_pitch,
            fb: fb_base_address as *mut u16,
        })
    }

    /// Draw a pixel to the framebuffer
    pub fn draw_pixel(&self, x: isize, y: isize, attr: char) {
        let offs = (y * self.pitch) + (x * 4);

        let offs_ptr = (self.fb as isize + offs) as *mut u32;

        unsafe { *offs_ptr = VGAPAL[attr as usize * 0x0F_usize] }
    }

    /// Draw a rectangle to the framebuffer
    pub fn draw_rect(&self, x1: isize, y1: isize, x2: isize, y2: isize, attr: char, fill: bool) {
        debug!("Drawing");
        let mut y = y1;

        while y < y2 {
            let mut x = x1;

            while x < x2 {
                if (x == x1 || x == x2) || (y == y1 || y == y2) {
                    self.draw_pixel(x, y, attr);
                } else if fill {
                    self.draw_pixel(x, y, ((attr as usize & 0xf0) >> 4) as u8 as char)
                }
                x += 1;
            }
            y += 1;
        }
    }
}
