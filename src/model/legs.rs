use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Legs {
    Legs,
    Body,
}

impl fmt::Display for Legs {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Legs::Legs => write!(fmt, "Legs"),
            Legs::Body => write!(fmt, "Body"),
        }
    }
}
