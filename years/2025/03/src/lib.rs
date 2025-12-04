use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|u| u as u8))
                .collect_vec()
        })
        .collect_vec()
}

fn jolts(bank: &[u8], batteries: usize) -> Option<u64> {
    if batteries > bank.len() {
        return None;
    }
    if batteries == 0 {
        return Some(0);
    }

    // find the highest digit in the right-most position
    for d in (0..=9).rev() {
        for (index, digit) in bank.iter().enumerate() {
            if digit != &d {
                continue;
            }

            if let Some(next) = jolts(&bank[index + 1..], batteries - 1) {
                return Some(*digit as u64 * 10u64.pow(batteries as u32 - 1) + next);
            }
        }
    }

    None
}

pub fn part1(input: &str) -> eyre::Result<u64> {
    let banks = parse(input);
    Ok(banks.iter().filter_map(|bank| jolts(&bank, 2)).sum())
}
pub fn part2(input: &str) -> eyre::Result<u64> {
    let banks = parse(input);
    Ok(banks.iter().filter_map(|bank| jolts(&bank, 12)).sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "811111111111119
987654321111111
234234234234278
818181911112111";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 357);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 3121910778619);
        Ok(())
    }
}
