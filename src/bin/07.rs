use std::collections::BTreeMap;

use nom::{
    character::complete::{self, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

// Implicitly orders from top as least to bottom as greatest
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Card {
    JPartTwo,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    JPartOne,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: Option<HandType>,
}

fn parse_hand_part_one(input: &str) -> IResult<&str, Hand> {
    let (input, (cards_raw, bid)) = separated_pair(alphanumeric1, space1, complete::u32)(input)?;
    let cards: Vec<Card> = cards_raw
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::JPartOne,
            b'T' => Card::T,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            _ => unreachable!(),
        })
        .collect();

    Ok((
        input,
        Hand {
            cards,
            bid,
            hand_type: None,
        },
    ))
}

fn parse_hand_part_two(input: &str) -> IResult<&str, Hand> {
    let (input, (cards_raw, bid)) = separated_pair(alphanumeric1, space1, complete::u32)(input)?;
    let cards: Vec<Card> = cards_raw
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::JPartTwo,
            b'T' => Card::T,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            _ => unreachable!(),
        })
        .collect();

    Ok((
        input,
        Hand {
            cards,
            bid,
            hand_type: None,
        },
    ))
}

fn parse_input(input: &str, part_one: bool) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(
        newline,
        if part_one {
            parse_hand_part_one
        } else {
            parse_hand_part_two
        },
    )(input)?;
    Ok((input, hands))
}

fn solve(input: &str, part_one: bool) -> u32 {
    let (_, mut hands) = parse_input(input, part_one).unwrap();

    for h in hands.iter_mut() {
        let mut card_freq: Vec<u8> = if part_one {
            h.cards
                .iter()
                .fold(BTreeMap::new(), |mut freq, c| {
                    *freq.entry(c).or_default() += 1;
                    freq
                })
                .into_values()
                .collect()
        } else {
            let mut card_freq_map: BTreeMap<&Card, u8> =
                h.cards.iter().fold(BTreeMap::new(), |mut freq, c| {
                    *freq.entry(c).or_default() += 1;
                    freq
                });
            if card_freq_map.len() != 1 {
                let joker_count = card_freq_map.remove(&Card::JPartTwo).unwrap_or_default();
                card_freq_map
                    .iter_mut()
                    .max_by(|(_, x), (_, y)| x.cmp(y))
                    .map(|(_, v)| *v += joker_count);
            }

            card_freq_map.into_values().collect()
        };
        card_freq.sort();
        card_freq.reverse();

        if card_freq.len() == 1 {
            h.hand_type = Some(HandType::FiveOfKind);
        } else if card_freq.len() == 2 && card_freq[0] == 4 && card_freq[1] == 1 {
            h.hand_type = Some(HandType::FourOfKind);
        } else if card_freq.len() == 2 && card_freq[0] == 3 && card_freq[1] == 2 {
            h.hand_type = Some(HandType::FullHouse);
        } else if card_freq.len() == 3 && card_freq[0] == 3 && card_freq[1] == 1 {
            h.hand_type = Some(HandType::ThreeOfKind);
        } else if card_freq.len() == 3 && card_freq[0] == 2 && card_freq[1] == 2 {
            h.hand_type = Some(HandType::TwoPair);
        } else if card_freq.len() == 4 {
            h.hand_type = Some(HandType::OnePair);
        } else {
            h.hand_type = Some(HandType::HighCard);
        }
    }

    hands.sort_by(|a, b| {
        (a.hand_type.as_ref().unwrap(), &a.cards).cmp(&(b.hand_type.as_ref().unwrap(), &b.cards))
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
