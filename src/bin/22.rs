use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(22);

struct Coord {
    x: usize,
    y: usize,
    z: u32,
}

struct Brick {
    start: Coord,
    end: Coord,
}

#[derive(Copy, Clone, Debug)]
struct BrickPiece {
    brick_id: usize,
    z_start: u32,
    z_end: u32,
    tainted: bool,
}

fn parse_brick(input: &str) -> IResult<&str, Coord> {
    let (input, x) = complete::u32(input)?;
    let (input, y) = preceded(tag(","), complete::u32)(input)?;
    let (input, z) = preceded(tag(","), complete::u32)(input)?;
    Ok((
        input,
        Coord {
            x: x as usize,
            y: y as usize,
            z,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(
        newline,
        separated_pair(parse_brick, tag("~"), parse_brick).map(|(start, end)| Brick { start, end }),
    )(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut bricks) = parse_input(input).unwrap();
    bricks.sort_by_key(
        |Brick {
             start: Coord { z, .. },
             ..
         }| *z,
    );

    let mut grid: [[Vec<BrickPiece>; 10]; 10] = Default::default();
    for (cur_bid, brick) in bricks.iter().enumerate() {
        let Brick {
            start: cur_start,
            end: cur_end,
        } = brick;

        let mut z_placement: u32 = 0;
        for x in cur_start.x..=cur_end.x {
            for y in cur_start.y..=cur_end.y {
                if let Some(BrickPiece { z_end, .. }) = grid[x][y].iter().last() {
                    z_placement = z_placement.max(*z_end);
                }
            }
        }

        let mut taints_id: HashSet<usize> = HashSet::new();
        let mut taints_xy: Vec<(usize, usize)> = Vec::new();
        for x in cur_start.x..=cur_end.x {
            for y in cur_start.y..=cur_end.y {
                if let Some(BrickPiece {
                    brick_id, z_end, ..
                }) = grid[x][y].iter().last()
                {
                    if z_placement == *z_end {
                        taints_id.insert(*brick_id);
                        taints_xy.push((x, y));
                    }
                }
            }
        }

        let cur_z_start = z_placement + 1;
        let cur_z_end = cur_end.z - cur_start.z + cur_z_start;
        for x in cur_start.x..=cur_end.x {
            for y in cur_start.y..=cur_end.y {
                if taints_id.len() == 1 && taints_xy.contains(&(x, y)) {
                    grid[x][y].iter_mut().last().unwrap().tainted = true;
                }
                grid[x][y].push(BrickPiece {
                    brick_id: cur_bid,
                    z_start: cur_z_start,
                    z_end: cur_z_end,
                    tainted: false,
                });
            }
        }
    }

    let mut disintegrated: HashMap<usize, bool> = HashMap::new();
    for x in 0..10 {
        for y in 0..10 {
            for BrickPiece {
                brick_id, tainted, ..
            } in grid[x][y].iter()
            {
                if let Some(taint_status) = disintegrated.get_mut(brick_id) {
                    if !(*taint_status) {
                        *taint_status = *tainted;
                    }
                } else {
                    disintegrated.insert(*brick_id, *tainted);
                }
            }
        }
    }

    Some(
        disintegrated
            .iter()
            .filter(|(_, tainted)| !**tainted)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
