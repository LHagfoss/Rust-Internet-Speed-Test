
use clap::{Parser, Subcommand};
use crate::config::FileSize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the speed test
    Test {
        #[arg(short, long, value_enum, default_value_t = FileSize::Mb100)]
        size: FileSize,

        #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..=60))]
        duration: u64,
    },
    /// Show application information
    About,
    /// Show application version
    Version,
}
