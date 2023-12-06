use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Conversion {
    dst_range_start: i64,
    src_range_start: i64,
    range_length: i64,
}

#[derive(Debug)]
struct ConversionMap {
    // src: String,
    // dst: String,
    conversions: Vec<Conversion>,
    // compressed_conversion: Conversion
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<ConversionMap>,
}

fn parse_conversion(input: &str) -> IResult<&str, Conversion> {
    let (input, dst_range_start) = terminated(complete::i64, space1)(input)?;
    let (input, src_range_start) = terminated(complete::i64, space1)(input)?;
    let (input, range_length) = complete::i64(input)?;

    Ok((
        input,
        Conversion {
            dst_range_start,
            src_range_start,
            range_length,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, ConversionMap> {
    let (input, (_, _)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, conversions) = separated_list1(newline, parse_conversion)(input)?;

    Ok((
        input,
        ConversionMap {
            // src: String::from_str(src).unwrap(),
            // dst: String::from_str(dst).unwrap(),
            conversions,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, complete::i64),
        tag("\n\n"),
    )(input)?;
    let (input, maps) = separated_list1(many1(newline), parse_map)(input)?;

    Ok((input, Almanac { seeds, maps }))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, mut almanac) = parse_input(input).unwrap();
    let seeds = &mut almanac.seeds;

    Some(
        *seeds
            .iter_mut()
            .map(|s| {
                for m in &almanac.maps {
                    for c in &m.conversions {
                        if (c.src_range_start..(c.src_range_start + c.range_length)).contains(s) {
                            *s += c.dst_range_start - c.src_range_start;
                            break;
                        }
                    }
                }
                s
            })
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, almanac) = parse_input(input).unwrap();
    let seeds = almanac.seeds.chunks(2).collect::<Vec<_>>();

    // Brute force - Part 2: 125742456 (4222.8s)
    Some(
        seeds
            .iter()
            .map(|s| {
                let seed_start = s[0];
                let seed_range = s[1];
                let mut min_seed = i64::MAX;

                for i in seed_start..(seed_start + seed_range) {
                    let mut seed = i;
                    for m in &almanac.maps {
                        for c in &m.conversions {
                            if (c.src_range_start..(c.src_range_start + c.range_length))
                                .contains(&seed)
                            {
                                seed += c.dst_range_start - c.src_range_start;
                                break;
                            }
                        }
                    }
                    min_seed = min_seed.min(seed);
                }

                min_seed
            })
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
