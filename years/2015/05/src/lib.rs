use itertools::Itertools;

pub fn part1(input: &str) -> eyre::Result<usize> {
    fn is_nice(input: &&str) -> bool {
        const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
        const DISALLOWED: &[&str] = &["ab", "cd", "pq", "xy"];

        let enough_vowels = input.chars().filter(|c| VOWELS.contains(c)).count() >= 3;
        let allowed = !DISALLOWED.iter().any(|c| input.contains(c));
        let has_pair = input.chars().tuple_windows().any(|(a, b)| a == b);

        enough_vowels && allowed && has_pair
    }

    Ok(input.lines().filter(is_nice).count())
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    fn is_nice(input: &&str) -> bool {
        let mut has_non_overlapping_match = false;

        'outer: for i in 0..input.len() {
            let pair = input.get(i..=i + 1);
            for j in i + 2..input.len() {
                let other = input.get(j..=j + 1);
                if pair == other {
                    has_non_overlapping_match = true;
                    break 'outer;
                }
            }
        }

        let has_straddled_pair = input.chars().tuple_windows().any(|(a, _, b)| a == b);
        has_non_overlapping_match && has_straddled_pair
    }
    Ok(input.lines().filter(is_nice).count())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_works() -> eyre::Result<()> {
        const INPUT: &str = "ugknbfddgicrmopn";
        assert_eq!(super::part1(INPUT)?, 1);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        const INPUT: &str = "qjhvhtzxzqqjkmpb";
        assert_eq!(super::part2(INPUT)?, 1);
        Ok(())
    }
}
