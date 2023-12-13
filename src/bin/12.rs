use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{self, newline},
    multi::{many0, many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(12);

#[derive(Debug)]
struct ConditionRecord {
    springs: Vec<u8>,
    groups: Vec<usize>,
}

fn parse_groups(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, groups) = separated_list1(tag(","), complete::u32)(input)?;
    Ok((input, groups.iter().map(|c| *c as usize).collect()))
}

fn parse_condition_record(input: &str) -> IResult<&str, ConditionRecord> {
    let (input, (springs, groups)) = separated_pair(
        many1(alt((tag("."), tag("#"), tag("?")))),
        tag(" "),
        parse_groups,
    )(input)?;
    Ok((
        input,
        ConditionRecord {
            springs: springs.iter().map(|s| s.bytes().next().unwrap()).collect(),
            groups,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ConditionRecord>> {
    separated_list1(newline, parse_condition_record)(input)
}

fn gen_permutation(springs: Vec<u8>, springs_perm: &mut Vec<String>, mut i: usize) {
    while i < springs.len() && springs[i] != b'?' {
        i += 1;
    }
    if i >= springs.len() {
        springs_perm.push(String::from_utf8(springs).unwrap());
        return;
    }

    let mut operational = springs.clone();
    operational[i] = b'.';
    gen_permutation(operational, springs_perm, i);

    let mut damaged = springs;
    damaged[i] = b'#';
    gen_permutation(damaged, springs_perm, i);
}

fn parse_spring_groups(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = many0(tag("."))(input)?;
    if input.is_empty() {
        return Ok((input, Vec::with_capacity(0)));
    }
    separated_list1(many1(tag(".")), take_till1(|c| c == '.'))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cond_records) = parse_input(input).unwrap();

    Some(
        cond_records
            .iter()
            .map(|record: &ConditionRecord| {
                let ConditionRecord {
                    springs: orig_springs,
                    groups,
                } = &record;

                let q_count = orig_springs
                    .iter()
                    .filter(|spring| spring == &&b'?')
                    .count();
                let mut permutations: Vec<String> = Vec::with_capacity(2_usize.pow(q_count as u32));
                gen_permutation(orig_springs.to_vec(), &mut permutations, 0);

                permutations
                    .iter()
                    .filter(|permutation| {
                        let permutation_groups = parse_spring_groups(permutation).unwrap().1;
                        let spring_perm_count: Vec<usize> =
                            permutation_groups.iter().map(|s| s.len()).collect();
                        groups == &spring_perm_count
                    })
                    .count()
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut cond_records) = parse_input(input).unwrap();
    for r in cond_records.iter_mut() {
        let mut unfold_spring = r.springs.clone();
        unfold_spring.insert(0, b'?');
        r.springs
            .extend(unfold_spring.iter().cycle().take(4 * unfold_spring.len()));

        r.groups
            .extend(r.groups.clone().iter().cycle().take(4 * r.groups.len()));
    }

    let mut cache: HashMap<&str, Vec<Vec<usize>>> = HashMap::new();

    Some(
        cond_records
            .iter()
            .map(|record: &ConditionRecord| {
                cache.clear();
                let ConditionRecord {
                    springs: orig_springs,
                    groups,
                } = &record;

                let spring_groups: Vec<&str> =
                    parse_spring_groups(std::str::from_utf8(&orig_springs).unwrap())
                        .unwrap()
                        .1;
                // let possible_spring_groups_record: Vec<Vec<Vec<usize>>> = spring_groups
                //     .iter()
                //     .map(|springs| {
                //         if !cache.contains_key(springs) {
                //             let mut permutations: Vec<String> = Vec::new();
                //             gen_permutation(springs.bytes().collect(), &mut permutations, 0);

                //             let possible_groups_count: Vec<Vec<usize>> = permutations
                //                 .iter()
                //                 .map(|permutation| {
                //                     let permutation_groups =
                //                         parse_spring_groups(permutation).unwrap().1;
                //                     permutation_groups.iter().map(|s| s.len()).collect()
                //                 })
                //                 .collect();

                //             cache.insert(&springs, possible_groups_count);
                //         }
                //         cache.get(springs).unwrap().clone()
                //     })
                //     .collect();

                // let mut arranged_groups: Vec<Vec<usize>> = possible_spring_groups_record[0].clone();
                // for psg in possible_spring_groups_record.iter().skip(1) {
                //     let mut arranged_groups_extended: Vec<Vec<usize>> = Vec::new();
                //     for ag in &arranged_groups {
                //         for a in psg {
                //             let mut ag_e = ag.clone();
                //             ag_e.extend(a.iter());
                //             arranged_groups_extended.push(ag_e);
                //         }
                //     }
                //     arranged_groups = arranged_groups_extended;
                // }

                // let count = arranged_groups.iter().filter(|ag| groups == *ag).count();
                // count

                1
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
