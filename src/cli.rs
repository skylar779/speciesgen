use crate::command::Commands;
use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "skylar779",
    version,
    about,
    long_about = "speciesgen is a mod generator for Starbound to add support for modded species in the Big Fatties mod."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
