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

/// Prints to stdout with a newline, debug prefix, but will not print on release
#[macro_export]
#[cfg(not(release))]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::print::_print(format_args_nl!("[Debug] {}{}{}", $crate::colorize::Color::Blue, format_args!($($arg)*),$crate::colorize::Color::Reset)));
}

/// Prints to stdout with a newline, debug prefix, but will not print on release
#[macro_export]
#[cfg(release)]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
