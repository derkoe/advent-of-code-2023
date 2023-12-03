use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
#[aoc_generator(day3, part2)]
fn parse_input_part1(input: &str) -> Result<Vec<Vec<char>>, ParseIntError> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn check_symbols_around(chars: &Vec<Vec<char>>, line_no: usize, col_no: usize) -> bool {
    let line_count = chars.len();
    let line_length = chars[0].len();
    let mut is_part_number = false;
    if line_no > 0 {
        if is_symbol(chars[line_no - 1][col_no]) {
            is_part_number = true;
        }
        if col_no > 0 && is_symbol(chars[line_no - 1][col_no - 1]) {
            is_part_number = true;
        }
        if col_no + 1 < line_length && is_symbol(chars[line_no - 1][col_no + 1]) {
            is_part_number = true;
        }
    }
    if col_no > 0 && is_symbol(chars[line_no][col_no - 1]) {
        is_part_number = true;
    }
    if col_no + 1 < line_length && is_symbol(chars[line_no][col_no + 1]) {
        is_part_number = true;
    }
    if line_no + 1 < line_count {
        if is_symbol(chars[line_no + 1][col_no]) {
            is_part_number = true;
        }
        if col_no > 0 && is_symbol(chars[line_no + 1][col_no - 1]) {
            is_part_number = true;
        }
        if col_no + 1 < line_length && is_symbol(chars[line_no + 1][col_no + 1]) {
            is_part_number = true;
        }
    }
    is_part_number
}

#[aoc(day3, part1)]
fn part_number_sum(chars: &Vec<Vec<char>>) -> u32 {
    let line_count = chars.len();
    let line_length = chars[0].len();
    let mut sum = 0;
    let mut line_no = 0;
    while line_no < line_count {
        let mut col_no = 0;
        let mut is_part_number = false;
        while col_no < line_length {
            if chars[line_no][col_no].is_ascii_digit() {
                is_part_number |= check_symbols_around(chars, line_no, col_no);
                if col_no + 1 < line_length && chars[line_no][col_no + 1].is_ascii_digit() {
                    is_part_number |= check_symbols_around(chars, line_no, col_no + 1);
                    if col_no + 2 < line_length && chars[line_no][col_no + 2].is_ascii_digit() {
                        is_part_number |= check_symbols_around(chars, line_no, col_no + 2);
                        if is_part_number {
                            sum += chars[line_no][col_no..col_no + 3]
                                .iter()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap();
                        }
                        is_part_number = false;
                        col_no += 2;
                    } else {
                        if is_part_number {
                            sum += chars[line_no][col_no..col_no + 2]
                                .iter()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap();
                        }
                        is_part_number = false;
                        col_no += 1;
                    }
                } else {
                    if is_part_number {
                        sum += chars[line_no][col_no].to_digit(10).unwrap();
                    }
                    is_part_number = false;
                }
            }
            col_no += 1;
        }
        line_no += 1;
    }

    sum
}

fn read_number(chars: &Vec<Vec<char>>, line_no: usize, col_no: usize) -> u32 {
    let line_length = chars[0].len();
    let mut min_col_no = col_no;
    let mut max_col_no = col_no;
    if col_no - 1 > 0 && chars[line_no][col_no - 1].is_ascii_digit() {
        min_col_no -= 1;
        if col_no - 2 > 0 && chars[line_no][col_no - 2].is_ascii_digit() {
            min_col_no -= 1;
        }
    }

    if col_no + 1 < line_length && chars[line_no][col_no + 1].is_ascii_digit() {
        max_col_no += 1;
        if col_no + 2 < line_length && chars[line_no][col_no + 2].is_ascii_digit() {
            max_col_no += 1;
        }
    }

    chars[line_no][min_col_no..max_col_no + 1]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn find_numbers_around(chars: &Vec<Vec<char>>, line_no: usize, col_no: usize) -> Vec<u32> {
    let line_length = chars[0].len();
    let mut numbers = Vec::new();

    if line_no > 0 {
        // above
        if chars[line_no - 1][col_no].is_ascii_digit() {
            numbers.push(read_number(chars, line_no - 1, col_no));
        } else {
            if col_no > 0 && chars[line_no - 1][col_no - 1].is_ascii_digit() {
                numbers.push(read_number(chars, line_no - 1, col_no - 1));
            }
            if col_no + 1 < line_length && chars[line_no - 1][col_no + 1].is_ascii_digit() {
                numbers.push(read_number(chars, line_no - 1, col_no + 1));
            }
        }

        // below
        if chars[line_no + 1][col_no].is_ascii_digit() {
            numbers.push(read_number(chars, line_no + 1, col_no));
        } else {
            if col_no > 0 && chars[line_no + 1][col_no - 1].is_ascii_digit() {
                numbers.push(read_number(chars, line_no + 1, col_no - 1));
            }
            if col_no + 1 < line_length && chars[line_no + 1][col_no + 1].is_ascii_digit() {
                numbers.push(read_number(chars, line_no + 1, col_no + 1));
            }
        }

        // left
        if col_no > 0 && chars[line_no][col_no - 1].is_ascii_digit() {
            numbers.push(read_number(chars, line_no, col_no - 1));
        }

        // right
        if col_no + 1 < line_length && chars[line_no][col_no + 1].is_ascii_digit() {
            numbers.push(read_number(chars, line_no, col_no + 1));
        }
    }

    numbers
}

#[aoc(day3, part2)]
fn find_gear(chars: &Vec<Vec<char>>) -> u32 {
    let line_count = chars.len();
    let line_length = chars[0].len();
    let mut sum = 0;
    let mut line_no = 0;
    while line_no < line_count {
        let mut col_no = 0;
        while col_no < line_length {
            if chars[line_no][col_no] == '*' {
                let numbers = find_numbers_around(chars, line_no, col_no);
                if numbers.len() == 2 {
                    println!(
                        "{}:{} = {} * {}",
                        line_no + 1,
                        col_no + 1,
                        numbers[0],
                        numbers[1]
                    );
                    sum += numbers[0] * numbers[1];
                }
            }
            col_no += 1;
        }
        line_no += 1;
    }
    sum
}
