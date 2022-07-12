use super::Chests;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Subs {
    Busty,
    Milky,
    Hyper,
    Saturated,
    Stuffed,
    Packed,
    Glutted,
    Filled,
    Gorged,
}

impl Subs {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Subs::Busty => "busty",
            Subs::Milky => "milky",
            Subs::Hyper => "hyper",
            Subs::Saturated => "saturated",
            Subs::Stuffed => "stuffed",
            Subs::Packed => "packed",
            Subs::Glutted => "glutted",
            Subs::Filled => "filled",
            Subs::Gorged => "gorged",
        }
    }

    #[inline]
    pub fn as_friendly(&self) -> &'static str {
        match self {
            Subs::Busty => "Busty",
            Subs::Milky => "XBusty",
            Subs::Hyper => "Hyper",
            Subs::Saturated => "Saturated",
            Subs::Stuffed => "Stuffed",
            Subs::Packed => "Packed",
            Subs::Glutted => "Glutted",
            Subs::Filled => "Filled",
            Subs::Gorged => "Gorged",
        }
    }

    #[inline]
    pub fn to_chest(&self) -> Chests {
        match self {
            Subs::Busty | Subs::Milky => Chests::Chest,
            Subs::Hyper => Chests::Bust,
            _ => Chests::Belly,
        }
    }
}

impl fmt::Display for Subs {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl Into<String> for Subs {
    #[inline]
    fn into(self) -> String {
        self.to_string()
    }
}
