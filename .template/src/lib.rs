mod parse {
    use nom::IResult;

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, ()> {
        Ok((input, ()))
    }
}

pub fn part1(input: &str) -> u32 {
    0
}
pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "TEST INPUT";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 0);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
