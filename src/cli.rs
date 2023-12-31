use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to myapp
    Day1 {
        path: PathBuf,
    },
    Day2 {
        path: PathBuf,
    },
    Day3 {
        path: PathBuf,
    },
    Day4 {
        path: PathBuf,
    },
    Day5 {
        path: PathBuf,
    },
    Day6 {
        path: PathBuf,
    },
}
