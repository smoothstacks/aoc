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

fn calculate_jolts(bank: &[u8]) -> u8 {
    let mut left = (0, bank[0]);
    let mut right = (1, bank[1]);
    let mut best = left.1 * 10 + right.1;
    loop {
        // look for right onwards for the max
        for i in right.0 + 1..bank.len() {
            if bank[i] >= right.1 {
                right = (i, bank[i]);
            }
        }
        // now search up to right for left
        for i in 0..right.0 {
            if bank[i] >= left.1 {
                left = (i, bank[i]);
            }
        }

        best = (left.1 * 10 + right.1).max(best);

        if right.0 == bank.len() - 1 {
            break;
        }

        left = right;
        right = (right.0 + 1, bank[right.0 + 1]);
    }

    best
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    let banks = parse(input);
    Ok(banks.iter().map(|bank| calculate_jolts(&bank) as u32).sum())
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 357);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
