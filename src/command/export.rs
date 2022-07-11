use crate::command::Export;
use crate::error::Error;
use crate::fs;
use crate::model::Weightstage;
use std::path::PathBuf;

#[inline]
pub fn export(args: Export, weightstages: Vec<Weightstage>) {
    if let Some(config_file) = args.config {
        if let Err(msg) = export_config(config_file, weightstages) {
            println!("error: {msg}");
        }
    }
}

#[inline]
fn export_config(path: PathBuf, weightstages: Vec<Weightstage>) -> Result<(), Error> {
    fs::write_json_pretty(path, &weightstages)?;

    Ok(())
}
