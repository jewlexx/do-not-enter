//! Printing.

use crate::console;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    console::console()
        .write_fmt(format_args!("{}{}", args, crate::colorize::Color::Reset))
        .unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}

/// Prints with extra information
#[doc(hidden)]
#[macro_export]
macro_rules! print_extra {
    ($prefix:expr, $color:expr, $string:expr) => ({
        use $crate::time::interface::TimeManager;

        let timestamp = $crate::time::time_manager().uptime();

        $crate::print::_print(format_args_nl!(
            "[{} {:>3}.{:06}] {}{}",
            $prefix,
            timestamp.as_secs(),
            timestamp.subsec_micros(),
            $color,
            $string,
        ));
    });
    ($prefix:expr, $color:expr, $format_string:expr, $($arg:tt)*) => ({
        $crate::print_extra!($prefix, $color, format_args!($format_string, $($arg)*));
    })}

/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ($crate::print_extra!(" ", $crate::colorize::Color::Reset, $string));
    ($format_string:expr, $($arg:tt)*) => ($crate::print_extra!(" ", $crate::colorize::Color::Reset, $format_string, $($arg)*));
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => ($crate::print_extra!("W", $crate::colorize::Color::Yellow, $string));
    ($format_string:expr, $($arg:tt)*) => ($crate::print_extra!(" ", $crate::colorize::Color::Yellow, $format_string, $($arg)*));
}

/// Prints a debug message, with a newline
#[macro_export]
macro_rules! debug {
    ($string:expr) => ($crate::print_extra!("D", $crate::colorize::Color::TrueColor { r: 128, g: 128, b: 128 }, $string));
    ($format_string:expr, $($arg:tt)*) => ($crate::print_extra!("D", $crate::colorize::Color::TrueColor { r: 128, g: 128, b: 128 }, $format_string, $($arg)*));
}

/// Prints to stdout with a newline, debug prefix, but will not print on release
#[macro_export]
#[cfg(release)]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
