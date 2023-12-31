use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{fold_many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};
use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::UnGraph, stable_graph::NodeIndex, Graph, Undirected},
};

advent_of_code::solution!(25);

fn parse_component(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, component) = alpha1(input)?;
    let (input, connections) = delimited(
        tag(": "),
        separated_list1(tag(" "), alpha1.map(|s: &str| s.to_string())),
        newline,
    )(input)?;
    Ok((input, (component.to_string(), connections)))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, HashSet<String>>> {
    fold_many1(
        parse_component,
        HashMap::<String, HashSet<String>>::new,
        |mut acc, (component, connections)| {
            for c in &connections {
                let conn = acc.entry(c.clone()).or_insert(HashSet::new());
                conn.insert(component.clone());
            }

            let conn = acc.entry(component).or_insert(HashSet::new());
            conn.extend(connections);

            acc
        },
    )(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, components) = parse_input(input).unwrap();

    let mut nodes: HashMap<&String, NodeIndex> = HashMap::new();
    let mut graph: Graph<String, u8, Undirected> = UnGraph::new_undirected();
    for c in components.keys() {
        let node_idx = graph.add_node(c.clone());
        nodes.insert(c, node_idx);
    }
    for (a, con) in &components {
        let a_nidx = nodes.get(a).unwrap();
        for b in con {
            let b_nidx = nodes.get(b).unwrap();
            if graph.contains_edge(*a_nidx, *b_nidx) {
                continue;
            }
            graph.add_edge(*a_nidx, *b_nidx, 1);
        }
    }

    let stoer_wagner: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (min_cut, partition) = stoer_wagner.unwrap().unwrap();
    if min_cut == 3 {
        Some(partition.len() * (nodes.len() - partition.len()))
    } else {
        None
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {}
}
