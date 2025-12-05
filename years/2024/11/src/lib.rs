#![feature(linked_list_cursors)]

use std::collections::HashMap;

use aoc_util::{
    math::{num_digits, split_num_at},
    parse::nom::{
        character::complete::{space1, u64},
        multi::separated_list1,
        IResult, Parser,
    },
};

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64).parse(input)
}

fn count_descendants(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(entry) = cache.get_mut(&(stone, blinks)) {
        return *entry;
    }

    let out = match stone {
        0 => count_descendants(1, blinks - 1, cache),
        x if num_digits(x) % 2 == 0 => {
            let n = num_digits(x);
            let (left, right) = split_num_at(x, n / 2);
            count_descendants(left, blinks - 1, cache) + count_descendants(right, blinks - 1, cache)
        }
        _ => count_descendants(stone * 2024, blinks - 1, cache),
    };

    cache.insert((stone, blinks), out);
    out
}

fn stare(stones: &[u64], blinks: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|s| count_descendants(*s, blinks, &mut cache))
        .sum()
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let (_, stones) = parse(input).expect("should parse");
    Ok(stare(&stones, 25))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let (_, stones) = parse(input).expect("should parse");
    Ok(stare(&stones, 75))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "125 17";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 55312);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 65601038650482);
        Ok(())
    }
}
