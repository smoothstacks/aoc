pub fn search(input: &str, prefix: &str) -> eyre::Result<u32> {
    (0..)
        .find(|n| {
            let input = format!("{}{n}", input.trim());
            let s = md5::compute(input);
            format!("{s:x}").starts_with(prefix)
        })
        .ok_or(eyre::Error::msg("none found"))
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    search(input, "00000")
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    search(input, "000000")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "abcdef";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 609043);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 6742839);
        Ok(())
    }
}
