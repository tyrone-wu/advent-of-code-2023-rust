use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded},
    IResult, Parser,
};

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Conjunction {
    memory: HashMap<String, usize>,
    state: u32,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(Conjunction),
    Broadcast,
}

#[derive(Debug, Clone)]
struct Path<'a> {
    module: Module,
    destinations: Vec<&'a str>,
}

fn parse_path(input: &str) -> IResult<&str, (&str, Path)> {
    let (input, (name, module)) = alt((
        preceded(complete::char('%'), alpha1).map(|name| (name, Module::FlipFlop(false))),
        preceded(complete::char('&'), alpha1).map(|name| {
            (
                name,
                Module::Conjunction(Conjunction {
                    memory: HashMap::new(),
                    state: 0,
                }),
            )
        }),
        tag("broadcaster").map(|name| (name, Module::Broadcast)),
    ))(input)?;
    let (input, destinations) =
        delimited(tag(" -> "), separated_list1(tag(", "), alpha1), newline)(input)?;

    Ok((
        input,
        (
            name,
            Path {
                module,
                destinations,
            },
        ),
    ))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Path>> {
    let (input, mut module_paths) =
        fold_many1(parse_path, HashMap::new, |mut acc, (name, module)| {
            acc.insert(name, module);
            acc
        })(input)?;

    for (n, p) in module_paths.clone().iter() {
        for d in p.destinations.iter() {
            if let Some(Path {
                module: Module::Conjunction(Conjunction { ref mut memory, .. }),
                ..
            }) = module_paths.get_mut(d)
            {
                memory.insert(String::from(*n), memory.len());
            }
        }
    }

    Ok((input, module_paths))
}

struct Pulse<'a> {
    source: &'a str,
    destination: &'a str,
    strength: bool,
}

enum Return {
    PartOne(u32, u32),
    PartTwo(Option<String>),
}

fn press_button(module_paths: &mut HashMap<&str, Path>, part_one: bool) -> Return {
    // p1
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    // p2
    let mut qt_mem: Option<String> = None;

    let start = Pulse {
        source: "button",
        destination: "broadcaster",
        strength: false,
    };
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::from([start]);

    while let Some(Pulse {
        source,
        destination,
        strength,
    }) = pulse_queue.pop_front()
    {
        if strength {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }

        if let Some(Path {
            module,
            destinations,
        }) = module_paths.get_mut(destination)
        {
            let processed_pulse: Option<bool> = match module {
                Module::FlipFlop(ref mut signal) => {
                    if !strength {
                        *signal = !*signal;
                        Some(*signal)
                    } else {
                        None
                    }
                }
                Module::Conjunction(Conjunction { memory, state }) => {
                    if destination == "qt" && strength {
                        qt_mem = Some(String::from(source));
                    }

                    let n_shift = memory.get(source).unwrap();
                    *state = (*state & !(1 << n_shift)) | ((strength as u32) << n_shift);
                    if *state == (1 << memory.len()) - 1 {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
                Module::Broadcast => Some(strength),
            };

            if let Some(send_strength) = processed_pulse {
                destinations.iter().for_each(|dst| {
                    pulse_queue.push_back(Pulse {
                        source: destination,
                        destination: dst,
                        strength: send_strength,
                    })
                });
            }
        }
    }

    if part_one {
        Return::PartOne(low_pulses, high_pulses)
    } else {
        Return::PartTwo(qt_mem)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut module_paths) = parse_input(input).unwrap();

    let cycles = 1000;
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..cycles {
        if let Return::PartOne(low, high) = press_button(&mut module_paths, true) {
            low_pulses += low;
            high_pulses += high;
        }
    }

    Some(low_pulses * high_pulses)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn lcm(values: Vec<usize>) -> usize {
    let mut a = values[0];
    for b in values.iter().skip(1) {
        a = a * b / gcd(a, *b)
    }
    a
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, mut module_paths) = parse_input(input).unwrap();
    let mut qt_inputs = module_paths
        .get("qt")
        .map(|Path { module, .. }| match module {
            Module::Conjunction(conjunction) => conjunction.memory.clone(),
            _ => unreachable!(),
        })
        .unwrap();

    let mut presses = 0;
    while qt_inputs.values().any(|input_presses| *input_presses == 0) {
        presses += 1;
        if let Return::PartTwo(Some(input_name)) = press_button(&mut module_paths, false) {
            qt_inputs.insert(input_name, presses);
        }
    }

    // println!("{:?}", qt_inputs);

    Some(lcm(qt_inputs.values().cloned().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {}
}
