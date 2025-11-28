use aoc_util::parse::nom::{
    character::complete::{newline, u8},
    multi::separated_list1,
    IResult, Parser,
};
use itertools::Itertools;

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(newline, u8.map(|i| i as u32)).parse(input)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let (_, containers) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    Ok((2..=containers.len())
        .map(|count| {
            containers
                .iter()
                .combinations(count)
                .filter(|c| c.iter().copied().sum::<u32>() == 150)
                .count()
        })
        .sum())
}

pub fn part2(input: &str) -> eyre::Result<usize> {
    let (_, containers) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    for i in 2..=containers.len() {
        let valid_combinations = containers
            .iter()
            .combinations(i)
            .filter(|c| c.iter().copied().sum::<u32>() == 150)
            .count();

        if valid_combinations > 0 {
            return Ok(valid_combinations);
        }
    }

    eyre::bail!("no combination found");
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "140
10
5";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 1);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 1);
        Ok(())
    }
}
