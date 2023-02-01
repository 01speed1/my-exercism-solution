#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9
    }
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White
    ]
}
