use core::fmt::Display;

use alloc::{format, string::String};

#[allow(dead_code)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl Colors {
    pub fn get_code(&self) -> &'static str {
        match self {
            Colors::Black => "30",
            Colors::Red => "31",
            Colors::Green => "32",
            Colors::Yellow => "33",
            Colors::Blue => "34",
            Colors::Magenta => "35",
            Colors::Cyan => "36",
            Colors::White => "37",
            Colors::BrightBlack => "30;1",
            Colors::BrightRed => "31;1",
            Colors::BrightGreen => "32;1",
            Colors::BrightYellow => "33;1",
            Colors::BrightBlue => "34;1",
            Colors::BrightMagenta => "35;1",
            Colors::BrightCyan => "36;1",
            Colors::BrightWhite => "37;1",
            Colors::Reset => "0",
        }
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\u{001b}[{}m", self.get_code())
    }
}

pub trait Colorize {
    fn colorize(self, color: Colors) -> String
    where
        Self: Sized + Display,
    {
        format!("{}{}{}", color, self, Colors::Reset)
    }
}
