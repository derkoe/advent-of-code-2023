use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Game {
    hand: String,
    bid: u32,
}

#[aoc_generator(day7, part1)]
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
fn calc(games: &Vec<Game>) -> u32 {
    let mut sorted_games = games.to_vec();
    sorted_games.sort_by(compare_hands);
    sorted_games
        .iter()
        .enumerate()
        .map(|(i, g)| {
            // println!("{}", g.hand);
            (i as u32 + 1) * g.bid
        })
        .sum()
}

fn map_card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn compare_hands(a: &Game, b: &Game) -> std::cmp::Ordering {
    let a_type = hand_type(&a.hand);
    let b_type = hand_type(&b.hand);

    let comp = a_type.cmp(&b_type);
    if comp == std::cmp::Ordering::Equal {
        let a_val = a.hand.chars().map(map_card_value);
        let b_val = b.hand.chars().map(map_card_value);
        a_val.cmp(b_val)
    } else {
        comp
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
