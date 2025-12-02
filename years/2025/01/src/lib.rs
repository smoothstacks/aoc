use aoc_util::parse::nom::{
    branch::alt,
    character::complete::{char, newline, u16},
    multi::separated_list1,
    IResult, Parser,
};

fn parse(input: &str) -> IResult<&str, Vec<i16>> {
    separated_list1(
        newline,
        (
            alt((char('R').map(|_| 1i16), char('L').map(|_| -1i16))),
            u16,
        )
            .map(|(a, b)| a * b as i16),
    )
    .parse(input)
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    let (_, turns) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    let mut dial = 50i16;
    let mut password = 0;
    for turn in turns {
        dial = (turn + dial).rem_euclid(100);
        if dial == 0 {
            password += 1;
        }
    }

    Ok(password)
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let (_, turns) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    let mut dial = 50i16;
    let mut password = 0;
    for turn in turns {
        let full_turns = (turn / 100).abs() as u32;
        password += full_turns;

        let next = dial + turn % 100;
        if dial != 0 && (next <= 0 || next > 99) {
            password += 1;
        }

        dial = next.rem_euclid(100);
    }

    Ok(password)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!((-1i32).rem_euclid(100), 99);
        assert_eq!(super::part1(INPUT)?, 3);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 6);
        Ok(())
    }
}
