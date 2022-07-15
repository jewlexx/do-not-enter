use crate::bsp;

pub mod interface {
    pub use core::fmt::Write;
}

pub fn console() -> impl interface::Write {
    bsp::console::console()
}
