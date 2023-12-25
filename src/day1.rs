use std::fs::File;
use std::path::PathBuf;

use crate::io::LineExtractor;

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

pub fn solve(file_path: PathBuf) {
    let total: i32 = File::open(file_path)
        .expect("Cannot read the file")
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .sum();
    println!("The total is {}", total);
}

/// Given a line in a file, find the first and last numbers in the line. The digits are combined
/// to form a two digit number.
///
/// # Examples
/// ```
/// let line1 = String::from("trom7bone");
/// let line2 = String::from("asdf4jkdsqseven");
/// let line3 = String::from("asdasfourdsqwefiveasdastwobar");
/// assert_eq!(parse_line(line1), 77);
/// assert_eq!(parse_line(line2), 47);
/// assert_eq!(parse_line(line3), 42);
/// ```
fn parse_line(line: String) -> i32 {
    let numbers = extract_numbers(&line);
    let first_number = numbers.first().unwrap();
    let second_number = numbers.last().unwrap();
    format!("{}{}", first_number, second_number)
        .parse::<i32>()
        .unwrap()
}

/// Given a string, extract all number literals and written out numbers.
fn extract_numbers(line: &str) -> Vec<String> {
    line.char_indices()
        .filter_map(|(index, c)| find_number_string(index, c, &line))
        .collect()
}

/// Given an index in a string, the character at that index, and a string, determine if that
/// character is either a number or the beginning of a word of a number.
fn find_number_string(index: usize, c: char, line: &str) -> Option<String> {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(c.to_string()),
        'o' | 't' | 'f' | 's' | 'e' | 'n' => get_word_of_number(line, index),
        _ => None,
    }
}

/// Given a line and an index in that line, search for words that describe numbers,
/// such as `"one"`, `"eight"`, or `"nine"`.
///
/// These numbers are at most five characters long, so this function takes the index and looks five
/// characters further. If this string slice begins with any number-word, then the corresponding
/// number is returned as string.
fn get_word_of_number(line: &str, index: usize) -> Option<String> {
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
