#![feature(new_range_api)]
use std::range::RangeInclusive;

use aoc_util::parse::nom::{
    bytes::complete::tag,
    character::{
        char,
        complete::{newline, u64},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let ranges = separated_list1(
        newline,
        separated_pair(u64, char('-'), u64).map(|(l, r)| (l..=r).into()),
    );
    let ingredients = separated_list1(newline, u64);

    separated_pair(ranges, tag("\n\n"), ingredients).parse(input)
}

fn is_fresh(ranges: &Vec<RangeInclusive<u64>>, value: u64) -> bool {
    ranges.iter().any(|r| r.contains(&value))
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let (_, (ranges, ingredients)) = parse(input).expect("should parse");
    Ok(ingredients
        .into_iter()
        .filter(|i| is_fresh(&ranges, *i))
        .count())
}
pub fn part2(input: &str) -> eyre::Result<u64> {
    let (_, (mut ranges, _)) = parse(input).expect("should parse");

    ranges.sort_by_key(|r| r.start);
    ranges.reverse();

    // collapse down ranges so overlapping ranges become single ranges
    let mut collapsed = Vec::with_capacity(ranges.len());

    'outer: while let Some(range) = ranges.pop() {
        if range.last < range.start {
            continue;
        }

        if collapsed.is_empty() {
            collapsed.push(range);
        } else {
            let mut i = 0;
            loop {
                if i == collapsed.len() {
                    break;
                }

                let compare = &mut collapsed[i];

                if compare.start <= range.last && range.start <= compare.last {
                    // overlap - so merge
                    compare.start = compare.start.min(range.start);
                    compare.last = compare.last.max(range.last);
                    continue 'outer;
                }

                i += 1;
            }

            collapsed.push(range);
        }
    }

    Ok(collapsed
        .into_iter()
        .map(|r| (r.last - r.start) + 1)
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 3);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 14);
        Ok(())
    }
}
