pub use build::build;
use clap::{Args, Subcommand};
pub use export::export;
use std::path::PathBuf;

mod build;
mod export;

#[derive(Args)]
pub struct Build {
    /// Set the name of the species.
    #[clap(short, long, value_parser, value_name = "NAME")]
    pub species: String,

    /// Set the friendly name of the species.
    #[clap(long = "friendly-species", value_parser, value_name = "NAME")]
    pub friendly_species: String,

    /// Set the name of the author in the metadata.
    #[clap(long, value_parser, value_name = "NAME")]
    #[cfg_attr(unix, clap(env = "USER"))]
    #[cfg_attr(windows, clap(env = "USERNAME"))]
    pub author: String,

    /// Set the output directory. [default: <SPECIES>support]
    #[clap(short, long, value_parser, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Import a JSON file containing weightstages, otherwise the application will assume the default weightstages.
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Args)]
pub struct Export {
    /// Export the default config.
    #[clap(long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a mod.
    #[clap(arg_required_else_help = true)]
    Build(Build),

    /// Export various default configurations from the application.
    #[clap(arg_required_else_help = true)]
    Export(Export),
}
