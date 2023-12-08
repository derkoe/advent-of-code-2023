use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Game {
    hand: String,
    bid: u32,
}

#[aoc_generator(day7, part1)]
#[aoc_generator(day7, part2)]
fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap().to_string();
            let bid = parts.next().unwrap().parse().unwrap();
            Game { hand, bid }
        })
        .collect()
}

#[aoc(day7, part1)]
fn p1(games: &Vec<Game>) -> u32 {
    let mut sorted_games = games.to_vec();
    sorted_games.sort_by(compare_hands_p1);
    sorted_games
        .iter()
        .enumerate()
        .map(|(i, g)| (i as u32 + 1) * g.bid)
        .sum()
}

#[aoc(day7, part2)]
fn p2(games: &Vec<Game>) -> u32 {
    let mut sorted_games = games.to_vec();
    sorted_games.sort_by(compare_hands_p2);
    sorted_games
        .iter()
        .enumerate()
        .map(|(i, g)| (i as u32 + 1) * g.bid)
        .sum()
}

fn map_card_value_p1(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}
fn map_card_value_p2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn compare_hands_p1(a: &Game, b: &Game) -> std::cmp::Ordering {
    let a_type = hand_type(&a.hand);
    let b_type = hand_type(&b.hand);

    let comp = a_type.cmp(&b_type);
    if comp == std::cmp::Ordering::Equal {
        let a_val = a.hand.chars().map(map_card_value_p1);
        let b_val = b.hand.chars().map(map_card_value_p1);
        a_val.cmp(b_val)
    } else {
        comp
    }
}

fn compare_hands_p2(a: &Game, b: &Game) -> std::cmp::Ordering {
    let a_type = hand_type_p2(&a.hand);
    let b_type = hand_type_p2(&b.hand);

    let comp = a_type.cmp(&b_type);
    if comp == std::cmp::Ordering::Equal {
        let a_val = a.hand.chars().map(map_card_value_p2);
        let b_val = b.hand.chars().map(map_card_value_p2);
        a_val.cmp(b_val)
    } else {
        comp
    }
}

fn hand_type_p2(hand: &String) -> u32 {
    let hand_type_no_jokers = hand_type(hand);
    if hand.contains("J") {
        let hand_type_with_joker = hand
            .chars()
            .unique()
            .filter(|c| *c != 'J')
            .map(|c| hand.replace('J', &c.to_string()))
            .map(|h| hand_type(&h))
            .chain(Some(hand_type_no_jokers))
            .max()
            .unwrap();
        hand_type_with_joker
    } else {
        hand_type_no_jokers
    }
}

fn hand_type(hand: &String) -> u32 {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    for c in hand.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let pairs = counts.values().filter(|&&x| x == 2).count();
    let max_count = counts.values().max().unwrap_or(&0);

    match (*max_count, pairs) {
        (5, _) => 7, // Five of a kind
        (4, _) => 6, // Four of a kind
        (3, 1) => 5, // Full house
        (3, _) => 4, // Three of a kind
        (2, 2) => 3, // Two pair
        (2, _) => 2, // One pair
        _ => 1,      // High card
    }
}
