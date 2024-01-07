// I got super stuck, so I went to reddit and found https://github.com/rvodden/AoC23/blob/main/day-05/src/part1.rs.
// rvodden is very clever and helped me a lot. Now I know `nom` is the perfect library for AoC!
use std::{fs::read_to_string, ops::Range, path::PathBuf};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn solve(path: PathBuf) {
    let content = read_to_string(path).expect("Should read file");
    let (_, (seeds, almanac_ranges)) = parse_file(&content).expect("Should Parse");
    let min_location = seeds
        .into_iter()
        .map(|seed_num| follow_map(seed_num, "seed", &almanac_ranges))
        .min()
        .unwrap();
    println!("Minimum location: {min_location}");
}

fn parse_file(content: &str) -> IResult<&str, (Vec<u64>, Vec<AlmanacRange>)> {
    separated_pair(
        seeds,
        pair(newline, newline),
        separated_list1(newline, AlmanacRange::complete_mapping),
    )(content)
}

fn follow_map(value: u64, destination: &str, almanac_ranges: &[AlmanacRange]) -> u64 {
    print!("{destination} {value:10} ");
    if let Some(almanac_range) = almanac_ranges.iter().find(|ar| ar.source == destination) {
        let value = almanac_range.resolve(value);
        follow_map(value, &almanac_range.destination, almanac_ranges)
    } else {
        println!();
        value
    }
}

fn seeds(line: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(space1, complete::u64))(line)
}

#[derive(Clone)]
struct AlmanacRange {
    source: String,
    destination: String,
    ranges: Vec<(Range<u64>, u64)>,
}

impl AlmanacRange {
    /// Given a source number, return the destination value.
    /// If the source number is in a mapped range, the mapped value is returned.
    /// Otherwise, the source number is returned as-is.
    fn resolve(&self, source: u64) -> u64 {
        for (range, destination_start) in &self.ranges {
            if range.contains(&source) {
                let source_start = range.clone().min().expect("Should have minimum");
                let offset_from_start = source - source_start;
                return destination_start + offset_from_start;
            }
        }
        source
    }

    /// Parses a complete mapping of numbers for values of the given types.
    /// E.g.,
    /// ```
    /// "seed-to-soil map:
    /// 50 98 2
    /// 52 50 48"
    /// ```
    fn complete_mapping(input: &str) -> IResult<&str, AlmanacRange> {
        let (input, ((source, destination), ranges)) =
            separated_pair(Self::header, newline, many1(Self::range_mapping))(input)?;

        Ok((
            input,
            AlmanacRange {
                source: String::from(source),
                destination: String::from(destination),
                ranges,
            },
        ))
    }

    /// Parses the header of an almanac range.
    /// E.g., `"seed-to-soil map:"`
    fn header(line: &str) -> IResult<&str, (&str, &str)> {
        terminated(
            separated_pair(alpha1, tag("-to-"), alpha1),
            pair(space1, tag("map:")),
        )(line)
    }

    /// Parses one line of a range map.
    /// E.g., `"1965922922 2387203602 59808406"`
    fn range_mapping(line: &str) -> IResult<&str, (Range<u64>, u64)> {
        let (line, (dest_start, source_start, length)): (&str, (u64, u64, u64)) = tuple((
            terminated(complete::u64, space1),
            terminated(complete::u64, space1),
            terminated(complete::u64, newline),
        ))(line)?;

        Ok((line, (source_start..(source_start + length), dest_start)))
    }
}
