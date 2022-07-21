use crate::{
    mail::{
        mbox_call,
        mmio::{ch::*, tags::*, MBOX_REQUEST},
        MBOX,
    },
    println,
};

static VGAPAL: [u32; 16] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xAA5500, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF,
];

pub struct FrameBuffer {
    pitch: usize,
    width: usize,
    height: usize,
    isrgb: bool,
    fb: usize,
}

impl FrameBuffer {
    pub unsafe fn new() -> Option<Self> {
        println!("Initializing framebuffer");
        MBOX[0] = 35 * 4; // Length of message in bytes
        MBOX[1] = MBOX_REQUEST;

        MBOX[2] = MBOX_TAG_SETPHYWH; // Tag identifier
        MBOX[3] = 8; // Value size in bytes
        MBOX[4] = 0;
        MBOX[5] = 1920; // Value(width)
        MBOX[6] = 1080; // Value(height)

        MBOX[7] = MBOX_TAG_SETVIRTWH;
        MBOX[8] = 8;
        MBOX[9] = 8;
        MBOX[10] = 1920;
        MBOX[11] = 1080;

        MBOX[12] = MBOX_TAG_SETVIRTOFF;
        MBOX[13] = 8;
        MBOX[14] = 8;
        MBOX[15] = 0; // Value(x)
        MBOX[16] = 0; // Value(y)

        MBOX[17] = MBOX_TAG_SETDEPTH;
        MBOX[18] = 4;
        MBOX[19] = 4;
        MBOX[20] = 32; // Bits per pixel

        MBOX[21] = MBOX_TAG_SETPXLORDR;
        MBOX[22] = 4;
        MBOX[23] = 4;
        MBOX[24] = 1; // RGB

        MBOX[25] = MBOX_TAG_GETFB;
        MBOX[26] = 8;
        MBOX[27] = 8;
        MBOX[28] = 4096; // FrameBufferInfo.pointer
        MBOX[29] = 0; // FrameBufferInfo.size

        MBOX[30] = MBOX_TAG_GETPITCH;
        MBOX[31] = 4;
        MBOX[32] = 4;
        MBOX[33] = 0; // Bytes per line

        MBOX[34] = MBOX_TAG_LAST;

        println!("Calling mbox");
        if mbox_call(MBOX_CH_PROP) && MBOX[20] == 32 && MBOX[28] != 0 {
            println!("Called mbox");
            MBOX[28] &= 0x3FFFFFFF; // Convert GPU address to ARM address

            Some(Self {
                width: MBOX[10],  // Actual physical width
                height: MBOX[11], // Actual physical height
                pitch: MBOX[33],  // Number of bytes per line
                isrgb: MBOX[24] != 0,
                fb: MBOX[28], // Pixel order
            })
        } else {
            None
        }
    }

    pub fn draw_pixel(&self, x: usize, y: usize, attr: char) {
        let offs = (y * self.pitch) + (x * 4);

        let offs_ptr = (self.fb + offs) as *mut u32;

        unsafe { *offs_ptr = VGAPAL[attr as usize * 0x0F_usize] }
    }

    pub fn draw_rect(&self, x1: usize, y1: usize, x2: usize, y2: usize, attr: char, fill: bool) {
        println!("Drawing");
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
