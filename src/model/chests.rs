use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub enum Chests {
    Chest,
    Bust,
    Belly,
    Arms,
}

impl fmt::Display for Chests {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chests::Chest => write!(fmt, "Chest"),
            Chests::Bust => write!(fmt, "Bust"),
            Chests::Belly => write!(fmt, "Belly"),
            Chests::Arms => write!(fmt, "Arms"),
        }
    }
}

impl Default for &Chests {
    #[inline]
    fn default() -> Self {
        &Chests::Chest
    }
}
