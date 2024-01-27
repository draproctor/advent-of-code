use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short)]
    pub year: String,

    #[clap(short)]
    pub day: String,

    #[clap(short)]
    pub input_file: PathBuf,
}
