//! Interaction with the framebuffer

use crate::{
    debug,
    mail::{
        mbox_call,
        mmio::{ch::*, tags::*, MBOX_REQUEST},
        MBOX,
    },
};

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
    /// Is the display RGB
    pub is_rgb: bool,
    pitch: usize,
    fb: usize,
}

impl FrameBuffer {
    /// Initialize a new framebuffer
    pub fn new(width: usize, height: usize) -> Option<Self> {
        debug!("Initializing framebuffer");
        {
            let mut inner = MBOX.lock();
            inner[0] = 35 * 4; // Length of message in bytes
            inner[1] = MBOX_REQUEST;

            inner[2] = MBOX_TAG_SETPHYWH; // Tag identifier
            inner[3] = 8; // Value size in bytes
            inner[4] = 0;
            inner[5] = 1920; // Value(width)
            inner[6] = 1080; // Value(height)

            inner[7] = MBOX_TAG_SETVIRTWH;
            inner[8] = 8;
            inner[9] = 8;
            inner[10] = width;
            inner[11] = height;

            inner[12] = MBOX_TAG_SETVIRTOFF;
            inner[13] = 8;
            inner[14] = 8;
            inner[15] = 0; // Value(x)
            inner[16] = 0; // Value(y)

            inner[17] = MBOX_TAG_SETDEPTH;
            inner[18] = 4;
            inner[19] = 4;
            inner[20] = 32; // Bits per pixel

            inner[21] = MBOX_TAG_SETPXLORDR;
            inner[22] = 4;
            inner[23] = 4;
            inner[24] = 1; // RGB

            inner[25] = MBOX_TAG_GETFB;
            inner[26] = 8;
            inner[27] = 8;
            inner[28] = 4096; // FrameBufferInfo.pointer
            inner[29] = 0; // FrameBufferInfo.size

            inner[30] = MBOX_TAG_GETPITCH;
            inner[31] = 4;
            inner[32] = 4;
            inner[33] = 0; // Bytes per line

            inner[34] = MBOX_TAG_LAST;
        };

        debug!("Calling mbox");
        let fbinfo_ptr = MBOX.lock()[28];
        let bbl = MBOX.lock()[20];

        if unsafe { mbox_call(MBOX_CH_PROP) } && bbl == 32 && fbinfo_ptr != 0 {
            debug!("Called mbox");

            let mut inner = MBOX.lock();
            inner[28] &= 0x3FFFFFFF; // Convert GPU address to ARM address

            Some(Self {
                width: inner[10],  // Actual physical width
                height: inner[11], // Actual physical height
                pitch: inner[33],  // Number of bytes per line
                is_rgb: inner[24] != 0,
                fb: inner[28] as *const usize as usize, // Pixel order
            })
        } else {
            None
        }
    }

    /// Draw a pixel to the framebuffer
    pub fn draw_pixel(&self, x: usize, y: usize, attr: char) {
        let offs = (y * self.pitch) + (x * 4);

        let offs_ptr = (self.fb + offs) as *mut u32;

        unsafe { *offs_ptr = VGAPAL[attr as usize * 0x0F_usize] }
    }

    /// Draw a rectangle to the framebuffer
    pub fn draw_rect(&self, x1: usize, y1: usize, x2: usize, y2: usize, attr: char, fill: bool) {
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
