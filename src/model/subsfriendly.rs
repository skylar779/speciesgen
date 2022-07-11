use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub enum SubsFriendly {
    Busty,
    XBusty,
    Hyper,
    Saturated,
    Stuffed,
    Packed,
    Glutted,
    Filled,
    Gorged,
}

impl fmt::Display for SubsFriendly {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubsFriendly::Busty => write!(fmt, "Busty"),
            SubsFriendly::XBusty => write!(fmt, "XBusty"),
            SubsFriendly::Hyper => write!(fmt, "Hyper"),
            SubsFriendly::Saturated => write!(fmt, "Saturated"),
            SubsFriendly::Stuffed => write!(fmt, "Stuffed"),
            SubsFriendly::Packed => write!(fmt, "Packed"),
            SubsFriendly::Glutted => write!(fmt, "Glutted"),
            SubsFriendly::Filled => write!(fmt, "Filled"),
            SubsFriendly::Gorged => write!(fmt, "Gorged"),
        }
    }
}

impl Into<String> for SubsFriendly {
    #[inline]
    fn into(self) -> String {
        self.to_string()
    }
}
