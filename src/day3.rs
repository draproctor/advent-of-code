use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use crate::io::LineExtractor;

const NON_SYMBOLS: &str = "01234566789.";

pub fn solve(path: PathBuf) {
    let content = File::open(path)
        .expect("Cannot open file")
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let symbol_tracker = symbol_tracker(content);
    let part_number_sum = &symbol_tracker
        .values()
        .map(|numbers| numbers.iter().sum::<i32>())
        .sum::<i32>();
    let power_of_parts_touching_gears = &symbol_tracker
        .values()
        .filter(|parts| parts.len() == 2)
        .filter_map(|parts| {
            if parts.len() == 2 {
                Some(parts.iter().product::<i32>())
            } else {
                None
            }
        })
        .sum::<i32>();
    println!("Sum of part numbers touching symbols: {part_number_sum}");
    println!("Power of parts touching gears: {power_of_parts_touching_gears}");
}

fn symbol_tracker(content: Vec<String>) -> HashMap<(i32, i32), Vec<i32>> {
    let board = create_board(&content);
    let symbol_positions = find_symbols(&board);
    let mut symbols_next_to_parts = HashMap::new();

    for (row_index, row) in content.iter().enumerate() {
        for number in extract_numbers(row) {
            let edge_of_number = edge_of_number(row_index as i32, number);
            for coords in &symbol_positions {
                if edge_of_number.contains(coords) {
                    let int = number.as_str().parse::<i32>().unwrap();
                    symbols_next_to_parts
                        .entry(*coords)
                        .or_insert(Vec::new())
                        .push(int);
                }
            }
        }
    }
    symbols_next_to_parts
}

fn edge_of_number(row_index: i32, number: Match<'_>) -> HashSet<(i32, i32)> {
    [row_index - 1, row_index, row_index + 1]
        .iter()
        .flat_map(|y| {
            let start = number.start() as i32 - 1;
            let end = number.end() as i32 + 1;
            (start..end).filter_map(move |x| if x < 0 || *y < 0 { None } else { Some((x, *y)) })
        })
        .collect()
}

fn create_board(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn find_symbols(board: &[Vec<char>]) -> HashSet<(i32, i32)> {
    board
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, ch)| {
                if NON_SYMBOLS.contains(*ch) {
                    None
                } else {
                    // We have to index the board by row then column,
                    // so we invert x and y.
                    Some((y as i32, x as i32))
                }
            })
        })
        .collect()
}

fn extract_numbers(line: &str) -> Vec<Match> {
    Regex::new(r"\d+").unwrap().find_iter(line).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    fn get_content() -> Vec<String> {
        vec![
            "467..114..".to_owned(),
            "...*......".to_owned(),
            "..35..633.".to_owned(),
            "......#...".to_owned(),
            "617*......".to_owned(),
            ".....+.58.".to_owned(),
            "..592.....".to_owned(),
            "......755.".to_owned(),
            "...$.*....".to_owned(),
            ".664.598..".to_owned(),
        ]
    }

    #[test]
    fn get_symbol_positions() {
        let board = super::create_board(&get_content());
        let expected = HashSet::from([(5, 5), (3, 8), (5, 8), (3, 1), (3, 4), (6, 3)]);
        assert_eq!(super::find_symbols(&board), expected);
    }

    #[test]
    fn get_edges_around_number() {
        let board = get_content();
        let first_numbers = super::extract_numbers(&board[0]);
        let expected_edge = HashSet::from([
            (2, 1),
            (1, 1),
            (2, 0),
            (1, 0),
            (0, 1),
            (3, 1),
            (0, 0),
            (3, 0),
        ]);
        assert_eq!(super::edge_of_number(0, first_numbers[0]), expected_edge);
    }

    #[test]
    fn total_engine_part_numbers() {
        let content = get_content();
        let part_number_sum = super::symbol_tracker(content)
            .values()
            .map(|numbers| numbers.iter().sum::<i32>())
            .sum::<i32>();
        assert_eq!(part_number_sum, 4361);
    }
}
