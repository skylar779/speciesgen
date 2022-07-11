use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    Essential,
}

impl fmt::Display for Rarity {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rarity::Common => write!(fmt, "Common"),
            Rarity::Uncommon => write!(fmt, "Uncommon"),
            Rarity::Rare => write!(fmt, "Rare"),
            Rarity::Legendary => write!(fmt, "Legendary"),
            Rarity::Essential => write!(fmt, "Essential"),
        }
    }
}
