advent_of_code::solution!(2);

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    game_num: u32,
    rounds: Vec<Round>,
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) =
        separated_list1(tag(", "), separated_pair(complete::u32, tag(" "), alpha1))(input)?;

    let mut round = Round {
        red: 0,
        blue: 0,
        green: 0,
    };
    for c in &cubes {
        match c.1 {
            "red" => round.red = c.0,
            "blue" => round.blue = c.0,
            "green" => round.green = c.0,
            _ => (),
        }
    }

    Ok((input, round))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_num) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), parse_round)(input)?;

    Ok((input, Game { game_num, rounds }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;

    Ok((input, games))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_input(input).unwrap();
    let config = Round {
        red: 12,
        blue: 14,
        green: 13,
    };

    Some(
        games
            .iter()
            .map(|g| {
                for r in &g.rounds {
                    if r.red > config.red || r.blue > config.blue || r.green > config.green {
                        return 0;
                    }
                }

                g.game_num
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = parse_input(input).unwrap();

    Some(
        games
            .iter()
            .map(|g| {
                let max_red = g.rounds.iter().max_by_key(|r| r.red).unwrap().red;
                let max_blue = g.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;
                let max_green = g.rounds.iter().max_by_key(|r| r.green).unwrap().green;

                max_red * max_blue * max_green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
