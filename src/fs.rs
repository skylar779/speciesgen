use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;
use std::{fs, io};

#[inline]
fn map_create_dir(path: &Path, result: io::Result<()>) -> Result<(), Error> {
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(Error::CreateDirectory(path.to_path_buf(), error)),
    }
}

#[inline]
fn map_ser_json(path: &Path, result: serde_json::Result<()>) -> Result<(), Error> {
    result.map_err(|error| Error::SerializeJson(path.to_path_buf(), error))
}

#[inline]
fn map_de_json<T>(path: &Path, result: serde_json::Result<T>) -> Result<T, Error> {
    result.map_err(|error| Error::DeserializeJson(path.to_path_buf(), error))
}

#[inline]
pub fn create<P>(path: P) -> Result<File, Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    File::create(path).map_err(|error| Error::WriteFile(path.to_path_buf(), error))
}

// unlike std::fs, doesnt error if it already exists
#[inline]
pub fn create_dir<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let result = fs::create_dir(path);

    map_create_dir(path, result)
}

// unlike std::fs, doesnt error if it already exists
#[inline]
pub fn create_dir_all<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let result = fs::create_dir_all(path);

    map_create_dir(path, result)
}

/// Write `value` serialized into JSON to `path`.
#[inline]
pub fn write_json_pretty<P, T>(path: P, value: &T) -> Result<(), Error>
where
    P: AsRef<Path>,
    T: ?Sized + Serialize,
{
    let path = path.as_ref();
    let writer = create(path)?;
    let result = serde_json::to_writer_pretty(writer, value);

    map_ser_json(path, result)?;

    Ok(())
}

#[inline]
pub fn read_json<P, T>(path: P) -> Result<T, Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let file = File::open(path).map_err(|error| Error::OpenFile(path.to_path_buf(), error))?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader);

    let value = map_de_json(path, result)?;

    Ok(value)
}
