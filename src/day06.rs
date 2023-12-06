use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6, part1)]
fn input_generator_part1(_input: &str) -> Vec<Race> {
    vec![
        Race {
            time: 56,
            distance: 546,
        },
        Race {
            time: 97,
            distance: 1927,
        },
        Race {
            time: 78,
            distance: 1131,
        },
        Race {
            time: 75,
            distance: 1139,
        },
    ]
}
#[aoc_generator(day6, part2)]
fn input_generator_part2(_input: &str) -> Vec<Race> {
    vec![Race {
        time: 56977875,
        distance: 546192711311139,
    }]
}

#[aoc(day6, part1)]
#[aoc(day6, part2)]
fn calc(races: &Vec<Race>) -> u32 {
    // multiply all time * distance
    races
        .iter()
        .map(|r| {
            let mut won_races = 0;
            for speed in 1..r.time {
                let distance = speed * (r.time - speed);
                if distance > r.distance {
                    won_races += 1;
                }
            }
            won_races
        })
        .fold(1, |a, b| a * b)
}
