use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::Lines;

#[derive(Debug)]
struct RangeMap {
    source_start: u32,
    range_length: u32,
    destination_start: u32,
}
impl RangeMap {
    fn map_range(&self, source_range: (u32, u32)) -> (u32, u32) {
        if source_range.0 + source_range.1 < self.source_start
            || source_range.0 > self.source_start + self.range_length
        {
            source_range
        } else {
            let start = source_range.0.max(self.source_start);
            let end = (source_range.0 + source_range.1).min(self.source_start + self.range_length);
            let new_start: u32 = self.destination_start + (start - self.source_start);
            println!(
                "Mapping {:?} to {:?} with {:?}",
                source_range,
                (new_start, end - start),
                self
            );
            (new_start, end - start)
        }
    }
}

#[derive(Debug)]
struct PlantInfo {
    seeds: Vec<(u32, u32)>,
    maps: Vec<Vec<RangeMap>>,
}

#[aoc_generator(day5, part1)]
fn parse_part1(input: &str) -> PlantInfo {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    PlantInfo {
        seeds,
        maps: parse_maps(lines),
    }
}

#[aoc_generator(day5, part2)]
fn parse_part2(input: &str) -> PlantInfo {
    let mut lines = input.lines();
    let seeds: Vec<(u32, u32)> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .tuples()
        .collect();

    PlantInfo {
        seeds,
        maps: parse_maps(lines),
    }
}

fn parse_maps(lines: Lines<'_>) -> Vec<Vec<RangeMap>> {
    let mut maps = Vec::new();
    lines.skip(1).for_each(|line| {
        if line.is_empty() {
            return;
        }
        if line.ends_with("map:") {
            maps.push(Vec::new());
            return;
        }
        maps.last_mut().unwrap().push(RangeMap {
            destination_start: line.split_whitespace().nth(0).unwrap().parse().unwrap(),
            source_start: line.split_whitespace().nth(1).unwrap().parse().unwrap(),
            range_length: line.split_whitespace().nth(2).unwrap().parse().unwrap(),
        });
    });
    maps
}

#[aoc(day5, part1)]
#[aoc(day5, part2)]
fn calc(plant_info: &PlantInfo) -> u32 {
    println!("{:?}", plant_info.seeds);
    plant_info
        .seeds
        .iter()
        .map(|seed| {
            plant_info.maps.iter().fold(*seed, |acc, map| {
                map.iter()
                    .fold(acc, |inner_acc, mapping| mapping.map_range(inner_acc))
            })
        })
        .map(|i| i.0)
        .min()
        .unwrap()
}
