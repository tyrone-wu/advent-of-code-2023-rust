use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(15);

fn parse_input_p1(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), take_till1(|c| c == ',' || c == '\n'))(input)
}

fn hash(code: &str, part_one: bool) -> u32 {
    let mut hash_num = 0;
    for c in code.bytes() {
        if part_one || (c != b'=' && c != b'-') {
            hash_num = (hash_num + c as u32) * 17 % 256
        } else {
            break;
        }
    }
    hash_num

    // code.bytes()
    //     .fold(0, |acc, c| {
    //         (acc + c as u32) * 17 % 256
    //     })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, init_seq) = parse_input_p1(input).unwrap();
    Some(init_seq.iter().map(|code| hash(code, true)).sum())
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Code<'a> {
    code: &'a str,
    focal_len: Option<u32>,
}

fn parse_lens_dash(input: &str) -> IResult<&str, Code> {
    let (input, code) = terminated(alpha1, tag("-"))(input)?;
    Ok((
        input,
        Code {
            code,
            focal_len: None,
        },
    ))
}

fn parse_lens_equals(input: &str) -> IResult<&str, Code> {
    let (input, (code, focal_len)) = separated_pair(alpha1, tag("="), complete::u32)(input)?;
    Ok((
        input,
        Code {
            code,
            focal_len: Some(focal_len),
        },
    ))
}

fn parse_input_p2(input: &str) -> IResult<&str, Vec<Code>> {
    separated_list1(
        alt((tag(","), tag("\n"))),
        alt((parse_lens_equals, parse_lens_dash)),
    )(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, init_seq) = parse_input_p2(input).unwrap();

    let mut boxes: Vec<Vec<Code>> = vec![vec![]; 256];
    for code in &init_seq {
        let hash = hash(code.code, false) as usize;
        let box_ref = &mut boxes[hash];

        let box_code_idx = box_ref.iter().position(|c| c.code == code.code);
        if let Some(code_idx) = box_code_idx {
            if code.focal_len.is_some() {
                box_ref[code_idx] = *code;
            } else {
                box_ref.remove(code_idx);
            }
        } else if code.focal_len.is_some() {
            box_ref.push(*code);
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, code)| (1 + i as u32) * (1 + j as u32) * code.focal_len.unwrap())
                    .sum::<u32>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
