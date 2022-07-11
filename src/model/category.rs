use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Headwear,
    Chestwear,
    Legwear,
}

impl fmt::Display for Category {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Headwear => write!(fmt, "Headwear"),
            Category::Chestwear => write!(fmt, "Chestwear"),
            Category::Legwear => write!(fmt, "Legwear"),
        }
    }
}
