use crate::command::Export;
use crate::error::Error;
use crate::fs;
use crate::model::Weightstage;
use std::borrow::Cow;
use std::iter;
use std::path::PathBuf;

#[inline]
pub fn export(args: Export, weightstages: Vec<Weightstage>) {
    if let Some(config_file) = args.config {
        if let Err(msg) = export_config(config_file, weightstages) {
            println!("error: {msg}");
        }
    }

    if let Some(color_options_file) = args.color_options {
        export_color_options();
    }
}

#[inline]
fn export_config(config_file: PathBuf, weightstages: Vec<Weightstage>) -> Result<(), Error> {
    fs::write_json_pretty(config_file, &weightstages)?;

    Ok(())
}

#[inline]
fn export_color_options() {}
