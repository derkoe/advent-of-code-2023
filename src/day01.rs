use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, num::ParseIntError};

#[aoc_generator(day1, part1)]
fn parse_input_part1(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    Ok(input
        .lines()
        .map(|line| {
            // find first and last digit in string line
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            digits
        })
        .collect())
}

#[aoc_generator(day1, part2)]
fn parse_input_part2(input: &str) -> Result<Vec<(u32, u32)>, ParseIntError> {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);
    Ok(input
        .lines()
        .map(|line| {
            let first_digit_pos: (&&str, u32) = digits
                .keys()
                .map(|key| (key, line.find(key)))
                .filter(|r| r.1.is_some())
                .map(|r| (r.0, r.1.unwrap() as u32))
                .reduce(|a, b| if a.1 < b.1 { a } else { b })
                .unwrap();

            // find last occurence
            let last_digit_pos: (&&str, u32) = digits
                .keys()
                .map(|key| (key, line.rfind(key)))
                .filter(|r| r.1.is_some())
                .map(|r| (r.0, r.1.unwrap() as u32))
                .reduce(|a, b| if a.1 > b.1 { a } else { b })
                .unwrap();

            (
                *digits.get(first_digit_pos.0).unwrap(),
                *digits.get(last_digit_pos.0).unwrap(),
            )
        })
        .collect())
}

#[aoc(day1, part1)]
fn sum(lines: &Vec<Vec<u32>>) -> u32 {
    let result = lines
        .iter()
        .map(|line| format!("{}{}", line.first().unwrap(), line.last().unwrap()))
        .map(|line| line.parse::<u32>().unwrap())
        .sum();
    result
}

#[aoc(day1, part2)]
fn sum2(lines: &Vec<(u32, u32)>) -> u32 {
    lines
        .iter()
        .map(|line| format!("{}{}", line.0, line.1).parse::<u32>().unwrap())
        .sum()
}
