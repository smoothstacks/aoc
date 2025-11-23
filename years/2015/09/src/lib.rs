use std::collections::HashSet;

use aoc_util::parse::nom::{
    bytes::tag,
    character::complete::{alpha1, newline, u16},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use petgraph::prelude::UnGraphMap;

fn parse(input: &str) -> IResult<&str, UnGraphMap<&str, u16>> {
    let cities = separated_pair(alpha1, tag(" to "), alpha1);
    let route = map(separated_pair(cities, tag(" = "), u16), |((l, r), w)| {
        (l, r, w)
    });
    map(separated_list1(newline, route), UnGraphMap::from_iter).parse(input)
}

fn path_length(graph: &UnGraphMap<&str, u16>, min_or_max: bool) -> eyre::Result<u16> {
    let mut result = if min_or_max { u16::MAX } else { 0 };
    let mut visited = HashSet::with_capacity(graph.node_count());

    for mut node in graph.nodes() {
        visited.clear();
        visited.insert(node);

        let mut total = 0;

        loop {
            let next_edge = {
                let unseen = graph.edges(node).filter(|v| !visited.contains(v.1));
                if min_or_max {
                    unseen.min_by_key(|a| a.2)
                } else {
                    unseen.max_by_key(|a| a.2)
                }
            };

            if let Some(edge) = next_edge {
                assert_eq!(edge.0, node);

                node = edge.1;
                total += edge.2;

                visited.insert(node);
            } else {
                break;
            }
        }

        if visited.len() == graph.node_count() {
            result = if min_or_max {
                result.min(total)
            } else {
                result.max(total)
            }
        }
    }

    Ok(result)
}

pub fn part1(input: &str) -> eyre::Result<u16> {
    let (_, graph) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;
    path_length(&graph, true)
}
pub fn part2(input: &str) -> eyre::Result<u16> {
    let (_, graph) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;
    path_length(&graph, false)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 605);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 982);
        Ok(())
    }
}
