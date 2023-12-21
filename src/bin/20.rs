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
    // Tracks name of module and it's index bit position
    inputs: HashMap<String, usize>,
    // Tracks the pulse strengths of the inputs
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

struct Pulse<'a> {
    source: &'a str,
    destination: &'a str,
    strength: bool,
}

enum ReturnPart {
    PartOne(u32, u32),
    PartTwo(Option<String>),
}

fn parse_path(input: &str) -> IResult<&str, (&str, Path)> {
    let (input, (name, module)) = alt((
        preceded(complete::char('%'), alpha1).map(|name| (name, Module::FlipFlop(false))),
        preceded(complete::char('&'), alpha1).map(|name| {
            (
                name,
                Module::Conjunction(Conjunction {
                    inputs: HashMap::new(),
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
                module: Module::Conjunction(Conjunction { ref mut inputs, .. }),
                ..
            }) = module_paths.get_mut(d)
            {
                inputs.insert(String::from(*n), inputs.len());
            }
        }
    }

    Ok((input, module_paths))
}

fn press_button(module_paths: &mut HashMap<&str, Path>, part_one: bool) -> ReturnPart {
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
        // p1
        if strength {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }

        // If module exists then process it, otherwise ignore
        if let Some(Path {
            module,
            destinations,
        }) = module_paths.get_mut(destination)
        {
            // Determines whether to send pulse, and the pulse strength
            let processed_pulse: Option<bool> = match module {
                Module::FlipFlop(ref mut signal) => {
                    if !strength {
                        *signal = !*signal;
                        Some(*signal)
                    } else {
                        None
                    }
                }
                Module::Conjunction(Conjunction { inputs, state }) => {
                    // p2
                    if destination == "qt" && strength {
                        qt_mem = Some(String::from(source));
                    }

                    let n_shift = inputs.get(source).unwrap();
                    *state = (*state & !(1 << n_shift)) | ((strength as u32) << n_shift);
                    if *state == (1 << inputs.len()) - 1 {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
                Module::Broadcast => Some(strength),
            };

            // Adds next pulses to process
            if let Some(send_strength) = processed_pulse {
                for dst in destinations {
                    pulse_queue.push_back(Pulse {
                        source: destination,
                        destination: dst,
                        strength: send_strength,
                    })
                }
            }
        }
    }

    // idk, there's probablt a better way of doing this
    if part_one {
        ReturnPart::PartOne(low_pulses, high_pulses)
    } else {
        ReturnPart::PartTwo(qt_mem)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut module_paths) = parse_input(input).unwrap();

    let cycles = 1000;
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..cycles {
        if let ReturnPart::PartOne(low, high) = press_button(&mut module_paths, true) {
            low_pulses += low;
            high_pulses += high;
        }
    }

    Some(low_pulses * high_pulses)
}

// calculates greatest common divisor
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

// calculates least common multiple of the given numbers
fn lcm(values: Vec<usize>) -> usize {
    let mut a = values[0];
    for b in values.iter().skip(1) {
        a = a * b / gcd(a, *b)
    }
    a
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, mut module_paths) = parse_input(input).unwrap();
    // Starts out as hashmap of strings with value of 0
    let mut qt_inputs = module_paths
        .get("qt")
        .map(|Path { module, .. }| match module {
            Module::Conjunction(conjunction) => conjunction.inputs.clone(),
            _ => unreachable!(),
        })
        .unwrap();

    let mut presses = 0;
    // Keep pressing until all inouts of `qt` have gotten a high pulse
    while qt_inputs.values().any(|input_presses| *input_presses == 0) {
        presses += 1;
        if let ReturnPart::PartTwo(Some(input_name)) = press_button(&mut module_paths, false) {
            // Adding this condition doesn't make it flaky, but produces the wrong output
            // if *qt_inputs.get(&input_name).unwrap() == 0 {
            //     qt_inputs.insert(input_name, presses);
            // }

            // Without the condition above, it produces flaky output, but one of the outputs is correct. happened to be my second execution that gave the right answer
            qt_inputs.insert(input_name, presses);
        }
    }

    dbg!(&qt_inputs);

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
