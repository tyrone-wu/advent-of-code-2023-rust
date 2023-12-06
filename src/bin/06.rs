use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

advent_of_code::solution!(6);

fn parse_input_part_one(input: &str) -> IResult<&str, Vec<RaceRecord>> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;
    let (input, times) = separated_list1(space1, complete::u64)(input)?;
    let (input, _) = tuple((newline, tag("Distance:"), space1))(input)?;
    let (input, distances) = separated_list1(space1, complete::u64)(input)?;

    Ok((
        input,
        times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| RaceRecord {
                time: *t,
                distance: *d,
            })
            .collect(),
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, races) = parse_input_part_one(input).unwrap();

    Some(
        races
            .iter()
            .map(|r| {
                let mut lose_count: u64 = 0;
                for h in 0..=r.time {
                    let distance = h * (r.time - h);
                    if distance <= r.distance {
                        lose_count += 1;
                    }
                }

                r.time - lose_count + 1
            })
            .product(),
    )
}

fn parse_input_part_two(input: &str) -> IResult<&str, RaceRecord> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;
    let (input, times) = separated_list1(space1, digit1)(input)?;
    let (input, _) = tuple((newline, tag("Distance:"), space1))(input)?;
    let (input, distances) = separated_list1(space1, digit1)(input)?;

    let actual_time = times
        .iter()
        .map(|t| t.chars())
        .flatten()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let actual_distance = distances
        .iter()
        .map(|d| d.chars())
        .flatten()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    Ok((
        input,
        RaceRecord {
            time: actual_time,
            distance: actual_distance,
        },
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, race) = parse_input_part_two(input).unwrap();

    let mut lose_count: u64 = 0;
    for h in 0..=race.time {
        let distance = h * (race.time - h);
        if distance <= race.distance {
            lose_count += 1;
        }
    }

    Some(race.time - lose_count + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
