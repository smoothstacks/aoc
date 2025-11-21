use itertools::Itertools;

#[derive(Debug, derive_more::Deref)]
struct Dimensions([u32; 3]);
impl Dimensions {
    fn new(mut lengths: [u32; 3]) -> Self {
        lengths.sort();
        Self(lengths)
    }
    fn sides(&self) -> impl Iterator<Item = u32> + use<'_> {
        self.0.iter().circular_tuple_windows().map(|(l, r)| l * r)
    }
    fn perimeters(&self) -> impl Iterator<Item = u32> + use<'_> {
        self.0.iter().combinations(2).map(|v| 2 * v[0] + 2 * v[1])
    }
    fn volume(&self) -> u32 {
        self.0.iter().product()
    }
    fn parse(input: &str) -> Self {
        let sizes = input
            .split("x")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect_vec();

        assert!(sizes.len() == 3);

        Dimensions::new(sizes.try_into().unwrap())
    }
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    Ok(input
        .lines()
        .map(Dimensions::parse)
        .map(|p| p.sides().map(|s| 2 * s).sum::<u32>() + p.sides().min().unwrap())
        .sum())
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    Ok(input
        .lines()
        .map(Dimensions::parse)
        .map(|p| p.perimeters().min().unwrap() + p.volume())
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2x3x4";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 58);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 34);
        Ok(())
    }
}
