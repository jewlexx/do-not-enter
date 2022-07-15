use crate::console;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    console::console().write_fmt(args).unwrap();
}

/// Prints to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints to stdout with a newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));

    ($($arg:tt)*) => ($crate::print::_print(format_args_nl!($($arg)*)));
}
