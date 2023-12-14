use std::collections::HashMap;

use nom::{
    bytes::complete::take_till1, character::complete::newline, multi::separated_list1, IResult,
};

advent_of_code::solution!(14);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Space {
    RoundRock,
    CubeRock,
    EmptySpace,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Space>>> {
    let (input, map) = separated_list1(newline, take_till1(|c| c == '\n'))(input)?;
    Ok((
        input,
        map.iter()
            .map(|row| {
                row.as_bytes()
                    .iter()
                    .map(|c| match c {
                        b'O' => Space::RoundRock,
                        b'#' => Space::CubeRock,
                        b'.' => Space::EmptySpace,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    ))
}

fn tilt_north(map: &mut Vec<Vec<Space>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != Space::RoundRock {
                continue;
            }

            let mut row_idx = i;
            while row_idx > 0 && map[row_idx - 1][j] == Space::EmptySpace {
                map[row_idx][j] = Space::EmptySpace;
                row_idx -= 1;
                map[row_idx][j] = Space::RoundRock;
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<Space>>) {
    for row in map.iter_mut() {
        for j in 0..row.len() {
            if row[j] != Space::RoundRock {
                continue;
            }

            let mut col_idx = j;
            while col_idx > 0 && row[col_idx - 1] == Space::EmptySpace {
                row[col_idx] = Space::EmptySpace;
                col_idx -= 1;
                row[col_idx] = Space::RoundRock;
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<Space>>) {
    for i in (0..map.len()).rev() {
        for j in 0..map[0].len() {
            if map[i][j] != Space::RoundRock {
                continue;
            }

            let mut row_idx = i;
            while row_idx < map.len() - 1 && map[row_idx + 1][j] == Space::EmptySpace {
                map[row_idx][j] = Space::EmptySpace;
                row_idx += 1;
                map[row_idx][j] = Space::RoundRock;
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<Space>>) {
    for row in map.iter_mut() {
        for j in (0..row.len()).rev() {
            if row[j] != Space::RoundRock {
                continue;
            }

            let mut col_idx = j;
            while col_idx < row.len() - 1 && row[col_idx + 1] == Space::EmptySpace {
                row[col_idx] = Space::EmptySpace;
                col_idx += 1;
                row[col_idx] = Space::RoundRock;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut map) = parse_input(input).unwrap();

    tilt_north(&mut map);

    Some(
        map.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter(|space| space == &&Space::RoundRock)
                    .count()
                    * (map.len() - i)
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut map) = parse_input(input).unwrap();

    let mut seen: HashMap<Vec<Vec<Space>>, u32> = HashMap::new();

    let cycle_limit = 1000000000;
    let mut i = 0;

    while i < cycle_limit {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);

        i += 1;

        if let Some(seen_len) = seen.get(&map) {
            let cycle_len = i - seen_len;
            let remaining = cycle_limit - i;
            i += (remaining / cycle_len) * cycle_len;
        } else {
            seen.insert(map.clone(), i);
        }
    }

    Some(
        map.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter(|space| space == &&Space::RoundRock)
                    .count()
                    * (map.len() - i)
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
