//! Interaction with the framebuffer

use alloc::boxed::Box;

mod mailbox;

use crate::debug;

static VGAPAL: [u32; 16] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xAA5500, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF,
];

#[repr(align(16))]
/// The structure for interaction with the framebuffer
pub struct FrameBuffer {
    /// Display width
    pub width: u32,
    /// Display height
    pub height: u32,
    pub vwidth: u32,
    pub vheight: u32,
    pub bytes: u32,
    pub depth: u32,
    pub ignorex: u32,
    pub ignorey: u32,
    pub size: u32,
}

impl FrameBuffer {
    /// Initialize a new framebuffer
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn ruspiro_error::Error + Send>> {
        // Ensure that framebuffer is aligned to 16 bytes
        assert_eq!(core::mem::align_of::<FrameBuffer>(), 16);

        debug!("Initializing framebuffer");

        let fb = Self {
            width,
            height,
            vwidth: width,
            vheight: height,
            depth: 24,
            bytes: 0,
            ignorex: 0,
            ignorey: 0,
            size: 0,
        };

        // let batch = MailboxBatch::empty()
        //     .with_tag(PhysicalSizeSet::new(width, height))
        //     .with_tag(VirtualSizeSet::new(width, height))
        //     .with_tag(DepthSet::new(16))
        //     .with_tag(PixelOrderSet::new(1))
        //     .with_tag(VirtualOffsetSet::new(0, 0))
        //     .with_tag(PitchGet::new())
        //     .with_tag(FramebufferAllocate::new(4));

        // let mut mb = Mailbox::new();

        // let batch_result = mb.send_batch(batch)?;

        // let fb_base_address = batch_result
        //     .get_tag::<FramebufferAllocate, _>()
        //     .response()
        //     .base_address;
        // let fb_pitch = batch_result.get_tag::<PitchGet, _>().response().pitch as isize;

        // Ok(Self {
        //     width,
        //     height,
        //     pitch: fb_pitch,
        //     fb: fb_base_address as *mut u32,
        // })
        todo!()
    }

    /// Test that it works
    pub fn draw_demo(&self) {
        for x in 100..200 {
            for y in 100..200 {
                self.draw_pixel(x, y, 1);
            }
        }
    }

    /// Draw a character to the framebuffer
    pub fn draw_char(&self, character: char, x: isize, y: isize, attr: usize) {
        use crate::font::{
            font_info::{FONT_HEIGHT, FONT_WIDTH},
            FONTS,
        };
        let char_usize = character as usize;

        if char_usize > FONTS.len() {
            panic!("Invalid character {}. Not available in fonts", character);
        }

        let glyph = FONTS[char_usize];

        for hi in 0..FONT_HEIGHT {
            for (wi, pixel) in glyph.iter().enumerate().take(FONT_WIDTH) {
                let pixel = *pixel as usize;
                let mask: usize = hi << wi;
                let col: usize = if pixel & mask == 0 {
                    (attr & 0xF0) >> 4
                } else {
                    attr & 0x0F
                };

                self.draw_pixel(x + wi as isize, y + hi as isize, col);
            }
        }

        debug!("{:?}", glyph);
    }

    /// Draw a pixel to the framebuffer
    pub fn draw_pixel(&self, x: isize, y: isize, attr: usize) {
        let offs = (y * self.pitch) + x;

        unsafe { core::ptr::write_volatile(self.fb.offset(offs), VGAPAL[attr]) };
    }

    /// Draw a rectangle to the framebuffer
    pub fn draw_rect(&self, x1: isize, y1: isize, x2: isize, y2: isize, attr: usize, fill: bool) {
        debug!("Drawing");
        let mut y = y1;

        while y < y2 {
            let mut x = x1;

            while x < x2 {
                if (x == x1 || x == x2) || (y == y1 || y == y2) {
                    self.draw_pixel(x, y, attr);
                } else if fill {
                    self.draw_pixel(x, y, (attr & 0xf0) >> 4)
                }
                x += 1;
            }
            y += 1;
        }
    }
}
