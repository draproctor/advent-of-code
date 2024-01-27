use std::{fs::read_to_string, path::PathBuf};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, multispace1, newline},
    combinator::recognize,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::solution;

solution!(|path| {
    let content = read_to_string(path).expect("Should read file");
    println!(
        "Product of winning times: {}",
        parse_file(&content)
            .iter()
            .map(TimedRace::count_better_times)
            .product::<u64>(),
    );

    println!(
        "Number of winning times (part 2): {}",
        parse_file_p2(&content).count_better_times(),
    );
});

fn parse_file(content: &str) -> Vec<TimedRace> {
    let (_, (times, distances)) =
        separated_pair(times, newline, distances)(content).expect("Should be able to parse file");
    times
        .iter()
        .zip(&distances)
        .map(|(time, distance)| TimedRace::new(*time, *distance))
        .collect()
}

fn times(input: &str) -> IResult<&str, Vec<u64>> {
    let prefix = recognize(tuple((tag("Time:"), multispace1)));
    preceded(prefix, separated_list1(multispace1, complete::u64))(input)
}

fn distances(input: &str) -> IResult<&str, Vec<u64>> {
    let prefix = recognize(tuple((tag("Distance:"), multispace1)));
    preceded(prefix, separated_list1(multispace1, complete::u64))(input)
}

fn parse_file_p2(content: &str) -> TimedRace {
    let (_, (time, distance)) =
        separated_pair(time, newline, distance)(content).expect("Should parse file (pt 2)");
    TimedRace { time, distance }
}

fn time(input: &str) -> IResult<&str, u64> {
    let prefix = recognize(tuple((tag("Time:"), multispace1)));
    preceded(prefix, fold_number)(input)
}

fn distance(input: &str) -> IResult<&str, u64> {
    let prefix = recognize(tuple((tag("Distance:"), multispace1)));
    preceded(prefix, fold_number)(input)
}

fn fold_number(input: &str) -> IResult<&str, u64> {
    let (input, folded_num) = separated_list1(multispace1, digit1)(input)?;
    Ok((input, folded_num.join("").parse().expect("Parsed number")))
}

#[derive(Debug, PartialEq)]
struct TimedRace {
    time: u64,
    distance: u64,
}

impl TimedRace {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn count_better_times(&self) -> u64 {
        (1..self.time)
            .filter_map(|button_held_duraction| {
                let race_time_left = self.time - button_held_duraction;
                let distance = button_held_duraction * race_time_left;
                if distance > self.distance {
                    Some(distance)
                } else {
                    None
                }
            })
            .count()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to get race times for {:?}", &self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_file() -> String {
        let mut file = String::new();
        file.push_str("Time:      7  15   30\n");
        file.push_str("Distance:  9  40  200");
        file
    }

    #[test]
    fn parsing() {
        let actual = parse_file(&input_file());
        let expected = vec![
            TimedRace {
                time: 7,
                distance: 9,
            },
            TimedRace {
                time: 15,
                distance: 40,
            },
            TimedRace {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn counting_race_times() {
        assert_eq!(
            TimedRace {
                time: 7,
                distance: 9,
            }
            .count_better_times(),
            4
        );
        assert_eq!(
            TimedRace {
                time: 15,
                distance: 40,
            }
            .count_better_times(),
            8
        );
        assert_eq!(
            TimedRace {
                time: 30,
                distance: 200,
            }
            .count_better_times(),
            9
        );
    }
}
