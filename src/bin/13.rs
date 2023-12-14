use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::newline,
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(13);

fn parse_reflection(input: &str) -> IResult<&str, Vec<&[u8]>> {
    let (input, reflection) = separated_list1(newline, take_till1(|c| c == '\n'))(input)?;
    Ok((
        input,
        reflection.iter().map(|line| line.as_bytes()).collect(),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<&[u8]>>> {
    separated_list1(tag("\n\n"), parse_reflection)(input)
}

fn conversion(space: &u8) -> u64 {
    match space {
        b'.' => 0,
        b'#' => 1,
        _ => unreachable!(),
    }
}

fn to_bitmap(map: &Vec<&[u8]>) -> (Vec<u64>, Vec<u64>) {
    let row_bitmap: Vec<u64> = map
        .iter()
        .map(|row| row.iter().fold(0, |acc, c| (acc << 1) | conversion(c)))
        .collect();

    let rows = map.len();
    let cols = map[0].len();
    let col_bitmap: Vec<u64> = (0..cols)
        .map(|j| (0..rows).fold(0, |acc, i| (acc << 1) | conversion(&map[i][j])))
        .collect();

    (row_bitmap, col_bitmap)
}

fn identify_reflection(map: &[u64], smudg: Option<usize>) -> Option<usize> {
    for i in 1..map.len() {
        let mut left = i - 1;
        let mut right = i;
        while 0 < left && right < map.len() - 1 {
            if map[left] != map[right] {
                break;
            }
            left -= 1;
            right += 1;
        }

        if (left == 0 || right == map.len() - 1) && map[left] == map[right] {
            if let Some(smudge_idx) = smudg {
                if left <= smudge_idx && smudge_idx <= right {
                    return Some(i);
                }
            } else {
                return Some(i);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reflections_map) = parse_input(input).unwrap();
    let reflections_bitmap: Vec<(Vec<u64>, Vec<u64>)> =
        reflections_map.iter().map(|map| to_bitmap(map)).collect();

    Some(
        reflections_bitmap
            .iter()
            .map(|(horizontal, vertical)| {
                if let Some(reflect_count) = identify_reflection(horizontal, None) {
                    reflect_count * 100
                } else {
                    identify_reflection(vertical, None).unwrap()
                }
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reflections_map) = parse_input(input).unwrap();
    let mut reflections_bitmap: Vec<(Vec<u64>, Vec<u64>)> =
        reflections_map.iter().map(|map| to_bitmap(map)).collect();

    Some(
        reflections_bitmap
            .iter_mut()
            .map(|(horizontal, vertical)| {
                for i in 0..horizontal.len() {
                    for n in (0..vertical.len()).rev() {
                        horizontal[i] ^= 1 << n;
                        if let Some(reflect_count) = identify_reflection(&horizontal, Some(i)) {
                            return reflect_count * 100;
                        }
                        horizontal[i] ^= 1 << n;
                    }
                }

                for i in 0..vertical.len() {
                    for n in (0..horizontal.len()).rev() {
                        vertical[i] ^= 1 << n;
                        if let Some(reflect_count) = identify_reflection(&vertical, Some(i)) {
                            return reflect_count;
                        }
                        vertical[i] ^= 1 << n;
                    }
                }

                unreachable!()
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
