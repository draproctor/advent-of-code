mod cli;
mod day1;
mod day2;
mod io;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 { path } => day1::solve(path),
        Commands::Day2 { path } => day2::solve(path),
    }

    Ok(())
}
