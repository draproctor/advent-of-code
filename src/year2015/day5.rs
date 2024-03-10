use std::fs::read_to_string;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::solution;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BANNED_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

solution!(|path| {
    let content = read_to_string(path).unwrap();
    let count = content
        .lines()
        .filter(|line| is_nice_string(line.to_string()))
        .count();
    println!("There are {count} nice strings.");
});

fn is_nice_string(input: String) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let mut num_vowels = 0;
    let mut has_double_letter = false;

    for (i, char) in input.char_indices() {
        if VOWELS.contains(&char) {
            num_vowels += 1;
        }
        let next_index = input.len().min(i + 1);
        if next_index == input.len() {
            break;
        }
        let next_char = chars[next_index];
        if BANNED_SUBSTRINGS.contains(&format!("{}{}", chars[i], next_char).as_str()) {
            return false;
        }
        if char == next_char {
            has_double_letter = true;
        }
    }
    println!("Input: {input}, Num vowels: {num_vowels}, Has Double Letter: {has_double_letter}");
    num_vowels >= 3 && has_double_letter
}

fn is_nice_string_p2(input: &str) -> bool {
    let mut has_double_double_letters = false;
    let mut has_repeated_char = false;
    for (index, _) in input.char_indices() {
        if index == input.len() - 3 {
            break;
        }
        let window = &input[index..index + 4];
        println!("Input: {input}, window: {window:?}");
        let window = window.as_bytes();
        if window.len() < 4 {
            break;
        }
        if !has_double_double_letters
            && format!("{}{}", window[0], window[1]) == format!("{}{}", window[2], window[3])
        {
            has_double_double_letters = true;
        }
        has_repeated_char = window[0] == window[2] || window[1] == window[3];
    }

    has_double_double_letters && has_repeated_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(is_nice_string("ugknbfddgicrmopn".to_string()), true);
        assert_eq!(is_nice_string("aaa".to_string()), true);
        assert_eq!(is_nice_string("jchzalrnumimnmhp".to_string()), false);
        assert_eq!(is_nice_string("haegwjzuvuyypxyu".to_string()), false);
        assert_eq!(is_nice_string("dvszwmarrgswjxmb".to_string()), false);
    }

    #[test]
    fn examples_p2() {
        assert_eq!(is_nice_string_p2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_string_p2("xxyxx"), true);
        assert_eq!(is_nice_string_p2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_string_p2("ieodomkazucvgmuy"), false);
    }
}
