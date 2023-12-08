use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(8);

struct Branch {
    start: (u8, u8, u8),
    left: (u8, u8, u8),
    right: (u8, u8, u8),
}

fn parse_branch(input: &str) -> IResult<&str, Branch> {
    let (input, (start, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")\n"),
        ),
    )(input)?;

    let start_arr = start.as_bytes();
    let left_arr = left.as_bytes();
    let right_arr = right.as_bytes();

    Ok((
        input,
        Branch {
            start: (start_arr[0], start_arr[1], start_arr[2]),
            left: (left_arr[0], left_arr[1], left_arr[2]),
            right: (right_arr[0], right_arr[1], right_arr[2]),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, (&[u8], HashMap<(u8, u8, u8), Branch>)> {
    let (input, path_str) = terminated(alpha1, tag("\n\n"))(input)?;
    let (input, branches) = fold_many1(parse_branch, HashMap::new, |mut acc, b| {
        acc.insert(b.start.clone(), b);
        acc
    })(input)?;

    Ok((input, (path_str.as_bytes(), branches)))
}
pub fn part_one(input: &str) -> Option<u32> {
    let (_, (path, map)) = parse_input(input).unwrap();

    let mut steps = 0;
    let mut position = (b'A', b'A', b'A');

    while position != (b'Z', b'Z', b'Z') {
        position = if path[steps % path.len()] == b'L' {
            map.get(&position).unwrap().left
        } else {
            map.get(&position).unwrap().right
        };

        steps += 1;
    }

    Some(steps as u32)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (path, map)) = parse_input(input).unwrap();

    let mut position_cycles: Vec<(&(u8, u8, u8), Vec<(&(u8, u8, u8), usize)>)> = map
        .iter()
        .filter(|((_, _, s), _)| s == &b'A')
        .map(|(s, _)| (s, Vec::new()))
        .collect();

    for (p, z_cycles) in position_cycles.iter_mut() {
        let mut steps = 0;
        let mut z_checkpoint = 0;

        while z_cycles.len() <= 1 || p != &z_cycles[0].0 {
            *p = if path[steps % path.len()] == b'L' {
                &map.get(p).unwrap().left
            } else {
                &map.get(p).unwrap().right
            };
            steps += 1;

            if p.2 == b'Z' {
                z_cycles.push((p, steps - z_checkpoint));
                z_checkpoint = steps;
            }
        }
    }

    // dbg!(position_cycles);

    let mut step_cycles: Vec<u64> = position_cycles
        .iter()
        .map(|(_, z_cycles)| z_cycles[0].1 as u64)
        .collect();
    let z_lcm = step_cycles.pop().unwrap();

    Some(step_cycles.iter().fold(z_lcm, |acc, c| lcm(acc, *c)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
