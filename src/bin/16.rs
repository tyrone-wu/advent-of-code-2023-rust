use std::collections::VecDeque;

use nom::{
    bytes::complete::take_till1, character::complete::newline, multi::separated_list1, IResult,
};

advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum Space {
    EmptySpace,
    MirrorForwardSlash,
    MirrorBackwardSlash,
    SplitterPipe,
    SplitterDash,
}

#[derive(Clone, Copy)]
struct Tile {
    space: Space,
    directions: u8,
}

#[derive(Clone, Copy)]
struct LightBeam {
    direction: u8,
    x: usize,
    y: usize,
}

fn parse_tile(input: &str) -> IResult<&str, Vec<Tile>> {
    let (input, row) = take_till1(|c| c == '\n')(input)?;
    Ok((
        input,
        row.as_bytes()
            .iter()
            .map(|c| Tile {
                space: match c {
                    b'.' => Space::EmptySpace,
                    b'/' => Space::MirrorForwardSlash,
                    b'\\' => Space::MirrorBackwardSlash,
                    b'|' => Space::SplitterPipe,
                    b'-' => Space::SplitterDash,
                    _ => unreachable!(),
                },
                directions: 0,
            })
            .collect(),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_tile)(input)
}

fn move_north(x: usize, y: usize) -> Option<LightBeam> {
    if x == 0 {
        None
    } else {
        Some(LightBeam {
            direction: 0,
            x: x - 1,
            y,
        })
    }
}

fn move_south(x: usize, y: usize, rows: usize) -> Option<LightBeam> {
    if x == rows - 1 {
        None
    } else {
        Some(LightBeam {
            direction: 1,
            x: x + 1,
            y,
        })
    }
}

fn move_east(x: usize, y: usize, cols: usize) -> Option<LightBeam> {
    if y == cols - 1 {
        None
    } else {
        Some(LightBeam {
            direction: 2,
            x,
            y: y + 1,
        })
    }
}

fn move_west(x: usize, y: usize) -> Option<LightBeam> {
    if y == 0 {
        None
    } else {
        Some(LightBeam {
            direction: 3,
            x,
            y: y - 1,
        })
    }
}

fn solve(mut map: Vec<Vec<Tile>>, start: LightBeam) -> u32 {
    let mut bfs_queue = VecDeque::from([start]);

    while let Some(LightBeam { direction, x, y }) = bfs_queue.pop_front() {
        if (map[x][y].directions >> direction) & 1 == 1 {
            continue;
        }

        map[x][y].directions |= 1 << direction;

        match map[x][y].space {
            Space::EmptySpace => match direction {
                0 => {
                    if let Some(light_beam) = move_north(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                1 => {
                    if let Some(light_beam) = move_south(x, y, map.len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                2 => {
                    if let Some(light_beam) = move_east(x, y, map[0].len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                3 => {
                    if let Some(light_beam) = move_west(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                _ => unreachable!(),
            },
            Space::MirrorForwardSlash => match direction {
                0 => {
                    if let Some(light_beam) = move_east(x, y, map[0].len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                1 => {
                    if let Some(light_beam) = move_west(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                2 => {
                    if let Some(light_beam) = move_north(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                3 => {
                    if let Some(light_beam) = move_south(x, y, map.len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                _ => unreachable!(),
            },
            Space::MirrorBackwardSlash => match direction {
                0 => {
                    if let Some(light_beam) = move_west(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                1 => {
                    if let Some(light_beam) = move_east(x, y, map[0].len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                2 => {
                    if let Some(light_beam) = move_south(x, y, map.len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                3 => {
                    if let Some(light_beam) = move_north(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                _ => unreachable!(),
            },
            Space::SplitterPipe => match direction {
                0 => {
                    if let Some(light_beam) = move_north(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                1 => {
                    if let Some(light_beam) = move_south(x, y, map.len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                2 | 3 => {
                    if let Some(light_beam) = move_north(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                    if let Some(light_beam) = move_south(x, y, map.len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                _ => unreachable!(),
            },
            Space::SplitterDash => match direction {
                0 | 1 => {
                    if let Some(light_beam) = move_east(x, y, map[0].len()) {
                        bfs_queue.push_back(light_beam);
                    }
                    if let Some(light_beam) = move_west(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                2 => {
                    if let Some(light_beam) = move_east(x, y, map[0].len()) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                3 => {
                    if let Some(light_beam) = move_west(x, y) {
                        bfs_queue.push_back(light_beam);
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|space| space.directions != 0).count() as u32)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();

    let start = LightBeam {
        direction: 2,
        x: 0,
        y: 0,
    };
    Some(solve(map, start))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();

    let mut start_locations: Vec<LightBeam> = Vec::with_capacity(map.len() * 2 + map[0].len() * 2);
    for i in 0..map.len() {
        start_locations.push(LightBeam {
            direction: 2,
            x: i,
            y: 0,
        });
        start_locations.push(LightBeam {
            direction: 3,
            x: i,
            y: map[0].len() - 1,
        });
    }
    for j in 0..map[0].len() {
        start_locations.push(LightBeam {
            direction: 1,
            x: 0,
            y: j,
        });
        start_locations.push(LightBeam {
            direction: 0,
            x: map.len() - 1,
            y: j,
        });
    }

    Some(
        start_locations
            .iter()
            .map(|start| solve(map.to_vec(), *start))
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
