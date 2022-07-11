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

impl fmt::Display for Subs {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Subs::Busty => write!(fmt, "busty"),
            Subs::Milky => write!(fmt, "milky"),
            Subs::Hyper => write!(fmt, "hyper"),
            Subs::Saturated => write!(fmt, "saturated"),
            Subs::Stuffed => write!(fmt, "stuffed"),
            Subs::Packed => write!(fmt, "packed"),
            Subs::Glutted => write!(fmt, "glutted"),
            Subs::Filled => write!(fmt, "filled"),
            Subs::Gorged => write!(fmt, "gorged"),
        }
    }
}

impl Into<String> for Subs {
    #[inline]
    fn into(self) -> String {
        self.to_string()
    }
}
