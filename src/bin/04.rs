use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(4);

struct Card {
    winning_nums: HashSet<u32>,
    drawn_nums: HashSet<u32>,
    copies: u32,
}

fn parse_nums(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, nums) = separated_list1(space1, complete::u32)(input)?;
    Ok((input, HashSet::from_iter(nums)))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = preceded(tuple((tag("Card"), space1)), digit1)(input)?;
    let (input, _) = tuple((tag(":"), space1))(input)?;
    let (input, (winning_nums, drawn_nums)) =
        separated_pair(parse_nums, tuple((tag(" |"), space1)), parse_nums)(input)?;

    Ok((
        input,
        Card {
            winning_nums,
            drawn_nums,
            copies: 1,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(newline, parse_card)(input)?;
    Ok((input, cards))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = parse_input(input).unwrap();

    Some(
        cards
            .iter()
            .map(|c| {
                let num_matches = c.winning_nums.intersection(&c.drawn_nums).count() as u32;

                if num_matches == 0 {
                    0
                } else {
                    2_u32.pow(num_matches - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut cards) = parse_input(input).unwrap();

    let mut num_scratchcards = 0;
    for i in 0..cards.len() {
        num_scratchcards += cards[i].copies;
        let num_matches = cards[i]
            .winning_nums
            .intersection(&cards[i].drawn_nums)
            .count();
        for j in (i + 1)..(cards.len().min(i + 1 + num_matches)) {
            cards[j].copies += cards[i].copies;
        }
    }

    Some(num_scratchcards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
