fn next_floor(floor: isize, input: char) -> isize {
    match input {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    }
}

pub fn part1(input: &str) -> eyre::Result<isize> {
    Ok(input.chars().fold(0isize, next_floor))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let mut floor = 0;
    for (position, input) in input.chars().enumerate() {
        floor = next_floor(floor, input);
        if floor < 0 {
            return Ok(position + 1);
        }
    }

    eyre::bail!("never entered basement");
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "))(((((";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 3);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 1);
        Ok(())
    }
}
