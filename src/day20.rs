use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum PulseType {
    High,
    Low,
}
#[derive(PartialEq, Debug, Clone)]
struct Pulse {
    pulse_type: PulseType,
    from: String,
    to: String,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    id: String,
    outputs: Vec<String>,
    flip_flop_state: bool,
    conjunction_state: HashMap<String, PulseType>,
}

#[aoc_generator(day20, part1)]
#[aoc_generator(day20, part2)]
fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let source = parts.next().unwrap();
        let outputs = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let module_type = match source.chars().next().unwrap() {
            '&' => ModuleType::Conjunction,
            '%' => ModuleType::FlipFlop,
            _ => ModuleType::Broadcaster,
        };
        let id = source.replace("%", "").replace("&", "");
        modules.insert(
            id.clone(),
            Module {
                module_type,
                id,
                outputs,
                flip_flop_state: false,
                conjunction_state: HashMap::new(),
            },
        );
    }
    modules
}

fn run_pulses(input_modules: &HashMap<String, Module>, max_iter: usize) -> usize {
    let mut modules = input_modules.clone();
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for button_press in 0..max_iter {
        let mut current_pulses = vec![Pulse {
            pulse_type: PulseType::Low,
            from: "button".to_string(),
            to: "broadcaster".to_string(),
        }];

        while current_pulses.len() > 0 {
            // println!("Current pulses: {:?}", current_pulses);
            // let last = current_pulses.last().unwrap();
            // println!("{} -{:?}-> {}", last.from, last.pulse_type, last.to);

            let pulse = current_pulses.pop().unwrap();
            match pulse.pulse_type {
                PulseType::Low => low_pulse_count += 1,
                PulseType::High => high_pulse_count += 1,
            }

            let module_opt = modules.get(pulse.to.as_str());
            if module_opt.is_none() {
                continue;
            }
            let mut module = module_opt.unwrap().clone();
            match module.module_type {
                ModuleType::Broadcaster => {
                    module.outputs.iter().for_each(|output| {
                        current_pulses.insert(
                            0,
                            Pulse {
                                pulse_type: pulse.pulse_type.clone(),
                                from: module.id.clone(),
                                to: output.clone(),
                            },
                        );
                    });
                }
                ModuleType::FlipFlop => {
                    if PulseType::Low == pulse.pulse_type {
                        module.flip_flop_state = !module.flip_flop_state;
                        modules.insert(module.id.clone(), module.clone());
                        let next_pulse_type = if module.flip_flop_state {
                            PulseType::High
                        } else {
                            PulseType::Low
                        };
                        module.outputs.iter().for_each(|output| {
                            current_pulses.insert(
                                0,
                                Pulse {
                                    pulse_type: next_pulse_type.clone(),
                                    from: module.id.clone(),
                                    to: output.clone(),
                                },
                            );
                        });
                    }
                }
                ModuleType::Conjunction => {
                    module
                        .conjunction_state
                        .insert(pulse.from, pulse.pulse_type);
                    modules.insert(module.id.clone(), module.clone());

                    // if all in conjunction_state are High, then output High
                    let next_pulse_type = if modules
                        .values()
                        .filter(|m| m.outputs.contains(&module.id))
                        .map(|m| {
                            module
                                .conjunction_state
                                .get(&m.id)
                                .unwrap_or_else(|| &PulseType::Low)
                        })
                        .all(|pulse_type| *pulse_type == PulseType::High)
                    {
                        PulseType::Low
                    } else {
                        if pulse.to == "sv"
                            || pulse.to == "ng"
                            || pulse.to == "ft"
                            || pulse.to == "jz"
                        {
                            println!("High {} at button press {}", pulse.to, button_press);
                        }
                        PulseType::High
                    };

                    module.outputs.iter().for_each(|output| {
                        current_pulses.insert(
                            0,
                            Pulse {
                                pulse_type: next_pulse_type.clone(),
                                from: module.id.clone(),
                                to: output.clone(),
                            },
                        );
                    });
                }
            }
        }
    }

    low_pulse_count * high_pulse_count
}

#[aoc(day20, part1)]
fn part1(modules: &HashMap<String, Module>) -> usize {
    run_pulses(modules, 1000)
}

#[aoc(day20, part2)]
fn part2(modules: &HashMap<String, Module>) -> usize {
    run_pulses(modules, 10000);
    // look at the printed numbers, get the intervals and multitpy them together, result is:
    224602011344203
}
