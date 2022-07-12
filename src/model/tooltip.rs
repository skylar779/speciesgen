use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tooltip {
    Armor,
}

impl fmt::Display for Tooltip {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tooltip::Armor => write!(fmt, "Armor"),
        }
    }
}
