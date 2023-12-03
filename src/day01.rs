use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

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
fn parse_input_part2(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut result = Vec::new();
            let char_count = line.len();
            let mut i = 0;
            while i < char_count {
                let current_char = line.chars().nth(i).or(Some('X')).unwrap();
                if current_char.is_ascii_digit() {
                    result.push(current_char.to_digit(10).unwrap());
                    i += 1;
                } else {
                    let substr = &line[i..];
                    match substr {
                        x if x.starts_with("one") => {
                            i += 3;
                            result.push(1)
                        }
                        x if x.starts_with("two") => {
                            i += 3;
                            result.push(2)
                        }
                        x if x.starts_with("three") => {
                            i += 5;
                            result.push(3)
                        }
                        x if x.starts_with("four") => {
                            i += 4;
                            result.push(4)
                        }
                        x if x.starts_with("five") => {
                            i += 4;
                            result.push(5)
                        }
                        x if x.starts_with("six") => {
                            i += 3;
                            result.push(6)
                        }
                        x if x.starts_with("seven") => {
                            i += 5;
                            result.push(7)
                        }
                        x if x.starts_with("eight") => {
                            i += 5;
                            result.push(8)
                        }
                        x if x.starts_with("nine") => {
                            i += 4;
                            result.push(9)
                        }
                        _ => i += 1,
                    }
                }
            }
            Vec::from(result)
        })
        .collect())
}

#[aoc(day1, part1)]
#[aoc(day1, part2)]
fn sum(lines: &Vec<Vec<u32>>) -> u32 {
    let result = lines
        .iter()
        .map(|line| format!("{}{}", line.first().unwrap(), line.last().unwrap()))
        .map(|line| line.parse::<u32>().unwrap())
        .sum();
    result
}
