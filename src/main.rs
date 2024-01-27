#![warn(clippy::pedantic)]

mod cli;
mod io;
pub mod solver;
pub mod year2015;
pub mod year2023;

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;

include!(concat!(env!("OUT_DIR"), "/module_selection.rs"));

fn main() {
    let args = Cli::parse();

    let solution_selector: HashMap<String, Box<dyn Fn(PathBuf)>> = solution_selector();
    let solution_lookup = format!("year{}::day{}", args.year, args.day);

    match solution_selector.get(&solution_lookup) {
        Some(solver) => solver(args.input_file),
        None => println!("There is no solution written for {solution_lookup} yet."),
    }
}
