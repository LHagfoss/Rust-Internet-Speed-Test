use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum FileSize {
    #[value(name = "100MB")]
    Mb100,
    #[value(name = "1GB")]
    Gb1,
    #[value(name = "10GB")]
    Gb10,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum, default_value_t = FileSize::Mb100)]
    pub size: FileSize,

    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..=60))]
    pub duration: u64,
}
