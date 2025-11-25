use aoc_util::parse::{
    nom::{
        branch::alt,
        bytes::tag,
        character::{
            char,
            complete::{alpha1, newline},
        },
        multi::separated_list1,
        sequence::{preceded, terminated},
        Parser,
    },
    parse_num,
};
use itertools::Itertools;
use petgraph::prelude::DiGraphMap;

fn parse(input: &str) -> eyre::Result<DiGraphMap<&str, i16>> {
    let parse_potential = (
        alpha1,
        tag(" would "),
        alt((
            preceded(tag("gain "), parse_num),
            preceded(tag("lose "), parse_num).map(|e: i16| -e),
        )),
        tag(" happiness units by sitting next to "),
        terminated(alpha1, char('.')),
    )
        .map(|(source, _, happiness, _, target)| (source, target, happiness));

    separated_list1(newline, parse_potential)
        .parse(input)
        .map(|e| DiGraphMap::from_edges(e.1))
        .map_err(|e| eyre::format_err!("parse error: {e}"))
}

fn max_happiness(graph: &DiGraphMap<&str, i16>) -> i16 {
    graph
        .nodes()
        .permutations(graph.node_count())
        .map(|arrangement| {
            arrangement
                .iter()
                .circular_tuple_windows()
                .fold(0, |happiness, (a, b)| {
                    happiness
                        + graph.edge_weight(a, b).copied().unwrap_or_default()
                        + graph.edge_weight(b, a).copied().unwrap_or_default()
                })
        })
        .max()
        .unwrap_or_default()
}

pub fn part1(input: &str) -> eyre::Result<i16> {
    let graph = parse(input)?;
    Ok(max_happiness(&graph))
}
pub fn part2(input: &str) -> eyre::Result<i16> {
    const ME: &str = "Me";

    let mut graph = parse(input)?;
    for node in graph.nodes().collect_vec() {
        graph.add_edge(ME, node, 0);
        graph.add_edge(node, ME, 0);
    }

    Ok(max_happiness(&graph))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 330);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 286);
        Ok(())
    }
}
