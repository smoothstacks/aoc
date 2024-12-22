use itertools::Itertools;

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = vec![];
    let mut second = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        let left: i32 = left.parse().unwrap();
        first.push(left);

        let right: i32 = right.parse().unwrap();
        second.push(right);
    }

    (first, second)
}

pub fn part1(input: &str) -> eyre::Result<i32> {
    let (mut first, mut second) = parse_lists(input);
    first.sort();
    second.sort();

    Ok(first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>())
}
pub fn part2(input: &str) -> eyre::Result<i32> {
    let (first, second) = parse_lists(input);
    Ok(first
        .iter()
        .cartesian_product(second)
        .filter_map(|(a, b)| (a == &b).then(|| b))
        .sum::<i32>())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 11);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 31);
        Ok(())
    }
}
