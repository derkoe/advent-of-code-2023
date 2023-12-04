use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

#[derive(Debug)]
struct Card {
    number: u32,
    winning: HashSet<u32>,
    mine: HashSet<u32>,
}

// parses cards in the format "Card   1: 72 28 41 15 98 13 27 99 93 38 | 62  5 80 81 53 29 23 25 59 72 90 19 54 86 68 73 55 21 56 27 32 15 12 42 44"
// the part after the pipe is the mine, the part before is the winning
#[aoc_generator(day4, part1)]
#[aoc_generator(day4, part2)]
fn parse_cards(input: &str) -> Result<Vec<Card>, ParseIntError> {
    let mut cards = Vec::new();
    for line in input.lines() {
        let mut card = line.split(":");
        let number = card.next().unwrap().replace("Card", "").trim().parse()?;
        let mut parts = card.next().unwrap().split(" | ");
        let winning = parse_set(parts.next().unwrap())?;
        let mine = parse_set(parts.next().unwrap())?;
        cards.push(Card {
            number,
            winning,
            mine,
        });
    }
    Ok(cards)
}

fn parse_set(input: &str) -> Result<HashSet<u32>, ParseIntError> {
    let mut set = HashSet::new();
    for number in input.split_whitespace() {
        set.insert(number.trim().parse()?);
    }
    Ok(set)
}

#[aoc(day4, part1)]
fn sum_part_numbers(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            let winning_count = card.winning.intersection(&card.mine).count();
            if winning_count <= 0 {
                0
            } else {
                2u32.pow(winning_count as u32 - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn cound_cards(cards: &Vec<Card>) -> u32 {
    let mut card_count: HashMap<u32, u32> = cards.iter().map(|card| (card.number, 1)).collect();

    cards.iter().for_each(|card| {
        let winning_count = card.winning.intersection(&card.mine).count();
        for num in card.number + 1..card.number + winning_count as u32 + 1 {
            *card_count.get_mut(&num).unwrap() += card_count[&card.number];
        }
    });
    card_count.iter().map(|e| e.1).fold(0, |a, b| a + b)
}
