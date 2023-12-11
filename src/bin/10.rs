use std::collections::VecDeque;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Animal {
    x: usize,
    y: usize,
    steps: u32,
}

// 0001 : (1 << 0) : north
// 0010 : (1 << 1) : south
// 0100 : (1 << 2) : east
// 1000 : (1 << 3) : west
// 0000 : 0        : ground
fn parse_tile(tile: &u8) -> u8 {
    match tile {
        b'|' => (1 << 0) | (1 << 1), // north south : 3
        b'-' => (1 << 2) | (1 << 3), // east west   : 12
        b'L' => (1 << 0) | (1 << 2), // north east  : 5
        b'J' => (1 << 0) | (1 << 3), // north west  : 9
        b'7' => (1 << 1) | (1 << 3), // south west  : 10
        b'F' => (1 << 1) | (1 << 2), // south east  : 6
        b'S' => (1 << 4) - 1,        // north south east west
        _ => 0,                      // ground
    }
}

fn parse_input(input: &str) -> (Animal, Vec<Vec<u8>>) {
    let mut start: Option<Animal> = None;
    let lines = input.lines();
    let mut map: Vec<Vec<u8>> = Vec::with_capacity(lines.clone().count());

    for (i, l) in lines.enumerate() {
        let tile_line: Vec<u8> =
            l.as_bytes()
                .iter()
                .enumerate()
                .fold(Vec::with_capacity(l.len()), |mut acc, (j, t)| {
                    if start.is_none() && t == &b'S' {
                        start = Some(Animal {
                            x: i,
                            y: j,
                            steps: 0,
                        });
                    }

                    acc.push(parse_tile(t));
                    acc
                });

        map.push(tile_line);
    }

    (start.unwrap(), map)
}

// screw it
fn clean_tiles(map: &mut Vec<Vec<u8>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                continue;
            }

            if i == 0 || ((map[i - 1][j] >> 1) & 1) & ((map[i][j] >> 0) & 1) == 0 {
                map[i][j] &= !(1 << 0);
            }
            if i == map.len() - 1 || ((map[i + 1][j] >> 0) & 1) & ((map[i][j] >> 1) & 1) == 0 {
                map[i][j] &= !(1 << 1);
            }
            if j == map[0].len() - 1 || ((map[i][j + 1] >> 3) & 1) & ((map[i][j] >> 2) & 1) == 0 {
                map[i][j] &= !(1 << 2);
            }
            if j == 0 || ((map[i][j - 1] >> 2) & 1) & ((map[i][j] >> 3) & 1) == 0 {
                map[i][j] &= !(1 << 3);
            }
        }
    }
}

fn bfs(map: &mut Vec<Vec<u8>>, start: Animal) -> u32 {
    let mut animals_move: VecDeque<Animal> = VecDeque::from([start]);
    let mut animals_meet_steps: Vec<u32> = Vec::new();

    while let Some(Animal { x, y, steps }) = animals_move.pop_front() {
        let current_tile = &mut map[x][y];
        if (*current_tile >> 4) & 1 == 1 {
            animals_meet_steps.push(steps);
            continue;
        }
        *current_tile |= 1 << 4;

        if (*current_tile >> 0) & 1 == 1 {
            animals_move.push_back(Animal {
                x: x - 1,
                y: y,
                steps: steps + 1,
            });
        }
        if (*current_tile >> 1) & 1 == 1 {
            animals_move.push_back(Animal {
                x: x + 1,
                y: y,
                steps: steps + 1,
            });
        }
        if (*current_tile >> 2) & 1 == 1 {
            animals_move.push_back(Animal {
                x: x,
                y: y + 1,
                steps: steps + 1,
            });
        }
        if (*current_tile >> 3) & 1 == 1 {
            animals_move.push_back(Animal {
                x: x,
                y: y - 1,
                steps: steps + 1,
            });
        }
    }

    animals_meet_steps.iter().max().unwrap() - 1
}

fn dfs(map: &mut Vec<Vec<u8>>, start: &(usize, usize), current: (usize, usize)) -> Option<u32> {
    if (map[current.0][current.1] >> 4) & 1 == 1 {
        if start == &current {
            return Some(0);
        } else {
            return None;
        }
    }
    map[current.0][current.1] |= 1 << 4;

    let mut steps: Option<u32> = None;
    if (map[current.0][current.1] >> 0) & 1 == 1 {
        if let Some(dfs_steps) = dfs(map, start, (current.0 - 1, current.1)) {
            steps = Some(steps.unwrap_or_default().max(dfs_steps) + 1);
        }
    }
    if (map[current.0][current.1] >> 1) & 1 == 1 {
        if let Some(dfs_steps) = dfs(map, start, (current.0 + 1, current.1)) {
            steps = Some(steps.unwrap_or_default().max(dfs_steps) + 1);
        }
    }
    if (map[current.0][current.1] >> 2) & 1 == 1 {
        if let Some(dfs_steps) = dfs(map, start, (current.0, current.1 + 1)) {
            steps = Some(steps.unwrap_or_default().max(dfs_steps) + 1);
        }
    }
    if (map[current.0][current.1] >> 3) & 1 == 1 {
        if let Some(dfs_steps) = dfs(map, start, (current.0, current.1 - 1)) {
            steps = Some(steps.unwrap_or_default().max(dfs_steps) + 1);
        }
    }

    steps
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, mut map) = parse_input(&input);
    clean_tiles(&mut map);
    Some(bfs(&mut map, start))
    // Some(dfs(&mut map, &(animal.x, animal.y), (animal.x, animal.y)).unwrap() / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (animal, mut map) = parse_input(&input);
    clean_tiles(&mut map);
    dfs(&mut map, &(animal.x, animal.y), (animal.x, animal.y));

    for r in map.iter_mut() {
        for c in r.iter_mut() {
            if (*c >> 4) & 1 == 0 {
                *c = 0;
            }
        }
    }

    let mut in_tiles = 0;
    for r in &map {
        let mut crosses_count: u8 = 0;

        let mut cross_flag = 0;
        for c in r {
            cross_flag ^= ((1 << 2) - 1) & c;
            if cross_flag == (1 << 1) | 1 {
                crosses_count += 1;
                cross_flag = 0;
            }
        }

        if crosses_count % 2 == 1 {
            continue;
        }

        let mut crosses_count: u8 = 0;
        for c in r {
            if *c == 0 && crosses_count % 2 == 1 {
                in_tiles += 1;
            } else {
                cross_flag ^= ((1 << 2) - 1) & c;
                if cross_flag == (1 << 1) | 1 {
                    crosses_count += 1;
                    cross_flag = 0;
                }
            }
        }
    }

    Some(in_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
