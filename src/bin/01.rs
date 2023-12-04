advent_of_code::solution!(1);

fn parse_digit(line_bytes: &[u8], i: usize, part_one: bool) -> Option<u32> {
    if line_bytes[i].is_ascii_digit() {
        return Some((line_bytes[i] - b'0') as u32);
    } else if !part_one {
        if i + 5 <= line_bytes.len() {
            match std::str::from_utf8(&line_bytes[i..(i + 5)]).unwrap() {
                "three" => return Some(3),
                "seven" => return Some(7),
                "eight" => return Some(8),
                _ => (),
            }
        }
        if i + 4 <= line_bytes.len() {
            match std::str::from_utf8(&line_bytes[i..(i + 4)]).unwrap() {
                "four" => return Some(4),
                "five" => return Some(5),
                "nine" => return Some(9),
                _ => (),
            }
        }
        if i + 3 <= line_bytes.len() {
            match std::str::from_utf8(&line_bytes[i..(i + 3)]).unwrap() {
                "one" => return Some(1),
                "two" => return Some(2),
                "six" => return Some(6),
                _ => (),
            }
        }
    }
    None
}

fn solve(input: &str, part_one: bool) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                let mut num = 0;

                for i in 0..line.len() {
                    if let Some(digit) = parse_digit(line, i, part_one) {
                        num = digit * 10;
                        break;
                    }
                }

                for i in (0..line.len()).rev() {
                    if let Some(digit) = parse_digit(line, i, part_one) {
                        num += digit;
                        break;
                    }
                }

                num
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
