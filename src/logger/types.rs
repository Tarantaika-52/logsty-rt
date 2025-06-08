use std::fmt::{Display, Formatter};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_code = match self {
            Color::Black => "0",
            Color::Red => "1",
            Color::Green => "2",
            Color::Yellow => "3",
            Color::Blue => "4",
            Color::Purple => "5",
            Color::Cyan => "6",
            Color::White => "7",
            Color::Fatal => "0;41"
        };

        write!(f, "{color_code}")
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Fatal,
}