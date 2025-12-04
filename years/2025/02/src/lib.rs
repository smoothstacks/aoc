use aoc_util::{
    math::num_digits,
    parse::nom::{
        character::complete::{char, usize},
        multi::separated_list1,
        sequence::separated_pair,
        IResult, Parser,
    },
};
use itertools::Itertools;
use std::{num::NonZero, ops::RangeInclusive};

fn parse(input: &str) -> IResult<&str, Vec<RangeInclusive<usize>>> {
    separated_list1(
        char(','),
        separated_pair(usize, char('-'), usize).map(|(a, b)| a..=b),
    )
    .parse(input)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    fn is_invalid_id(id: usize) -> bool {
        let num_digits = num_digits(id);
        if num_digits % 2 == 1 {
            false
        } else {
            let mul = 10usize.pow(num_digits as u32 / 2);
            let left = id / mul;
            let right = id - (left * mul);

            left == right
        }
    }

    let (_, pairs) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;
    Ok(pairs
        .into_iter()
        .flat_map(|r| r)
        .filter(|u| is_invalid_id(*u))
        .sum::<usize>())
}

pub fn part2(input: &str) -> eyre::Result<usize> {
    fn digit_chunks(mut n: usize, width: NonZero<u32>) -> impl Iterator<Item = usize> {
        let pow = 10usize.pow(width.get() as u32);
        std::iter::from_fn(move || {
            let window = n % pow;
            n /= pow;
            if n > 0 || window > 0 {
                Some(window)
            } else {
                None
            }
        })
    }

    fn is_invalid_id(id: usize) -> bool {
        let num_digits = num_digits(id) as u32;
        (1..=num_digits / 2).any(|width| {
            num_digits % width == 0
                && digit_chunks(id, NonZero::new(width).unwrap())
                    .unique()
                    .count()
                    == 1
        })
    }

    let (_, pairs) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;
    Ok(pairs
        .into_iter()
        .flat_map(|r| r)
        .filter(|u| is_invalid_id(*u))
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 1227775554);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 4174379265);
        Ok(())
    }
}
