use super::colors::ColorCode;

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_char(&mut self, character: char) -> fmt::Result {
        match character {
            '\n' => self.write_char('\r')?,
            character => {
                if self.column_position >= BUFFER_WIDTH {
                    self.write_char('\n')?;
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: character as u8,
                    color_code,
                };
                self.column_position += 1;
            }
        };

        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            self.write_char(character)?;
        }

        Ok(())
    }
}
