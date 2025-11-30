use std::collections::{HashMap, HashSet};

use aoc_util::parse::nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

type Mapping<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> IResult<&str, (&str, Mapping<'_>)> {
    let (remain, (mappings, molecule)) = separated_pair(
        separated_list1(newline, separated_pair(alpha1, tag(" => "), alpha1)),
        tag("\n\n"),
        alpha1,
    )
    .parse(input)?;

    let mut mapping = HashMap::new();
    for (key, value) in mappings {
        mapping
            .entry(key)
            .and_modify(|l: &mut Vec<&str>| l.push(value))
            .or_insert(vec![value]);
    }

    Ok((remain, (molecule, mapping)))
}

fn next_molecules<'a>(
    molecule: &'a str,
    replace: &'a str,
    with: &'a str,
) -> impl Iterator<Item = String> + use<'a> {
    molecule
        .match_indices(replace)
        .scan(molecule, |molecule, (n, _)| {
            let mut result = String::with_capacity(molecule.len() + with.len());
            result.push_str(&molecule[..n]);
            result.push_str(with);
            result.push_str(&molecule[n + replace.len()..]);
            Some(result)
        })
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let (_, (medicine, mapping)) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    let mut distinct_molecules = HashSet::new();

    for (to_replace, replace_with) in mapping.iter().flat_map(|(k, v)| v.iter().map(|v| (*k, *v))) {
        distinct_molecules.extend(next_molecules(medicine, to_replace, replace_with));
    }

    Ok(distinct_molecules.len())
}

pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::next_molecules;

    const INPUT: &str = "H => HO
H => OH
O => HH

HOH";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 4);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }

    #[test]
    fn next_molecules_works() {
        let next = next_molecules("a|b|a|b|a|b", "a", "c").collect_vec();
        assert_eq!(next, vec!["c|b|a|b|a|b", "a|b|c|b|a|b", "a|b|a|b|c|b"]);
    }
}
