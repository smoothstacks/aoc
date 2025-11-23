use aoc_util::parse::nom::{
    branch::alt,
    bytes::{complete::take, tag},
    character::{anychar, char},
    combinator::{map, map_res},
    sequence::preceded,
    IResult, Parser,
};

fn parser(input: &str) -> IResult<&str, char> {
    let hex = map_res(take(2usize), |s| u8::from_str_radix(s, 16));
    let hex_char = map(preceded(tag(r"\x"), hex), |u| u as u8 as char);
    let escaped_char = preceded(char('\\'), anychar);

    alt((char('"'), hex_char, escaped_char, anychar)).parse(input)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut total = 0;
    let mut actual = 0;

    for mut line in input.lines() {
        total += line.len();
        line = &line[1..line.len() - 1];

        while !line.is_empty() {
            line = parser(line)
                .map_err(|e| eyre::format_err!("parse error: {e}"))?
                .0;
            actual += 1;
        }
    }

    Ok(total - actual)
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#""qxfcsmh""#;

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 2);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }

    #[test]
    fn parser() -> eyre::Result<()> {
        assert_eq!(("", 0x27 as char), super::parser(r"\x27")?);
        Ok(())
    }
}
