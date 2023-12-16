use std::{fs, process::exit};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    green: u32,
    red: u32,
    blue: u32,
    rounds: u32,
    id: u32,
}

impl Game {
    fn new(input: &str) -> Game {
        // "Game 1: 3 blue, 2 green, 6 red"
        let game_and_rounds: Vec<&str> = input.split(':').collect();
        debug_assert_eq!(game_and_rounds.len(), 2);
        let game_id: u32 = game_and_rounds
            .first()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        println!("game_id: {game_id}");
        println!("game_and_rounds: {:?}", game_and_rounds);

        let rounds = &game_and_rounds[1].split(';').collect::<Vec<&str>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in rounds.iter() {
            let coloured_cubes = round.split(',').collect::<Vec<&str>>();
            for cube in coloured_cubes {
                let name_and_count = cube.trim_start().split(' ').collect::<Vec<&str>>();
                println!("name_and_count {:?}", name_and_count);
                debug_assert_eq!(name_and_count.len(), 2);

                match name_and_count[1] {
                    "red" => {
                        let count = name_and_count[0].parse::<u32>().unwrap();
                        if count.gt(&red) {
                            red = count
                        }
                    }
                    "green" => {
                        let count = name_and_count[0].parse::<u32>().unwrap();
                        if count.gt(&green) {
                            green = count
                        }
                    }
                    "blue" => {
                        let count = name_and_count[0].parse::<u32>().unwrap();
                        if count.gt(&blue) {
                            blue = count
                        }
                    }
                    _ => unreachable!("we shouldn't run out of colours here"),
                }
            }
        }

        Game {
            rounds: rounds.len() as u32,
            id: game_id,
            red,
            green,
            blue,
        }
    }
    fn is_possible_with_this_combination(&self, red: u32, green: u32, blue: u32) -> bool {
        red >= self.red && green >= self.green && blue >= self.blue
    }
}

fn part_1(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    lines.iter().fold(0, |mut acc, line| {
        if line.is_empty() {
            return acc;
        }
        let game = Game::new(line);
        if game.is_possible_with_this_combination(12, 13, 14) {
            acc += game.id
        }
        acc
    })
}

// fn part_2() {}

fn main() {
    let input = fs::read_to_string("../../../day2-input.txt").unwrap();

    println!("{}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new() {
        let test = Game::new("Game 28: 4 blue, 7 green, 4 red; 2 red, 4 blue, 7 green; 6 blue, 11 green, 4 red; 6 blue, 6 green, 3 red; 6 green, 12 red");
        let example = Game {
            green: 11,
            red: 12,
            blue: 6,
            rounds: 5,
            id: 28,
        };

        assert_eq!(test, example);

        let test_2 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let example_2 = Game {
            green: 2,
            red: 4,
            blue: 6,
            rounds: 3,
            id: 1,
        };

        assert_eq!(test_2, example_2)
    }

    #[test]
    fn test_game_is_possible_with_this_combination() {
        let example = Game {
            red: 12,
            green: 11,
            blue: 6,
            rounds: 5,
            id: 28,
        };

        let example_2 = Game {
            red: 4,
            green: 2,
            blue: 6,
            rounds: 3,
            id: 1,
        };
        assert!(!example.is_possible_with_this_combination(5, 7, 7));
        assert!(example.is_possible_with_this_combination(12, 11, 6));
        assert!(!example.is_possible_with_this_combination(11, 12, 4));

        assert!(example_2.is_possible_with_this_combination(5, 7, 7));
        assert!(!example_2.is_possible_with_this_combination(1, 1, 0));
        assert!(example_2.is_possible_with_this_combination(5, 7, 7));
    }

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_1(input), 8)
    }
}
