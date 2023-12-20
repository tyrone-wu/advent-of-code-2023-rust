use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{self, alpha1, newline},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(19);

#[derive(Clone, Copy, Debug)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Clone, Copy, Debug)]
struct Rule<'a> {
    part: usize,
    condition: Condition,
    value: u64,
    next: &'a str,
}

#[derive(Clone, Copy, Debug)]
enum Workflow<'a> {
    Accept,
    Reject,
    Rules(Rule<'a>),
    Next(&'a str),
}

#[derive(Debug)]
struct Xmas([u64; 4]);

fn parse_xmas(input: &str) -> IResult<&str, Xmas> {
    let (input, x) = preceded(tag("x="), complete::u64)(input)?;
    let (input, m) = preceded(tag(",m="), complete::u64)(input)?;
    let (input, a) = preceded(tag(",a="), complete::u64)(input)?;
    let (input, s) = preceded(tag(",s="), complete::u64)(input)?;
    Ok((input, Xmas([x, m, a, s])))
}

fn parse_xmas_list(input: &str) -> IResult<&str, Vec<Xmas>> {
    separated_list1(newline, delimited(tag("{"), parse_xmas, tag("}")))(input)
}

fn parse_condition(input: &str) -> IResult<&str, Rule> {
    let (input, part) = alt((
        complete::char('x').map(|_| 0),
        complete::char('m').map(|_| 1),
        complete::char('a').map(|_| 2),
        complete::char('s').map(|_| 3),
    ))(input)?;
    let (input, condition) = alt((
        complete::char('<').map(|_| Condition::LessThan),
        complete::char('>').map(|_| Condition::GreaterThan),
    ))(input)?;
    let (input, value) = complete::u64(input)?;
    let (input, next) = preceded(tag(":"), alpha1)(input)?;

    Ok((
        input,
        Rule {
            part,
            condition,
            value,
            next,
        },
    ))
}

fn parse_workflow(input: &str) -> IResult<&str, (&str, Vec<Workflow>)> {
    let (input, name) = alpha1(input)?;
    let (input, rules) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            alt((
                complete::char('A').map(|_| Workflow::Accept),
                complete::char('R').map(|_| Workflow::Reject),
                parse_condition.map(|c| Workflow::Rules(c)),
                is_not("AR}").map(|next| Workflow::Next(next)),
            )),
        ),
        tag("}\n"),
    )(input)?;

    Ok((input, (name, rules)))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Vec<Workflow>>> {
    let (input, mut workflows) = fold_many1(parse_workflow, HashMap::new, |mut acc, (k, v)| {
        acc.insert(k, v);
        acc
    })(input)?;
    workflows.insert("A", vec![Workflow::Accept]);
    workflows.insert("R", vec![Workflow::Reject]);
    Ok((input, workflows))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Vec<Workflow>>, Vec<Xmas>)> {
    separated_pair(parse_workflows, tag("\n"), parse_xmas_list)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (workflow_rules, xmas_list)) = parse_input(input).unwrap();

    Some(
        xmas_list
            .iter()
            .filter_map(|xmas| {
                let mut current_flow = Workflow::Next("in");
                while let Workflow::Next(name) = current_flow {
                    let workflows = workflow_rules.get(&name).unwrap();

                    for w in workflows {
                        match w {
                            Workflow::Rules(rule) => {
                                let pass = match rule.condition {
                                    Condition::LessThan => xmas.0[rule.part] < rule.value,
                                    Condition::GreaterThan => xmas.0[rule.part] > rule.value,
                                };

                                if pass {
                                    current_flow = Workflow::Next(rule.next);
                                    break;
                                }
                            }
                            _ => current_flow = *w,
                        }
                    }
                }

                match current_flow {
                    Workflow::Accept => Some(xmas.0.iter().sum::<u64>()),
                    _ => None,
                }
            })
            .sum(),
    )
}

fn process_workflow(
    ranges: &mut [(u64, u64); 4],
    workflow: &Workflow,
    workflow_rules: &HashMap<&str, Vec<Workflow>>,
) -> u64 {
    match workflow {
        Workflow::Accept => ranges.iter().map(|(min, max)| max - min + 1).product(),
        Workflow::Reject => 0,
        Workflow::Rules(rule) => {
            let Rule {
                part,
                condition,
                value,
                next,
            } = rule;

            let (left, right) = ranges[*part];
            let (pass, fail) = match condition {
                Condition::LessThan => ((left, *value - 1), (*value, right)),
                Condition::GreaterThan => ((*value + 1, right), (left, *value)),
            };

            let mut total = 0;
            if pass.0 <= pass.1 {
                let mut pass_ranges = ranges.clone();
                pass_ranges[*part] = pass;
                total += process_workflow(&mut pass_ranges, &Workflow::Next(next), workflow_rules);
            }
            if fail.0 <= fail.1 {
                ranges[*part] = fail;
            }
            total
        }
        Workflow::Next(next) => {
            let rules = workflow_rules.get(next).unwrap();
            rules
                .iter()
                .map(|rule| process_workflow(ranges, rule, workflow_rules))
                .sum()
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (workflow_rules, _)) = parse_input(input).unwrap();
    let mut xmas_ranges: [(u64, u64); 4] = [(1, 4000); 4];
    Some(process_workflow(
        &mut xmas_ranges,
        &Workflow::Next("in"),
        &workflow_rules,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
