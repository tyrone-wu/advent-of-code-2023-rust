use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use nom::{
    bytes::complete::take_till1, character::complete::newline, multi::separated_list1, IResult,
};

advent_of_code::solution!(17);

fn parse_row(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, row) = take_till1(|c| c == '\n')(input)?;
    Ok((input, row.bytes().map(|c| (c - b'0') as u32).collect()))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, row) = separated_list1(newline, parse_row)(input)?;
    Ok((input, row))
}

fn step(
    current_pos: (usize, usize),
    direction: (i32, i32),
    current_heat_loss: u32,
    map: &[Vec<u32>],
    current_path: u64,
    prev_shift: u8,
    unvisited: &mut BinaryHeap<Reverse<(u32, (usize, usize), u64)>>,
) {
    let dst = (
        (current_pos.0 as i32 + direction.0) as usize,
        (current_pos.1 as i32 + direction.1) as usize,
    );
    let new_heat_loss = current_heat_loss + map[dst.0][dst.1];
    let dst_path = (current_path >> 4) | (1 << prev_shift);
    unvisited.push(Reverse((new_heat_loss, dst, dst_path)));
}

// 0001 : north
// 0010 : south
// 0100 : east
// 1000 : west
pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();
    let target = (map.len() - 1, map[0].len() - 1);

    let mut visited: HashSet<((usize, usize), u64)> = HashSet::new();
    let mut unvisited: BinaryHeap<Reverse<(u32, (usize, usize), u64)>> =
        BinaryHeap::from([Reverse((0, (0, 0), 0))]);

    let prev_shift = 8;

    let (north_lim, south_lim, east_lim, west_lim) = (0..3).fold((0, 0, 0, 0), |acc, _| {
        (
            (acc.0 << 4) | 1,
            (acc.1 << 4) | (1 << 1),
            (acc.2 << 4) | (1 << 2),
            (acc.3 << 4) | (1 << 3),
        )
    });

    while let Some(Reverse((current_heat_loss, current_pos, current_path))) = unvisited.pop() {
        if current_pos == target {
            return Some(current_heat_loss);
        } else if visited.contains(&(current_pos, current_path)) {
            continue;
        }
        visited.insert((current_pos, current_path));

        if current_path != north_lim
            && current_pos.0 > 0
            && (current_path >> (prev_shift + 1)) & 1 == 0
        {
            step(
                current_pos,
                (-1, 0),
                current_heat_loss,
                &map,
                current_path,
                prev_shift,
                &mut unvisited,
            );
        }
        if current_path != south_lim
            && current_pos.0 < map.len() - 1
            && (current_path >> prev_shift) & 1 == 0
        {
            step(
                current_pos,
                (1, 0),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 1,
                &mut unvisited,
            );
        }
        if current_path != east_lim
            && current_pos.1 < map[0].len() - 1
            && (current_path >> (prev_shift + 3)) & 1 == 0
        {
            step(
                current_pos,
                (0, 1),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 2,
                &mut unvisited,
            );
        }
        if current_path != west_lim
            && current_pos.1 > 0
            && (current_path >> (prev_shift + 2)) & 1 == 0
        {
            step(
                current_pos,
                (0, -1),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 3,
                &mut unvisited,
            );
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();
    let target = (map.len() - 1, map[0].len() - 1);

    let mut visited: HashSet<((usize, usize), u64)> = HashSet::new();
    let mut unvisited: BinaryHeap<Reverse<(u32, (usize, usize), u64)>> =
        BinaryHeap::from([Reverse((0, (0, 0), 0))]);

    let prev_shift = 36;
    let prev_4_mask = ((1 << 16) - 1) << 24;

    let (north_lim, south_lim, east_lim, west_lim) = (0..10).fold((0, 0, 0, 0), |acc, _| {
        (
            (acc.0 << 4) | 1,
            (acc.1 << 4) | (1 << 1),
            (acc.2 << 4) | (1 << 2),
            (acc.3 << 4) | (1 << 3),
        )
    });

    while let Some(Reverse((current_heat_loss, current_pos, current_path))) = unvisited.pop() {
        let current_prev_4 = current_path & prev_4_mask;
        if current_pos == target
            && ((current_prev_4 == north_lim & prev_4_mask)
                || (current_prev_4 == south_lim & prev_4_mask)
                || (current_prev_4 == east_lim & prev_4_mask)
                || (current_prev_4 == west_lim & prev_4_mask))
        {
            return Some(current_heat_loss);
        } else if visited.contains(&(current_pos, current_path)) {
            continue;
        }
        visited.insert((current_pos, current_path));

        let mut valid_path = (1 << 4) - 1;
        if current_path == north_lim
            || current_path == south_lim
            || current_path == east_lim
            || current_path == west_lim
        {
            valid_path ^= current_path >> prev_shift;
        } else if (current_prev_4 != north_lim & prev_4_mask)
            && (current_prev_4 != south_lim & prev_4_mask)
            && (current_prev_4 != east_lim & prev_4_mask)
            && (current_prev_4 != west_lim & prev_4_mask)
            && current_path != 0
        {
            valid_path = current_path >> prev_shift;
        }

        if valid_path & 1 == 1 && current_pos.0 > 0 && (current_path >> (prev_shift + 1)) & 1 == 0 {
            step(
                current_pos,
                (-1, 0),
                current_heat_loss,
                &map,
                current_path,
                prev_shift,
                &mut unvisited,
            );
        }
        if (valid_path >> 1) & 1 == 1
            && current_pos.0 < map.len() - 1
            && (current_path >> prev_shift) & 1 == 0
        {
            step(
                current_pos,
                (1, 0),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 1,
                &mut unvisited,
            );
        }
        if (valid_path >> 2) & 1 == 1
            && current_pos.1 < map[0].len() - 1
            && (current_path >> (prev_shift + 3)) & 1 == 0
        {
            step(
                current_pos,
                (0, 1),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 2,
                &mut unvisited,
            );
        }
        if (valid_path >> 3) & 1 == 1
            && current_pos.1 > 0
            && (current_path >> (prev_shift + 2)) & 1 == 0
        {
            step(
                current_pos,
                (0, -1),
                current_heat_loss,
                &map,
                current_path,
                prev_shift + 3,
                &mut unvisited,
            );
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(71));
    }
}
