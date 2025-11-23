pub fn part1(_: &str) -> eyre::Result<u32> {
    Ok(0)
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "TEST INPUT";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 0);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
