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
    energized: u32,
    directions: u8,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
struct LightBeam {
    direction: Direction,
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
                energized: 0,
                directions: 0,
            })
            .collect(),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_tile)(input)
}

fn solve(mut map: Vec<Vec<Tile>>, start: LightBeam) -> u32 {
    let mut bfs_queue = VecDeque::from([start]);

    while let Some(LightBeam { direction, x, y }) = bfs_queue.pop_front() {
        if map[x][y].energized > 0 {
            match direction {
                Direction::North => {
                    if map[x][y].directions >> 0 & 1 == 1 {
                        continue;
                    }
                }
                Direction::South => {
                    if map[x][y].directions >> 1 & 1 == 1 {
                        continue;
                    }
                }
                Direction::East => {
                    if map[x][y].directions >> 2 & 1 == 1 {
                        continue;
                    }
                }
                Direction::West => {
                    if map[x][y].directions >> 3 & 1 == 1 {
                        continue;
                    }
                }
            }
        }

        map[x][y].energized += 1;
        map[x][y].directions |= 1 << {
            match direction {
                Direction::North => 0,
                Direction::South => 1,
                Direction::East => 2,
                Direction::West => 3,
            }
        };

        match map[x][y].space {
            Space::EmptySpace => match direction {
                Direction::North => {
                    if x == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x: x - 1,
                        y,
                    });
                }
                Direction::South => {
                    if x == map.len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x: x + 1,
                        y,
                    });
                }
                Direction::East => {
                    if y == map[0].len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x,
                        y: y + 1,
                    });
                }
                Direction::West => {
                    if y == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x,
                        y: y - 1,
                    });
                }
            },
            Space::MirrorForwardSlash => match direction {
                Direction::North => {
                    if y == map[0].len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::East,
                        x,
                        y: y + 1,
                    });
                }
                Direction::South => {
                    if y == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::West,
                        x,
                        y: y - 1,
                    });
                }
                Direction::East => {
                    if x == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::North,
                        x: x - 1,
                        y,
                    });
                }
                Direction::West => {
                    if x == map.len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::South,
                        x: x + 1,
                        y,
                    });
                }
            },
            Space::MirrorBackwardSlash => match direction {
                Direction::North => {
                    if y == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::West,
                        x,
                        y: y - 1,
                    });
                }
                Direction::South => {
                    if y == map[0].len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::East,
                        x,
                        y: y + 1,
                    });
                }
                Direction::East => {
                    if x == map.len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::South,
                        x: x + 1,
                        y,
                    });
                }
                Direction::West => {
                    if x == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction: Direction::North,
                        x: x - 1,
                        y,
                    });
                }
            },
            Space::SplitterPipe => match direction {
                Direction::North => {
                    if x == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x: x - 1,
                        y,
                    });
                }
                Direction::South => {
                    if x == map.len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x: x + 1,
                        y,
                    });
                }
                Direction::East | Direction::West => {
                    if x > 0 {
                        bfs_queue.push_back(LightBeam {
                            direction: Direction::North,
                            x: x - 1,
                            y,
                        });
                    }
                    if x < map.len() - 1 {
                        bfs_queue.push_back(LightBeam {
                            direction: Direction::South,
                            x: x + 1,
                            y,
                        });
                    }
                }
            },
            Space::SplitterDash => match direction {
                Direction::North | Direction::South => {
                    if y < map[0].len() - 1 {
                        bfs_queue.push_back(LightBeam {
                            direction: Direction::East,
                            x,
                            y: y + 1,
                        });
                    }
                    if y > 0 {
                        bfs_queue.push_back(LightBeam {
                            direction: Direction::West,
                            x,
                            y: y - 1,
                        });
                    }
                }
                Direction::East => {
                    if y == map[0].len() - 1 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x,
                        y: y + 1,
                    });
                }
                Direction::West => {
                    if y == 0 {
                        continue;
                    }
                    bfs_queue.push_back(LightBeam {
                        direction,
                        x,
                        y: y - 1,
                    });
                }
            },
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|space| space.energized > 0).count() as u32)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();

    let start = LightBeam {
        direction: Direction::East,
        x: 0,
        y: 0,
    };
    Some(solve(map, start))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();

    let mut start_locations: Vec<LightBeam> = Vec::new();
    for i in 0..map.len() {
        start_locations.push(LightBeam {
            direction: Direction::East,
            x: i,
            y: 0,
        });
        start_locations.push(LightBeam {
            direction: Direction::West,
            x: i,
            y: map[0].len() - 1,
        });
    }
    for j in 0..map[0].len() {
        start_locations.push(LightBeam {
            direction: Direction::South,
            x: 0,
            y: j,
        });
        start_locations.push(LightBeam {
            direction: Direction::North,
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
