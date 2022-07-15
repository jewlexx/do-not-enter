use core::fmt;
use spin::Mutex;

use crate::console::{
    self,
    interface::{Statistics, Write},
};

struct QEMUOutputInner {
    chars_written: usize,
}

impl QEMUOutputInner {
    const fn new() -> Self {
        QEMUOutputInner { chars_written: 0 }
    }

    fn write_char(&mut self, character: char) {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, character as u8);
        }

        self.chars_written += 1;
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            if character == '\n' {
                self.write_char('\r');
            }

            self.write_char(character);
        }

        Ok(())
    }
}

pub struct QEMUOutput {
    inner: Mutex<QEMUOutputInner>,
}

impl QEMUOutput {
    pub const fn new() -> QEMUOutput {
        QEMUOutput {
            inner: Mutex::new(QEMUOutputInner::new()),
        }
    }
}

impl Write for QEMUOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        let mut inner = &mut *self.inner.lock();

        fmt::Write::write_fmt(&mut inner, args)
    }
}

impl Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        let inner = &*self.inner.lock();

        inner.chars_written
    }
}

impl console::interface::All for QEMUOutput {}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static dyn console::interface::All {
    &QEMU_OUTPUT
}
