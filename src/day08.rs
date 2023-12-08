use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Instructions {
    directions: String,
    nodes: HashMap<String, (String, String)>,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[aoc_generator(day8, part1)]
#[aoc_generator(day8, part2)]
fn parse_input(input: &str) -> Instructions {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().to_string();
    let nodes = lines
        .skip(1)
        .map(|line| {
            let from = line.split(" = ").nth(0).unwrap().to_string();
            let to_string = line
                .split(" = ")
                .nth(1)
                .unwrap()
                .replace("(", "")
                .replace(")", "");
            let left: &str = to_string.split(", ").nth(0).unwrap();
            let right: &str = to_string.split(", ").nth(1).unwrap();
            (from, (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<String, (String, String)>>();

    Instructions { directions, nodes }
}

#[aoc(day8, part1)]
fn p1(instructions: &Instructions) -> usize {
    find_path(instructions, "AAA", "ZZZ")
}

fn find_path(instructions: &Instructions, start: &str, end: &str) -> usize {
    let mut current = start;
    let mut count = 0;
    let mut reached_zzz = false;

    while !reached_zzz {
        instructions.directions.chars().for_each(|c| {
            let (left, right) = instructions.nodes.get(current).unwrap();
            count += 1;
            match c {
                'L' => current = left,
                'R' => current = right,
                _ => panic!("Invalid direction"),
            }
            if current.ends_with(end) {
                reached_zzz = true;
                return;
            }
        });
    }

    count
}

#[aoc(day8, part2)]
fn p2(instructions: &Instructions) -> usize {
    instructions
        .nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| find_path(instructions, &k, "Z"))
        .fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let instructions = parse_input(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(instructions.directions, "RL");
        assert_eq!(instructions.nodes.len(), 7);
    }
}
