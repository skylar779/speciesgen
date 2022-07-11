use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Test,
    Add,
    Remove,
    Copy,
    Move,
    Replace,
}

impl fmt::Display for Op {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Test => write!(fmt, "Test"),
            Op::Add => write!(fmt, "Add"),
            Op::Remove => write!(fmt, "Remove"),
            Op::Copy => write!(fmt, "Copy"),
            Op::Move => write!(fmt, "Move"),
            Op::Replace => write!(fmt, "Replace"),
        }
    }
}
