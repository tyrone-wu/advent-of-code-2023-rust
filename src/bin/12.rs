use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

advent_of_code::solution!(12);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct ConditionRecord {
    springs: Vec<Spring>,
    groupings: Vec<usize>,
}

fn parse_condition_record(input: &str) -> IResult<&str, ConditionRecord> {
    let (input, (springs, groupings)) = separated_pair(
        many1(alt((
            complete::char('.').map(|_| Spring::Operational),
            complete::char('#').map(|_| Spring::Damaged),
            complete::char('?').map(|_| Spring::Unknown),
        ))),
        tag(" "),
        separated_list1(tag(","), complete::u32.map(|num| num as usize)),
    )(input)?;
    Ok((input, ConditionRecord { springs, groupings }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ConditionRecord>> {
    separated_list1(newline, parse_condition_record)(input)
}

// ty icub3d from yt for awesome explanation 
fn permutation(
    springs: &[Spring],
    groupings: &[usize],
    cache: &mut HashMap<ConditionRecord, u64>,
) -> u64 {
    let cond_record = ConditionRecord {
        springs: springs.to_vec(),
        groupings: groupings.to_vec(),
    };
    if let Some(&arrangements) = cache.get(&cond_record) {
        return arrangements;
    }

    if groupings.is_empty() {
        let is_valid = if springs.iter().any(|s| *s == Spring::Damaged) {
            0
        } else {
            1
        };
        cache.insert(cond_record, is_valid);
        return is_valid;
    }

    let dmg_count = springs.iter().filter(|s| *s == &Spring::Damaged).count();
    let unknown_count = springs.iter().filter(|s| *s == &Spring::Unknown).count();
    let dmg_count_target = groupings.iter().sum();
    if dmg_count + unknown_count < dmg_count_target || dmg_count > dmg_count_target {
        cache.insert(cond_record, 0);
        return 0;
    }

    if springs[0] == Spring::Operational {
        let arrangements = permutation(&springs[1..], groupings, cache);
        cache.insert(cond_record, arrangements);
        return arrangements;
    }

    let mut total_arrangements = 0;
    let group_target = groupings[0];
    let current_group_size = springs[0..group_target]
        .iter()
        .filter(|s| *s != &Spring::Operational)
        .count();
    if current_group_size == group_target
        && ((group_target < springs.len() && springs[group_target] != Spring::Damaged)
            || springs.len() == group_target)
    {
        let next = springs.len().min(group_target + 1);
        total_arrangements = permutation(&springs[next..], &groupings[1..], cache);
    }

    if springs[0] == Spring::Unknown {
        total_arrangements += permutation(&springs[1..], groupings, cache);
    }

    cache.insert(cond_record, total_arrangements);
    total_arrangements
}

fn solve(cond_records: &Vec<ConditionRecord>) -> u64 {
    let mut cache: HashMap<ConditionRecord, u64> = HashMap::new();
    cond_records
        .iter()
        .map(|ConditionRecord { springs, groupings }| permutation(springs, groupings, &mut cache))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, cond_records) = parse_input(input).unwrap();
    Some(solve(&cond_records))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut cond_records) = parse_input(input).unwrap();
    for ConditionRecord { springs, groupings } in cond_records.iter_mut() {
        let mut unfolded_part = vec![Spring::Unknown];
        unfolded_part.extend(springs.iter());
        springs.extend(unfolded_part.iter().cycle().take(4 * unfolded_part.len()));

        groupings.extend(groupings.clone().iter().cycle().take(4 * groupings.len()));
    }

    Some(solve(&cond_records))
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
