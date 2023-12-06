mod parse {
    pub const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn word_digit(input: &str) -> Option<u32> {
        DIGITS
            .iter()
            .enumerate()
            .find_map(|(i, s)| input.starts_with(s).then_some((i + 1) as u32))
    }
    fn numeric_digit(input: &str) -> Option<u32> {
        input.chars().nth(0).and_then(|c| c.to_digit(10))
    }
    pub fn calibration_digits<'a>(mut input: &'a str) -> impl Iterator<Item = u32> + 'a {
        std::iter::from_fn(move || {
            while input.len() > 0 {
                let res = word_digit(input).or(numeric_digit(input));
                if let Some(res) = res {
                    input = &input[1..];
                    return Some(res);
                }
                input = &input[1..];
            }

            None
        })
    }
}

pub fn part1(input: &str) -> u32 {
    fn get_calibration_value<'a>(line: &'a str) -> u32 {
        let mut itr = line.chars().filter_map(|c| c.to_digit(10));
        let tens = itr.next().unwrap_or_default();
        let ones = itr.last().unwrap_or(tens);

        (tens * 10) + ones
    }

    input.lines().map(get_calibration_value).sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut itr = parse::calibration_digits(line);
            let first = itr.nth(0).unwrap();
            let last = itr.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(super::part1(INPUT), 142);
    }

    #[test]
    fn part2_works() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(super::part2(INPUT), 281);
    }

    #[test]
    fn parse_works() {
        const INPUT: &str = "twofour36";
        let v: Vec<_> = super::parse::calibration_digits(INPUT).collect();
        assert_eq!(v, vec![2, 4, 3, 6]);
    }
}
