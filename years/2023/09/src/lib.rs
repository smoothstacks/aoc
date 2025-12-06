mod parse {
    pub fn parse(input: &str) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| str::parse::<i64>(s).expect("integer values"))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn diffs(itr: &[i64]) -> Vec<i64> {
    itr.windows(2).map(|window| window[1] - window[0]).collect()
}

fn next(itr: &[i64]) -> i64 {
    if itr.iter().all(|s| s == &0) {
        return 0;
    }

    let diffs = diffs(itr);
    let next_diff = next(&diffs);
    itr.last().unwrap() + next_diff
}

fn prev(itr: &[i64]) -> i64 {
    if itr.iter().all(|s| s == &0) {
        return 0;
    }

    let diffs = diffs(itr);
    let prev_diff = prev(&diffs);
    itr.first().unwrap() - prev_diff
}

pub fn part1(input: &str) -> eyre::Result<i64> {
    let sequences = parse::parse(input);

    let mut sum = 0;
    for sequence in sequences {
        sum += next(&sequence);
    }

    Ok(sum)
}

pub fn part2(input: &str) -> eyre::Result<i64> {
    let sequences = parse::parse(input);

    let mut sum = 0;
    for sequence in sequences {
        sum += prev(&sequence);
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const INPUT_2: &str =
        "9 5 1 -3 -7 -11 -15 -19 -23 -27 -31 -35 -39 -43 -47 -51 -55 -59 -63 -67 -71";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 114);
        assert_eq!(super::part1(INPUT_2)?, -75);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 2);
        Ok(())
    }
}
