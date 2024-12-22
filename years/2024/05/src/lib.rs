use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit0, newline},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

// map of page numbers -> numbers that should NOT predcede them
#[derive(Debug, derive_more::Deref, derive_more::DerefMut, PartialEq, Eq)]
struct Rules(std::collections::HashMap<u32, HashSet<u32>>);

impl FromIterator<(u32, u32)> for Rules {
    fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
        let mut out = Rules(std::collections::HashMap::new());
        for (a, b) in iter.into_iter() {
            out.entry(a)
                .and_modify(|r| {
                    r.insert(b);
                })
                .or_insert(HashSet::from_iter([b]));
        }
        out
    }
}

type Updates = Vec<Vec<u32>>;

fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(digit0, |s: &str| s.parse::<u32>())(input)
}

fn parse(input: &str) -> IResult<&str, (Rules, Updates)> {
    separated_pair(
        map(
            separated_list0(newline, separated_pair(parse_num, tag("|"), parse_num)),
            Rules::from_iter,
        ),
        tag("\n\n"),
        separated_list0(newline, separated_list0(char(','), parse_num)),
    )(input)
}

pub fn part1(input: &'static str) -> eyre::Result<u32> {
    let (_, (rules, updates)) = parse(input)?;

    Ok(updates
        .iter()
        .filter(|update| is_correct(update, &rules))
        .filter_map(|update| update.get(update.len() / 2))
        .sum())
}
pub fn part2(input: &'static str) -> eyre::Result<u32> {
    let (_, (rules, mut updates)) = parse(input)?;

    Ok(updates
        .iter_mut()
        .filter(|update| !is_correct(update, &rules))
        .map(|update| {
            fix(update, &rules);
            update
        })
        .filter_map(|update| update.get(update.len() / 2))
        .sum())
}

fn fix(update: &mut [u32], rules: &Rules) {
    while !is_correct(update, rules) {
        for i in (0..update.len()).rev() {
            if let Some(rule) = rules.get(&update[i]) {
                for p in (0..i).rev() {
                    if rule.contains(&(update[p] as u32)) {
                        update.swap(i, p);
                        break;
                    }
                }
            }
        }
    }
}

fn is_correct(update: &[u32], rules: &Rules) -> bool {
    let mut disallowed: HashSet<u32> = HashSet::new();

    for page in update.iter().rev() {
        if disallowed.contains(page) {
            return false;
        }

        if let Some(pages) = rules.get(page) {
            disallowed.extend(pages);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 143);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 123);
        Ok(())
    }
}
