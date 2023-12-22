use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::{self, newline},
    multi::{many0, many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(12);

fn parse_groups(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, groups) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, groups.iter().map(|c| *c as usize).collect()))
}

fn parse_springs(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, springs) = take_till1(|c| c == ' ')(input)?;
    Ok((input, springs.bytes().collect()))
}

fn parse_spring_groups(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = many0(tag("."))(input)?;
    if input.is_empty() {
        return Ok((input, Vec::with_capacity(0)));
    }
    let (input, spring_groups) = separated_list1(many1(tag(".")), take_till1(|c| c == '.'))(input)?;
    Ok((input, spring_groups.iter().map(|s| s.to_string()).collect()))
}

fn parse_condition_record(input: &str) -> IResult<&str, (Vec<u8>, Vec<usize>)> {
    let (input, (springs, groups)) = separated_pair(parse_springs, tag(" "), parse_groups)(input)?;
    Ok((input, (springs, groups)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<u8>, Vec<usize>)>> {
    separated_list1(newline, parse_condition_record)(input)
}

fn generate_permutation(springs: Vec<u8>, springs_perm: &mut Vec<String>, mut i: usize) {
    while i < springs.len() && springs[i] != b'?' {
        i += 1;
    }
    if i >= springs.len() {
        springs_perm.push(String::from_utf8(springs).unwrap());
        return;
    }

    let mut operational = springs.clone();
    operational[i] = b'.';
    generate_permutation(operational, springs_perm, i);

    let mut damaged = springs;
    damaged[i] = b'#';
    generate_permutation(damaged, springs_perm, i);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input_records) = parse_input(input).unwrap();

    let condition_records: Vec<(Vec<String>, &Vec<usize>)> = input_records
        .iter()
        .map(|(springs, groupings)| {
            let (_, spring_groups) =
                parse_spring_groups(&String::from_utf8(springs.to_vec()).unwrap()).unwrap();
            (spring_groups, groupings)
        })
        .collect();

    let mut cache: HashMap<String, Vec<Vec<usize>>> = HashMap::new();

    // Some(
    //     input_records
    //         .iter()
    //         .map(|(original_springs, original_groupings)| {
    //             let unknown_count = original_springs
    //                 .iter()
    //                 .filter(|spring| spring == &&b'?')
    //                 .count();
    //             let mut permutations: Vec<String> =
    //                 Vec::with_capacity(2_usize.pow(unknown_count as u32));
    //             generate_permutation(original_springs.to_vec(), &mut permutations, 0);

    //             permutations
    //                 .iter()
    //                 .filter(|permutation| {
    //                     let (_, permutation_groups) = parse_spring_groups(permutation).unwrap();
    //                     let damaged_count: Vec<usize> =
    //                         permutation_groups.iter().map(|s| s.len()).collect();
    //                     original_groupings == &damaged_count
    //                 })
    //                 .count()
    //         })
    //         .sum::<usize>() as u32,
    // )

    Some(
        condition_records
            .iter()
            .map(|(original_springs, original_groupings)| {
                let possible_spring_groups: Vec<Vec<Vec<usize>>> = original_springs
                    .iter()
                    .map(|springs| {
                        if !cache.contains_key(springs) {
                            let mut permutations: Vec<String> = Vec::new();
                            generate_permutation(springs.bytes().collect(), &mut permutations, 0);

                            let possible_groups_count: Vec<Vec<usize>> = permutations
                                .iter()
                                .map(|permutation| {
                                    let (_, permutation_groups) =
                                        parse_spring_groups(permutation).unwrap();
                                    permutation_groups.iter().map(|s| s.len()).collect()
                                })
                                .collect();

                            cache.insert(springs.to_string(), possible_groups_count);
                        }
                        cache.get(springs).unwrap().clone()
                    })
                    .collect();

                let mut arranged_groups: Vec<Vec<usize>> = possible_spring_groups[0].clone();
                for psg in possible_spring_groups.iter().skip(1) {
                    let mut arranged_groups_extended: Vec<Vec<usize>> = Vec::new();
                    for ag in &arranged_groups {
                        if ag.len() > original_groupings.len() {
                            continue;
                        }

                        for g in psg {
                            let mut ag_e = ag.clone();
                            ag_e.extend(g.iter());
                            arranged_groups_extended.push(ag_e);
                        }
                    }
                    arranged_groups = arranged_groups_extended;
                }

                arranged_groups
                    .iter()
                    .filter(|ag| original_groupings == ag)
                    .count()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut _input_records) = parse_input(input).unwrap();

    // for (springs, groupings) in input_records.iter_mut() {
    //     let mut unfold_spring = springs.clone();
    //     unfold_spring.insert(0, b'?');
    //     springs.extend(unfold_spring.iter().cycle().take(4 * unfold_spring.len()));

    //     groupings.extend(groupings.clone().iter().cycle().take(4 * groupings.len()));
    // }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
