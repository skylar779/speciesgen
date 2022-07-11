use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Headwear,
    Chestwear,
    Legwear,
}
