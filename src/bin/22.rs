use std::collections::{HashMap, HashSet, VecDeque};

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
    z_end: u32,
    tainted: bool,
}

#[derive(Debug)]
struct BrickNode {
    parents: Vec<usize>,
    children: Vec<usize>,
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

fn stack_bricks(bricks: &[Brick]) -> (Vec<usize>, HashMap<usize, BrickNode>) {
    let mut brick_graph: HashMap<usize, BrickNode> = HashMap::new();
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
                    z_end: cur_z_end,
                    tainted: false,
                });
            }
        }

        for bid in &taints_id {
            let children = &mut brick_graph.get_mut(bid).unwrap().children;
            if !children.contains(&cur_bid) {
                children.push(cur_bid)
            }
        }

        brick_graph.insert(
            cur_bid,
            BrickNode {
                parents: taints_id.into_iter().collect(),
                children: Vec::new(),
            },
        );
    }

    let mut disintegrated_status: HashMap<usize, bool> = HashMap::new();
    for x in 0..10 {
        for y in 0..10 {
            for BrickPiece {
                brick_id, tainted, ..
            } in grid[x][y].iter()
            {
                *disintegrated_status.entry(*brick_id).or_insert(false) |= *tainted;
            }
        }
    }
    let disintegrated: Vec<usize> = disintegrated_status
        .iter()
        .filter_map(|(bid, status)| if !status { Some(*bid) } else { None })
        .collect();

    (disintegrated, brick_graph)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut bricks) = parse_input(input).unwrap();
    bricks.sort_by_key(
        |Brick {
             start: Coord { z, .. },
             ..
         }| *z,
    );

    let (disintegrated, _) = stack_bricks(&bricks);
    Some(disintegrated.iter().count())
}

fn calculate_fall(
    brick_graph: &HashMap<usize, BrickNode>,
    start: usize,
    disintegrated: &[usize],
) -> usize {
    if disintegrated.contains(&start) {
        return 0;
    }

    let mut brick_chain: HashSet<usize> = HashSet::new();
    let mut waiting: HashMap<usize, HashSet<usize>> = HashMap::new();

    let neighbors = brick_graph
        .get(&start)
        .unwrap()
        .children
        .iter()
        .map(|bid| (*bid, start));
    let mut bfs = VecDeque::from_iter(neighbors);

    while let Some((curr_bid, prev_bid)) = bfs.pop_front() {
        if brick_chain.contains(&curr_bid) {
            continue;
        }

        let BrickNode {
            parents, children, ..
        } = brick_graph.get(&curr_bid).unwrap();

        if parents.len() > 1 {
            let waiting_for = waiting
                .entry(curr_bid)
                .or_insert(HashSet::from_iter(parents.clone()));
            waiting_for.remove(&prev_bid);
            if !waiting_for.is_empty() {
                continue;
            }
        }

        for c in children {
            bfs.push_back((*c, curr_bid));
        }
        brick_chain.insert(curr_bid);
    }

    brick_chain.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, mut bricks) = parse_input(input).unwrap();
    bricks.sort_by_key(
        |Brick {
             start: Coord { z, .. },
             ..
         }| *z,
    );
    let (disintegrated, brick_graph) = stack_bricks(&bricks);

    Some(
        (0..brick_graph.len())
            .map(|bid| calculate_fall(&brick_graph, bid, &disintegrated))
            .sum(),
    )
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
