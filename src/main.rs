mod cli;
mod day1;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 { path } => day1::solve(path),
    }

    Ok(())
}
