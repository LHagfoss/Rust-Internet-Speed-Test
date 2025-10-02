use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
pub enum FileSize {
    #[value(name = "100MB")]
    Mb100,
    #[value(name = "1GB")]
    Gb1,
    #[value(name = "10GB")]
    Gb10,
}
