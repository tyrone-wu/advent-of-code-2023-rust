use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(9);

fn parse_history(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reading) = separated_list1(tag(" "), complete::i32)(input)?;
    Ok((input, vec![reading]))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Vec<i32>>>> {
    separated_list1(newline, parse_history)(input)
}

fn solve(input: &str, part_one: bool) -> i32 {
    let (_, mut sensor_readings) = parse_input(input).unwrap();

    sensor_readings
        .iter_mut()
        .map(|r| {
            let mut can_extrapolate = false;
            while !can_extrapolate {
                let mut steps: Vec<i32> = Vec::new();
                let level = &r[r.len() - 1];
                can_extrapolate = true;

                for i in 0..(level.len() - 1) {
                    let step = level[i + 1] - level[i];
                    steps.push(step);
                    if step != 0 {
                        can_extrapolate = false;
                    }
                }
                r.push(steps);
            }

            if part_one {
                r.iter().map(|s| s.last().unwrap()).sum::<i32>()
            } else {
                r.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i % 2 == 0 {
                            *s.first().unwrap()
                        } else {
                            s.first().unwrap() * -1
                        }
                    })
                    .sum::<i32>()
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input, true))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
