use std::fs::read_to_string;
use std::path::PathBuf;

use regex::{Match, Regex};

pub fn solve(path: PathBuf) {
    let content = read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();
    let scratch_cards = ScratchCard::from_lines(&content);
    println!(
        "Sum of scratch card scores: {}",
        scratch_cards.iter().map(ScratchCard::score).sum::<i32>(),
    );
    println!(
        "Sum of scratch cards generated: {}",
        multiply_cards(&scratch_cards)
    );
}

#[derive(Debug, PartialEq)]
struct ScratchCard {
    winning: Vec<i32>,
    drawn: Vec<i32>,
}

impl ScratchCard {
    fn score(&self) -> i32 {
        #[allow(clippy::cast_possible_truncation)]
        let winning_numbers_drawn = self.total_winning_numbers() as u32;
        if winning_numbers_drawn == 0 {
            return 0;
        }
        2i32.pow(winning_numbers_drawn - 1)
    }

    fn total_winning_numbers(&self) -> usize {
        self.drawn
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn from_lines(lines: &[String]) -> Vec<Self> {
        lines
            .iter()
            .map(|line| ScratchCard::from(line.as_str()))
            .collect()
    }
}

impl From<&str> for ScratchCard {
    /// Parse strings like this:
    /// ```
    /// "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    /// ```
    /// Winning numbers are on the left side of the `|`.
    /// Numbers on the scratch card are on the right side.
    fn from(line: &str) -> Self {
        let number_pattern = Regex::new(r"\d+").unwrap();
        let parser = |mat: Match| mat.as_str().parse::<i32>().unwrap();
        let winning_part = line
            .chars()
            .skip_while(|ch| *ch != ':')
            .take_while(|ch| *ch != '|')
            .collect::<String>();
        let winning = number_pattern
            .find_iter(&winning_part)
            .map(parser)
            .collect::<Vec<i32>>();
        let drawn_part = line.chars().skip_while(|ch| *ch != '|').collect::<String>();
        let drawn = number_pattern
            .find_iter(&drawn_part)
            .map(parser)
            .collect::<Vec<i32>>();
        Self { winning, drawn }
    }
}

fn multiply_cards(scratch_cards: &[ScratchCard]) -> usize {
    let mut card_counter = vec![1; scratch_cards.len()];
    let final_index = card_counter.len() - 1;
    for (index, card) in scratch_cards.iter().enumerate() {
        let total_wins = card.total_winning_numbers();
        if total_wins == 0 {
            continue;
        }
        let start = (index + 1).min(final_index);
        let end = (index + total_wins).min(final_index);
        for update_index in start..=end {
            card_counter[update_index] += card_counter[index];
        }
    }
    card_counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::ScratchCard;

    fn input_file() -> Vec<String> {
        [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(String::from)
        .into()
    }

    #[test]
    fn parsing() {
        let actual: Vec<ScratchCard> = ScratchCard::from_lines(&input_file());
        let expected = vec![
            ScratchCard {
                winning: vec![41, 48, 83, 86, 17],
                drawn: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            ScratchCard {
                winning: vec![13, 32, 20, 16, 61],
                drawn: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            ScratchCard {
                winning: vec![1, 21, 53, 59, 44],
                drawn: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            ScratchCard {
                winning: vec![41, 92, 73, 84, 69],
                drawn: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            ScratchCard {
                winning: vec![87, 83, 26, 28, 32],
                drawn: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            ScratchCard {
                winning: vec![31, 18, 13, 56, 72],
                drawn: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn scoring() {
        let scores: Vec<i32> = ScratchCard::from_lines(&input_file())
            .iter()
            .map(ScratchCard::score)
            .collect();
        assert_eq!(scores[0], 8);
        assert_eq!(scores[1], 2);
        assert_eq!(scores[2], 2);
        assert_eq!(scores[3], 1);
        assert_eq!(scores[4], 0);
        assert_eq!(scores[5], 0);
        assert_eq!(scores.iter().sum::<i32>(), 13);
    }

    #[test]
    fn counting() {
        let count: Vec<usize> = ScratchCard::from_lines(&input_file())
            .iter()
            .map(ScratchCard::total_winning_numbers)
            .collect();
        assert_eq!(count[0], 4);
        assert_eq!(count[1], 2);
        assert_eq!(count[2], 2);
        assert_eq!(count[3], 1);
        assert_eq!(count[4], 0);
        assert_eq!(count[5], 0);
    }

    #[test]
    fn card_totaling() {
        let cards = ScratchCard::from_lines(&input_file());
        let actual = super::multiply_cards(&cards);
        assert_eq!(actual, 30);
    }
}
