use std::collections::HashSet;

use nom::{
    bytes::complete::take_till1, character::complete::newline, multi::separated_list1, IResult,
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

pub fn part_two(input: &str) -> Option<usize> {
    let (_, (start, map)) = parse_input(input).unwrap();
    let remaining_steps = 5000;

    let steps = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut current_steps: HashSet<(i64, i64)> = HashSet::from([start]);
    let mut next_steps: HashSet<(i64, i64)> = HashSet::new();

    let rows = map.len() as i64;
    let cols = map[0].len() as i64;

    for _ in 0..remaining_steps {
        for (x, y) in current_steps {
            for (dx, dy) in steps {
                let (x_new, y_new) = (x + dx, y + dy);

                let x_map = ((x_new % rows) + rows) % rows;
                let y_map = ((y_new % cols) + cols) % cols;

                if map[x_map as usize][y_map as usize] == Space::GardenPlot {
                    next_steps.insert((x_new, y_new));
                }
            }
        }
        current_steps = next_steps.clone();
        next_steps.clear();
    }

    Some(current_steps.len())
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(16));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
