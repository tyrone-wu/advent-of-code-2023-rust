use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    Some(
        map.iter()
            .enumerate()
            .map(|(i, row)| {
                let mut part_num_row_sum = 0;

                let mut num = 0;
                let mut is_part_num = false;

                for (j, c) in row.iter().enumerate() {
                    if c.is_ascii_digit() {
                        if !is_part_num {
                            if i > 0 {
                                if j > 0
                                    && !map[i - 1][j - 1].is_ascii_digit()
                                    && map[i - 1][j - 1] != b'.'
                                {
                                    is_part_num = true;
                                } else if !map[i - 1][j].is_ascii_digit() && map[i - 1][j] != b'.' {
                                    is_part_num = true;
                                } else if j + 1 < row.len()
                                    && !map[i - 1][j + 1].is_ascii_digit()
                                    && map[i - 1][j + 1] != b'.'
                                {
                                    is_part_num = true;
                                }
                            }
                            if i + 1 < row.len() {
                                if j > 0
                                    && !map[i + 1][j - 1].is_ascii_digit()
                                    && map[i + 1][j - 1] != b'.'
                                {
                                    is_part_num = true;
                                } else if !map[i + 1][j].is_ascii_digit() && map[i + 1][j] != b'.' {
                                    is_part_num = true;
                                } else if j + 1 < row.len()
                                    && !map[i + 1][j + 1].is_ascii_digit()
                                    && map[i + 1][j + 1] != b'.'
                                {
                                    is_part_num = true;
                                }
                            }
                            if j > 0 && !row[j - 1].is_ascii_digit() && row[j - 1] != b'.' {
                                is_part_num = true;
                            }
                            if j + 1 < row.len()
                                && !row[j + 1].is_ascii_digit()
                                && row[j + 1] != b'.'
                            {
                                is_part_num = true;
                            }
                        }
                        num = num * 10 + (c - b'0') as u32;
                    } else {
                        if is_part_num {
                            part_num_row_sum += num;
                        }
                        num = 0;
                        is_part_num = false;
                    }
                }

                if is_part_num {
                    part_num_row_sum += num;
                }

                part_num_row_sum
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        let mut num = 0;
        let mut gear_coord: Option<(usize, usize)> = None;

        for (j, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                if gear_coord.is_none() {
                    if i > 0 {
                        if j > 0 && !map[i - 1][j - 1].is_ascii_digit() && map[i - 1][j - 1] == b'*'
                        {
                            gear_coord = Some((i - 1, j - 1));
                        } else if !map[i - 1][j].is_ascii_digit() && map[i - 1][j] == b'*' {
                            gear_coord = Some((i - 1, j));
                        } else if j + 1 < row.len()
                            && !map[i - 1][j + 1].is_ascii_digit()
                            && map[i - 1][j + 1] == b'*'
                        {
                            gear_coord = Some((i - 1, j + 1));
                        }
                    }
                    if i + 1 < row.len() {
                        if j > 0 && !map[i + 1][j - 1].is_ascii_digit() && map[i + 1][j - 1] == b'*'
                        {
                            gear_coord = Some((i + 1, j - 1));
                        } else if !map[i + 1][j].is_ascii_digit() && map[i + 1][j] == b'*' {
                            gear_coord = Some((i + 1, j));
                        } else if j + 1 < row.len()
                            && !map[i + 1][j + 1].is_ascii_digit()
                            && map[i + 1][j + 1] == b'*'
                        {
                            gear_coord = Some((i + 1, j + 1));
                        }
                    }
                    if j > 0 && !row[j - 1].is_ascii_digit() && row[j - 1] == b'*' {
                        gear_coord = Some((i, j - 1));
                    }
                    if j + 1 < row.len() && !row[j + 1].is_ascii_digit() && row[j + 1] == b'*' {
                        gear_coord = Some((i, j + 1));
                    }
                }
                num = num * 10 + (c - b'0') as u32;
            } else {
                if let Some(gear) = gear_coord {
                    gears.entry(gear).or_insert(Vec::new()).push(num);
                }
                num = 0;
                gear_coord = None;
            }
        }

        if let Some(gear) = gear_coord {
            gears.entry(gear).or_insert(Vec::new()).push(num);
        }
    }

    let mut gear_ratio = 0;
    for (_, nums) in &gears {
        if nums.len() == 2 {
            gear_ratio += nums.iter().product::<u32>();
        }
    }

    Some(gear_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
