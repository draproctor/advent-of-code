use std::fs::File;
use std::path::PathBuf;

use crate::io::LineExtractor;
use crate::solution;

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

solution!(|path: PathBuf| {
    let id_total: i32 = File::open(path.clone())
        .expect("Cannot read the file")
        .lines()
        .map(|line| Game::from(line.unwrap()))
        .filter_map(|game| {
            if game.is_possible() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    println!("The total of the possible game IDs is {id_total}");

    let power_total: i32 = File::open(path)
        .expect("Cannot read the file")
        .lines()
        .map(|line| Game::from(line.unwrap()).power())
        .sum();
    println!("The total power of the games is {power_total}");
});

/// Represents a game an elf wants to play with us.
///
/// A game is played with a bag of colored cubes. In one game session, the elf will reach into the
/// bag an unknown number of times. Each time the elf grabs a random number of cubes.
///
/// A game is "possible" if the number of cubes for each color do not exceed a threshold.
/// E.g., if there is a limit of 10 red cubes and the elf draws 12 red cubes in one game session,
/// then we know it's "impossible" for there to be only 10 red cubes in the bag.
#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    cube_draws: Vec<CubeDraw>,
}

impl Game {
    /// Returns true if all of the cube draws made for a game are below a threshold,
    /// one threshold for each color of cube.
    fn is_possible(&self) -> bool {
        self.cube_draws.iter().all(|draw| {
            draw.red_cubes <= RED_LIMIT
                && draw.green_cubes <= GREEN_LIMIT
                && draw.blue_cubes <= BLUE_LIMIT
        })
    }

    /// Returns the power of a game.
    /// A power is calculated by finding the highest numbers of cubes drawn for a given color and
    /// multiplying the values for all cubes together.
    /// If one color is never drawn in a game, only the other maximum cube counts are considered.
    fn power(&self) -> i32 {
        let max_red_cubes = self
            .cube_draws
            .iter()
            .map(|draw| draw.red_cubes)
            .max()
            .unwrap_or(1);
        let max_green_cubes = self
            .cube_draws
            .iter()
            .map(|draw| draw.green_cubes)
            .max()
            .unwrap_or(1);
        let max_blue_cubes = self
            .cube_draws
            .iter()
            .map(|draw| draw.blue_cubes)
            .max()
            .unwrap_or(1);
        max_red_cubes * max_green_cubes * max_blue_cubes
    }
}

impl From<String> for Game {
    fn from(game_string: String) -> Game {
        Game {
            id: get_game_id(&game_string),
            cube_draws: get_cube_draws(&game_string),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CubeDraw {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

impl From<&str> for CubeDraw {
    fn from(game_string: &str) -> CubeDraw {
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        game_string.split(", ").for_each(|color_count| {
            let mut split = color_count.split_whitespace();
            let count = split.next().unwrap_or("0").parse::<i32>().unwrap();
            let color = split.next().unwrap_or("");
            match color {
                "red" => red_cubes += count,
                "green" => green_cubes += count,
                "blue" => blue_cubes += count,
                _ => (),
            };
        });

        CubeDraw {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }
}

/// Given a string slice, return the first number encountered.
fn get_game_id(line: &str) -> i32 {
    line.chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

/// Parse a line of text and return a vec of the `CubeDraws` in that line.
///
/// In this example, all text after the colon is the cube draws made by the elf.
/// ```
/// "Game 63: 4 red, 6 blue, 2 green; 3 green, 1 red, 5 blue; 7 blue, 5 green"
/// ```
fn get_cube_draws(line: &str) -> Vec<CubeDraw> {
    let cube_draws = line.split(':').last().unwrap_or("");
    if cube_draws.is_empty() {
        return vec![];
    }
    cube_draws.split(';').map(CubeDraw::from).collect()
}

#[cfg(test)]
mod tests {
    use super::{CubeDraw, Game};

    #[test]
    fn cube_draw_from_string() {
        assert_eq!(
            CubeDraw::from("2 red, 1 blue, 4 green"),
            CubeDraw {
                red_cubes: 2,
                blue_cubes: 1,
                green_cubes: 4
            }
        );
        assert_eq!(
            CubeDraw::from("6 blue, 2 green"),
            CubeDraw {
                red_cubes: 0,
                blue_cubes: 6,
                green_cubes: 2
            }
        );
        assert_eq!(
            CubeDraw::from("2 red, 5 green"),
            CubeDraw {
                red_cubes: 2,
                blue_cubes: 0,
                green_cubes: 5
            }
        );
    }

    #[test]
    fn get_game_id() {
        assert_eq!(
            super::get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            super::get_game_id("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            super::get_game_id("Game 21: 11 blue, 9 red; 8 red, 2 blue; 2 red, 11 blue, 2 green"),
            21
        );
        assert_eq!(
            super::get_game_id("Game 39: 10 blue, 4 green; 1 blue, 7 green, 5 red; 8 red, 2 blue",),
            39
        );
        assert_eq!(
            super::get_game_id(
                "Game 100: 10 blue, 2 red; 7 green, 20 blue, 9 red; 8 red, 6 green, 2 blue"
            ),
            100
        );
    }

    #[test]
    fn get_cube_draws() {
        assert!(super::get_cube_draws("").is_empty());
        assert_eq!(
            super::get_cube_draws(
                "4 green, 9 blue, 2 red; 2 blue, 8 green; 2 green, 2 red, 6 blue"
            ),
            vec![
                CubeDraw {
                    red_cubes: 2,
                    green_cubes: 4,
                    blue_cubes: 9
                },
                CubeDraw {
                    red_cubes: 0,
                    green_cubes: 8,
                    blue_cubes: 2,
                },
                CubeDraw {
                    red_cubes: 2,
                    green_cubes: 2,
                    blue_cubes: 6,
                }
            ]
        );
    }

    #[test]
    fn game_is_possible() {
        let possible_games = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 21: 11 blue, 9 red; 8 red, 2 blue; 2 red, 11 blue, 2 green",
            "Game 39: 10 blue, 4 green; 1 blue, 7 green, 5 red; 8 red, 2 blue",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            "Game 6: 12 red, 14 blue, 13 green; 2 blue, 1 red, 2 green",
        ];
        for input in possible_games {
            assert!(Game::from(input.to_string()).is_possible());
        }
        let impossible_games = [
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        ];
        for input in impossible_games {
            assert!(!Game::from(input.to_string()).is_possible());
        }
    }

    #[test]
    fn game_power() {
        for (input, expected_power) in &[
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ] {
            assert_eq!(
                Game::from((*input).to_string()).power(),
                expected_power.to_owned()
            );
        }
    }
}
