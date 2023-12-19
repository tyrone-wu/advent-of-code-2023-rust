use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

advent_of_code::solution!(18);

struct Dig<'a> {
    direction: (i32, i32),
    count: i64,
    color: &'a str,
}

fn parse_dig(input: &str) -> IResult<&str, Dig> {
    let (input, direction) = alt((
        complete::char('U').map(|_| (0, 1)),
        complete::char('D').map(|_| (0, -1)),
        complete::char('L').map(|_| (-1, 0)),
        complete::char('R').map(|_| (1, 0)),
    ))(input)?;
    let (input, count) = preceded(tag(" "), complete::i64)(input)?;
    let (input, color) = delimited(tag(" (#"), alphanumeric1, tag(")"))(input)?;
    Ok((
        input,
        Dig {
            direction,
            count,
            color,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Dig>> {
    separated_list1(newline, parse_dig)(input)
}

fn solve(dig_plan: &[Dig<'_>]) -> u64 {
    let mut map: HashMap<(i32, i32), u8> = HashMap::new();
    let mut prev = (0, 0);
    map.insert(prev, 0);

    for Dig {
        direction,
        count,
        color: _,
    } in dig_plan
    {
        let (prev_dir, current_dir): (u8, u8) = match direction {
            (0, 1) => (1, 1 << 1),
            (0, -1) => (1 << 1, 1),
            (-1, 0) => (1 << 3, 1 << 2),
            (1, 0) => (1 << 2, 1 << 3),
            _ => unreachable!(),
        };

        for _ in 0..*count {
            *map.get_mut(&prev).unwrap() |= prev_dir;
            let coord = (prev.0 + direction.0, prev.1 + direction.1);
            map.insert(coord, current_dir);
            prev = coord;
        }
    }

    if let Some(up) = map.get_mut(&(0, 1)) {
        *up |= 1;
    }
    if let Some(down) = map.get_mut(&(0, -1)) {
        *down |= 1 << 1;
    }
    if let Some(left) = map.get_mut(&(-1, 0)) {
        *left |= 1 << 3;
    }
    if let Some(right) = map.get_mut(&(1, 0)) {
        *right |= 1 << 2;
    }

    let min_x = map.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = map.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;

    (min_y..=max_y)
        .map(|y| {
            let mut cross_flag: u8 = 0;
            let total_cross: u32 = (min_x..=max_x)
                .map(|x| {
                    if let Some(direction) = map.get(&(x, y)) {
                        cross_flag ^= ((1 << 2) - 1) & *direction;
                        if cross_flag == (1 << 1) | 1 {
                            cross_flag = 0;
                            return 1;
                        }
                    }
                    0
                })
                .sum();

            if total_cross % 2 == 1 {
                return 0;
            }

            let mut cross_count = 0;
            cross_flag = 0;
            (min_x..=max_x)
                .map(|x| {
                    if let Some(direction) = map.get(&(x, y)) {
                        cross_flag ^= ((1 << 2) - 1) & *direction;
                        if cross_flag == (1 << 1) | 1 {
                            cross_count += 1;
                            cross_flag = 0;
                        }
                        0
                    } else if cross_count % 2 == 1 {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum::<u64>()
        + map.len() as u64
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, dig_plan) = parse_input(input).unwrap();
    Some(solve(&dig_plan) as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, dig_plan) = parse_input(input).unwrap();

    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    coordinates.push((0, 0));
    for (i, Dig { color, .. }) in dig_plan.iter().enumerate() {
        let count = i64::from_str_radix(&color[..(color.len() - 1)], 16).unwrap();
        let direction = color.as_bytes().iter().last().unwrap();
        let prev = coordinates[i];
        let coord = match direction {
            b'0' => (prev.0 + count, prev.1),
            b'1' => (prev.0, prev.1 - count),
            b'2' => (prev.0 - count, prev.1),
            b'3' => (prev.0, prev.1 + count),
            _ => unreachable!(),
        };

        coordinates.push(coord);
    }

    // Some(solve(&dig_plan))

    let shoelace = (0..coordinates.len() - 1)
        .map(|i| {
            let a = coordinates[i];
            let b = coordinates[i + 1];

            (a.0 * b.1) - (a.1 * b.0)
        })
        .sum::<i64>()
        .abs()
        / 2;

    let perimeter = (0..coordinates.len() - 1)
        .map(|i| {
            let a = coordinates[i];
            let b = coordinates[i + 1];

            (a.0 - b.0).abs() + (a.1 - b.1).abs()
        })
        .sum::<i64>();

    Some(shoelace + perimeter / 2 + 1)

    // let picks = shoelace - perimeter / 2 + 1;
    // Some(picks + perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
