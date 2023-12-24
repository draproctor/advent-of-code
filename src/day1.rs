use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

const NUMBERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

trait LineExtractor {
    fn lines(&self) -> Lines<BufReader<&File>>;
}

impl LineExtractor for File {
    /// Return an iterator for reading the lines of a given file.
    fn lines(&self) -> Lines<BufReader<&File>> {
        BufReader::new(self).lines()
    }
}

pub fn solve(file_path: PathBuf) {
    let total: i32 = File::open(file_path)
        .expect("Cannot read the file")
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .sum();
    println!("The total is {}", total);
}

fn parse_line(line: String) -> i32 {
    let numbers = extract_numbers(&line);
    let first_number = numbers.first().unwrap();
    let second_number = numbers.last().unwrap();
    return format!("{}{}", first_number, second_number)
        .parse::<i32>()
        .unwrap();
}

/// Given a string, extract all number literals and written out numbers.
fn extract_numbers(line: &str) -> Vec<String> {
    line.char_indices()
        .filter_map(|(index, c)| extract_number_from_index(index, &c, &line))
        .collect()
}

fn extract_number_from_index(index: usize, c: &char, line: &str) -> Option<String> {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(c.to_string()),
        'o' | 't' | 'f' | 's' | 'e' | 'n' => extract_number_from_word(line, index),
        _ => None,
    }
}

fn extract_number_from_word(line: &str, index: usize) -> Option<String> {
    let last_index = line.len().min(index + 5);
    match line.get(index..last_index) {
        Some(slice) => NUMBERS.iter().find_map(|(number, value)| {
            if slice.starts_with(number) {
                return Some(value.to_string());
            }
            return None;
        }),
        None => None,
    }
}
