use std::collections::HashSet;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn expand_rows(image: &mut Vec<Vec<u8>>, gap: usize) -> Vec<Vec<u8>> {
    let mut expanded_image = Vec::with_capacity(image.len() * 2);
    while let Some(row) = image.pop() {
        if !row.contains(&b'#') {
            for _ in 1..gap {
                expanded_image.push(row.clone());
            }
        }
        expanded_image.push(row);
    }

    expanded_image
}

fn transpose(image: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = image.len();
    let cols = image[0].len();

    (0..cols)
        .map(|c| (0..rows).map(|r| image[r][c]).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut image = parse_input(input);

    let mut expanded_image: Vec<Vec<u8>> = expand_rows(&mut image, 2);
    expanded_image = transpose(&mut expanded_image);
    expanded_image = expand_rows(&mut expanded_image, 2);

    let rows = expanded_image[0].len();
    let galaxies: Vec<(usize, usize)> = expanded_image
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, space)| {
            if space == &b'.' {
                None
            } else {
                Some((i / rows, i % rows))
            }
        })
        .collect();

    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(i, s1)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|s2| s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1))
                    .sum::<usize>()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let image = parse_input(input);

    let col_size = image[0].len();
    let galaxies: Vec<(usize, usize)> = image
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, space)| {
            if space == &b'.' {
                None
            } else {
                Some((i / col_size, i % col_size))
            }
        })
        .collect();

    let empty_rows: HashSet<usize> = image
        .iter()
        .enumerate()
        .filter_map(|(i, row)| if row.contains(&b'#') { None } else { Some(i) })
        .fold(HashSet::with_capacity(image.len()), |mut acc, idx| {
            acc.insert(idx);
            acc
        });
    let empty_cols: HashSet<usize> = (0..col_size)
        .filter_map(|j| {
            for i in 0..image.len() {
                if image[i][j] == b'#' {
                    return None;
                }
            }
            Some(j)
        })
        .fold(HashSet::with_capacity(image.len()), |mut acc, idx| {
            acc.insert(idx);
            acc
        });

    let gap: usize = 1000000;
    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(i, s1)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|s2| {
                        ((s1.0.min(s2.0))..(s1.0.max(s2.0)))
                            .map(|step| if empty_rows.contains(&step) { gap } else { 1 })
                            .sum::<usize>()
                            + ((s1.1.min(s2.1))..(s1.1.max(s2.1)))
                                .map(|step| if empty_cols.contains(&step) { gap } else { 1 })
                                .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
