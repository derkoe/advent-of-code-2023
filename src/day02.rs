use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    number: u32,
    draws: Vec<Draw>,
}

#[aoc_generator(day2, part1)]
#[aoc_generator(day2, part2)]
fn parse_input_part1(input: &str) -> Result<Vec<Game>, ParseIntError> {
    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();
    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();

    Ok(input
        .lines()
        .map(|line| {
            let draws = line[line.find(":").unwrap()..]
                .split(";")
                .map(|draw_str| Draw {
                    red: red_re.captures(draw_str).map_or(0, |matches| {
                        matches.get(1).map_or(0, |m| m.as_str().parse().unwrap())
                    }),
                    green: green_re.captures(draw_str).map_or(0, |matches| {
                        matches.get(1).map_or(0, |m| m.as_str().parse().unwrap())
                    }),
                    blue: blue_re.captures(draw_str).map_or(0, |matches| {
                        matches.get(1).map_or(0, |m| m.as_str().parse().unwrap())
                    }),
                })
                .collect::<Vec<Draw>>();
            Game {
                number: line[5..line.find(":").unwrap()].parse().unwrap(),
                draws,
            }
        })
        .collect())
}

#[aoc(day2, part1)]
fn sum(games: &Vec<Game>) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= max_red && draw.green <= max_green && draw.blue <= max_blue)
        })
        .map(|game| game.number)
        .sum()
}

#[aoc(day2, part2)]
fn sum2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            game.draws.iter().for_each(|draw| {
                min_red = std::cmp::max(min_red, draw.red);
                min_green = std::cmp::max(min_green, draw.green);
                min_blue = std::cmp::max(min_blue, draw.blue);
            });
            min_red * min_green * min_blue
        })
        .sum()
}
