use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult, Parser,
};

advent_of_code::solution!(23);

#[derive(Debug, PartialEq)]
enum Space {
    Forest,
    Path,
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Space>>> {
    separated_list1(
        newline,
        many1(alt((
            complete::char('#').map(|_| Space::Forest),
            complete::char('.').map(|_| Space::Path),
            complete::char('^').map(|_| Space::North),
            complete::char('v').map(|_| Space::South),
            complete::char('>').map(|_| Space::East),
            complete::char('<').map(|_| Space::West),
        ))),
    )(input)
}

fn dfs_p1(
    map: &[Vec<Space>],
    current: Coord,
    mut visited: HashSet<Coord>,
    part_one: bool,
) -> usize {
    if visited.contains(&current) {
        return 0;
    }
    visited.insert(current);

    let Coord { x, y } = current;
    if x == map.len() - 1 && y == map[0].len() - 2 {
        return visited.len() - 1;
    }

    let mut max_visited = 0;

    if (map[x][y] == Space::Path || map[x][y] == Space::North || !part_one)
        && x > 0
        && map[x - 1][y] != Space::Forest
        && (map[x - 1][y] != Space::South || !part_one)
    {
        max_visited = max_visited.max(dfs_p1(
            map,
            Coord { x: x - 1, y },
            visited.clone(),
            part_one,
        ));
    }
    if (map[x][y] == Space::Path || map[x][y] == Space::South || !part_one)
        && x < map.len() - 1
        && map[x + 1][y] != Space::Forest
        && (map[x + 1][y] != Space::North || !part_one)
    {
        max_visited = max_visited.max(dfs_p1(
            map,
            Coord { x: x + 1, y },
            visited.clone(),
            part_one,
        ));
    }
    if (map[x][y] == Space::Path || map[x][y] == Space::East || !part_one)
        && y < map[0].len() - 1
        && map[x][y + 1] != Space::Forest
        && (map[x][y + 1] != Space::West || !part_one)
    {
        max_visited = max_visited.max(dfs_p1(
            map,
            Coord { x, y: y + 1 },
            visited.clone(),
            part_one,
        ));
    }
    if (map[x][y] == Space::Path || map[x][y] == Space::West || !part_one)
        && y > 0
        && map[x][y - 1] != Space::Forest
        && (map[x][y - 1] != Space::East || !part_one)
    {
        max_visited = max_visited.max(dfs_p1(map, Coord { x, y: y - 1 }, visited, part_one));
    }

    max_visited
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, map) = parse_input(input).unwrap();
    Some(dfs_p1(&map, Coord { x: 0, y: 1 }, HashSet::new(), true))
}

fn map_to_graph(
    current: (Coord, u32),
    mut prev_intersection: (Coord, u32),
    graph: &mut HashMap<Coord, Vec<(Coord, u32)>>,
    map: &[Vec<Space>],
    visited: &mut HashSet<Coord>,
) {
    let (current_coord, mut current_steps) = current;
    let Coord { x, y } = current_coord;
    let (prev_coord, prev_int_steps) = prev_intersection;

    let mut open_paths = Vec::new();
    if x > 0 && map[x - 1][y] != Space::Forest {
        open_paths.push(Space::North);
    }
    if x < map.len() - 1 && map[x + 1][y] != Space::Forest {
        open_paths.push(Space::South);
    }
    if y > 0 && map[x][y - 1] != Space::Forest {
        open_paths.push(Space::West);
    }
    if y < map[0].len() - 1 && map[x][y + 1] != Space::Forest {
        open_paths.push(Space::East);
    }

    if open_paths.len() > 2 || (x == map.len() - 1 && y == map[0].len() - 2) {
        let steps_diff = current_steps.abs_diff(prev_int_steps);
        let to_intersection = graph.entry(current_coord).or_insert(Vec::new());
        if !to_intersection.contains(&(prev_coord, steps_diff)) {
            to_intersection.push((prev_coord, steps_diff));
        }

        let from_intersection = graph.get_mut(&prev_coord).unwrap();
        if !from_intersection.contains(&(current_coord, steps_diff)) {
            from_intersection.push((current_coord, steps_diff));
        }

        prev_intersection = current;
    }

    if visited.contains(&current_coord) {
        return;
    }
    visited.insert(current_coord);

    current_steps += 1;
    for path in open_paths {
        match path {
            Space::North => map_to_graph(
                (Coord { x: x - 1, y }, current_steps),
                prev_intersection,
                graph,
                map,
                visited,
            ),
            Space::South => map_to_graph(
                (Coord { x: x + 1, y }, current_steps),
                prev_intersection,
                graph,
                map,
                visited,
            ),
            Space::East => map_to_graph(
                (Coord { x, y: y + 1 }, current_steps),
                prev_intersection,
                graph,
                map,
                visited,
            ),
            Space::West => map_to_graph(
                (Coord { x, y: y - 1 }, current_steps),
                prev_intersection,
                graph,
                map,
                visited,
            ),
            _ => unreachable!(),
        }
    }
}

fn dfs_p2(
    graph: &HashMap<Coord, Vec<(Coord, u32)>>,
    current: (Coord, u32),
    mut visited: HashSet<Coord>,
    target: &Coord,
) -> u32 {
    let (current_coord, current_steps) = current;
    if visited.contains(&current_coord) {
        return 0;
    }
    visited.insert(current_coord);

    if &current_coord == target {
        return current_steps;
    }

    // let Coord { x, y } = current_coord;
    let mut max_visited = 0;
    for (neighbor, steps) in graph.get(&current_coord).unwrap() {
        max_visited = max_visited.max(dfs_p2(
            graph,
            (*neighbor, steps + current_steps),
            visited.clone(),
            target,
        ))
    }

    max_visited
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input).unwrap();

    let start = Coord { x: 0, y: 1 };
    let mut graph: HashMap<Coord, Vec<(Coord, u32)>> = HashMap::from([(start, Vec::new())]);
    map_to_graph(
        (start, 1),
        (start, 1),
        &mut graph,
        &map,
        &mut HashSet::new(),
    );

    Some(
        dfs_p2(
            &graph,
            (start, 1),
            HashSet::new(),
            &Coord {
                x: map.len() - 1,
                y: map[0].len() - 2,
            },
        ) - 1,
    )
    // Some(dfs_p1(&map, Coord { x: 0, y: 1 }, HashSet::new(), false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
