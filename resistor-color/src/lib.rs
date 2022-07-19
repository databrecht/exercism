use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::{Display, Formatter, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, FromPrimitive, EnumIter)]
pub enum ResistorColor {
    Black = 0,
    Blue = 1,
    Brown = 2,
    Green = 3,
    Grey = 4,
    Orange = 5,
    Red = 6,
    Violet = 7,
    White = 8,
    Yellow = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_usize(value) {
        Some(x) => x.to_string(),
        _ => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::iter().collect()
}
