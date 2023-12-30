#![warn(clippy::pedantic)]

mod cli;
mod day1;
mod day2;
mod day3;
mod day4;
mod io;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Day1 { path } => day1::solve(path),
        Commands::Day2 { path } => day2::solve(&path),
        Commands::Day3 { path } => day3::solve(path),
        Commands::Day4 { path } => day4::solve(path),
    }
}
