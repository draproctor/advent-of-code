#![warn(clippy::pedantic)]

mod cli;
mod io;
pub mod solver;
pub mod year2015;
pub mod year2023;

use std::collections::HashMap;

use clap::Parser;
use cli::Cli;

use crate::solver::Solver;

include!(concat!(env!("OUT_DIR"), "/module_selection.rs"));

fn main() {
    let args = Cli::parse();

    let solution_selector: HashMap<String, Box<(dyn Solver + 'static)>> = solution_selector();
    let solution_lookup = format!("year{}::day{}", args.year, args.day);

    match solution_selector.get(&solution_lookup) {
        Some(solver) => solver.solve(args.input_file),
        None => println!("There is no solution written for {solution_lookup} yet."),
    }
}
