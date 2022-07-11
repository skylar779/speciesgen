use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chests {
    Chest,
    Bust,
    Belly,
}

impl fmt::Display for Chests {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chests::Chest => write!(fmt, "Chest"),
            Chests::Bust => write!(fmt, "Bust"),
            Chests::Belly => write!(fmt, "Belly"),
        }
    }
}
