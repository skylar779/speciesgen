use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Test,
    Add,
    Remove,
    Copy,
    Move,
    Replace,
}
