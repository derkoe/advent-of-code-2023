use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Debug)]
struct PartNumber {
    number: u32,
    line_no: usize,
    col_no: usize,
    col_end: usize,
}

struct Engine {
    part_numbers: Vec<PartNumber>,
    possible_gears: Vec<(usize, usize)>,
}

#[aoc_generator(day3, part1)]
#[aoc_generator(day3, part2)]
fn parse_input_part1(input: &str) -> Result<Engine, ParseIntError> {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let line_count = chars.len();
    let mut part_numbers = Vec::new();
    let mut possible_gears = Vec::new();

    for line_no in 0..line_count {
        let line = chars.get(line_no).unwrap();
        let line_length = chars.len();
        let mut col_no = 0;
        while col_no < line_length {
            if line[col_no].is_ascii_digit() {
                let (number, col_end) = read_number(&line, col_no);
                let is_part_number = check_symbols_around(&chars, line_no, col_no, col_end);
                if is_part_number {
                    part_numbers.push(PartNumber {
                        number,
                        line_no,
                        col_no,
                        col_end,
                    });
                }
                col_no = col_end;
            } else if line[col_no] == '*' {
                possible_gears.push((line_no, col_no));
            }
            col_no += 1;
        }
    }

    Ok(Engine {
        part_numbers,
        possible_gears,
    })
}

#[aoc(day3, part1)]
fn sum_part_numbers(games: &Engine) -> u32 {
    games.part_numbers.iter().map(|pn| pn.number).sum()
}

#[aoc(day3, part2)]
fn sum_gear_ratios(games: &Engine) -> u32 {
    games
        .possible_gears
        .iter()
        .map(|(gear_line_no, gear_col_no)| {
            let mut found_part_numbers = Vec::new();
            for pn in games.part_numbers.iter() {
                let begin_col = if pn.col_no > 0 {
                    pn.col_no - 1
                } else {
                    pn.col_no
                };

                if pn.line_no == *gear_line_no {
                    // gear at end of part number
                    if pn.col_end == *gear_col_no - 1 {
                        found_part_numbers.push(pn);
                    }
                    // gear at start of part number
                    if pn.col_no == *gear_col_no + 1 {
                        found_part_numbers.push(pn);
                    }
                }
                // gear over part number
                if pn.line_no == *gear_line_no - 1 {
                    for i in begin_col..pn.col_end + 2 {
                        if i == *gear_col_no {
                            found_part_numbers.push(pn);
                        }
                    }
                }
                // gear below part number
                if pn.line_no == *gear_line_no + 1 {
                    for i in begin_col..pn.col_end + 2 {
                        if i == *gear_col_no {
                            found_part_numbers.push(pn);
                        }
                    }
                }
            }
            if found_part_numbers.len() == 2 {
                found_part_numbers[0].number * found_part_numbers[1].number
            } else {
                0
            }
        })
        .sum()
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn check_symbols_around(
    chars: &Vec<Vec<char>>,
    line_no: usize,
    col_no: usize,
    col_end: usize,
) -> bool {
    let line_count = chars.len();
    let line_length = chars[0].len();
    let mut is_part_number = false;
    let end_col_check = if col_end + 2 < line_length {
        col_end + 2
    } else {
        col_end + 1
    };
    if line_no > 0 {
        if col_no > 0 && is_symbol(chars[line_no - 1][col_no - 1]) {
            is_part_number = true;
        }

        for i in col_no..end_col_check {
            if is_symbol(chars[line_no - 1][i]) {
                is_part_number = true;
            }
        }
    }
    if col_no > 0 && is_symbol(chars[line_no][col_no - 1]) {
        is_part_number = true;
    }
    if col_end + 1 < line_length && is_symbol(chars[line_no][col_end + 1]) {
        is_part_number = true;
    }
    if line_no + 1 < line_count {
        if col_no > 0 && is_symbol(chars[line_no + 1][col_no - 1]) {
            is_part_number = true;
        }
        for i in col_no..end_col_check {
            if is_symbol(chars[line_no + 1][i]) {
                is_part_number = true;
            }
        }
    }
    is_part_number
}

fn read_number(line: &Vec<char>, col_no: usize) -> (u32, usize) {
    let line_length = line.len();
    let mut max_col_no = col_no;

    if col_no + 1 < line_length && line[col_no + 1].is_ascii_digit() {
        max_col_no += 1;
        if col_no + 2 < line_length && line[col_no + 2].is_ascii_digit() {
            max_col_no += 1;
        }
    }

    let number = line[col_no..max_col_no + 1]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    (number, max_col_no)
}
