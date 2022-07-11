use std::path::PathBuf;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    CreateDirectory(PathBuf, io::Error),
    CreateFrame,
    CreateFramesMultiple,
    CreateItem,
    CreatePatch,
    CreateMetadata,
    SerializeJson(PathBuf, serde_json::Error),
    DeserializeJson(PathBuf, serde_json::Error),
    WriteFile(PathBuf, io::Error),
    OpenFile(PathBuf, io::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CreateDirectory(path, why) => {
                write!(fmt, "Couldn't create directory {path:?}: {why}")
            }
            Error::CreateFrame => write!(fmt, "Couldn't create frame"),
            Error::CreateFramesMultiple => write!(fmt, "Couldn't create frames"),
            Error::CreateItem => write!(fmt, "Couldn't create item"),
            Error::CreatePatch => write!(fmt, "Couldn't create patch"),
            Error::CreateMetadata => write!(fmt, "Couldn't create metadata"),
            Error::SerializeJson(path, why) => {
                write!(fmt, "Couldn't serialize JSON for {path:?}: {why}")
            }
            Error::DeserializeJson(path, why) => {
                write!(fmt, "Couldn't deserialize JSON from {path:?}: {why}")
            }
            Error::WriteFile(path, why) => write!(fmt, "Couldn't write to file {path:?}: {why}"),
            Error::OpenFile(path, why) => write!(fmt, "Couldn't open file {path:?}: {why}"),
        }
    }
}
