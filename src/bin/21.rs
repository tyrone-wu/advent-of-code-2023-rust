use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::take_till1,
    character::complete::newline,
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(21);

#[derive(PartialEq)]
enum Space {
    GardenPlot,
    Rock,
}

fn parse_input(input: &str) -> IResult<&str, ((i64, i64), Vec<Vec<Space>>)> {
    let (input, rows) = separated_list1(newline, take_till1(|c| c == '\n'))(input)?;
    let mut start = (0, 0);
    let map = rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, c)| match c {
                    b'.' => Space::GardenPlot,
                    b'#' => Space::Rock,
                    b'S' => {
                        start = (i as i64, j as i64);
                        Space::GardenPlot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Ok((input, (start, map)))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (start, map)) = parse_input(input).unwrap();
    let remaining_steps = 64;

    let steps = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut current_steps: HashSet<(i64, i64)> = HashSet::from([start]);
    let mut next_steps: HashSet<(i64, i64)> = HashSet::new();

    let rows = map.len() as i64;
    let cols = map[0].len() as i64;

    for _ in 0..remaining_steps {
        for (x, y) in current_steps {
            for (dx, dy) in steps {
                let (x_new, y_new) = (x + dx, y + dy);
                if 0 <= x_new
                    && x_new < rows
                    && 0 <= y_new
                    && y_new < cols
                    && map[x_new as usize][y_new as usize] == Space::GardenPlot
                {
                    next_steps.insert((x_new, y_new));
                }
            }
        }
        current_steps = next_steps.clone();
        next_steps.clear();
    }

    Some(current_steps.len())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, (start, map)) = parse_input(input).unwrap();
    let remaining_steps = 26501365;

    let rows = map.len() as i64;
    let cols = map[0].len() as i64;

    // ty icub3d from yt again!
    let steps = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut step_distances: HashMap<(i64, i64), u32> = HashMap::new();
    let mut frontier: VecDeque<((i64, i64), u32)> = VecDeque::from([(start, 0)]);
    while let Some((position, distance)) = frontier.pop_front() {
        if step_distances.contains_key(&position) {
            continue;
        }

        step_distances.insert(position, distance);
        for (dx, dy) in steps {
            let (x_new, y_new) = (position.0 + dx, position.1 + dy);
            if 0 <= x_new
                && x_new < rows
                && 0 <= y_new
                && y_new < cols
                && map[x_new as usize][y_new as usize] == Space::GardenPlot
            {
                frontier.push_back(((x_new, y_new), distance + 1));
            }
        }
    }

    let (odd, even, odd_edges, even_edges) = step_distances.iter().fold(
        (0_usize, 0_usize, 0_usize, 0_usize),
        |(mut odd, mut even, mut odd_edges, mut even_edges), (_, distance)| {
            if *distance % 2 == 0 {
                even += 1;
                if *distance > 65 {
                    even_edges += 1;
                }
            } else {
                odd += 1;
                if *distance > 65 {
                    odd_edges += 1;
                }
            }
            (odd, even, odd_edges, even_edges)
        },
    );

    let grid_steps = (remaining_steps - (rows / 2)) / rows;
    let total_odd = (grid_steps + 1).pow(2) * odd as i64;
    let total_even = grid_steps.pow(2) * even as i64;
    let total_odd_edges = (grid_steps + 1) * odd_edges as i64;
    let total_even_edges = grid_steps * even_edges as i64;

    Some(total_even + total_odd + total_even_edges - total_odd_edges)

    // let mut current_steps: HashSet<(i64, i64)> = HashSet::from([start]);
    // let mut next_steps: HashSet<(i64, i64)> = HashSet::new();

    // for _ in 0..remaining_steps {
    //     for (x, y) in current_steps {
    //         for (dx, dy) in steps {
    //             let (x_new, y_new) = (x + dx, y + dy);

    //             let x_map = ((x_new % rows) + rows) % rows;
    //             let y_map = ((y_new % cols) + cols) % cols;

    //             if map[x_map as usize][y_map as usize] == Space::GardenPlot {
    //                 next_steps.insert((x_new, y_new));
    //             }
    //         }
    //     }
    //     current_steps = next_steps.clone();
    //     next_steps.clear();
    // }

    // Some(current_steps.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16733044));
    }
}
