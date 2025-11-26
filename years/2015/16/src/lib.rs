use std::{collections::HashMap, sync::LazyLock};

use aoc_util::parse::{
    nom::{
        bytes::complete::tag,
        character::complete::{alpha1, newline},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult, Parser,
    },
    parse_num,
};

static TAPE: LazyLock<HashMap<&str, u16>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ])
});

struct AuntSue<'a> {
    id: u16,
    things: HashMap<&'a str, u16>,
}

fn parse_sue(input: &str) -> IResult<&str, AuntSue<'_>> {
    separated_pair(
        preceded(tag("Sue "), parse_num),
        tag(": "),
        separated_list1(tag(", "), separated_pair(alpha1, tag(": "), parse_num)),
    )
    .map(|(id, things)| AuntSue {
        id,
        things: HashMap::from_iter(things),
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<AuntSue<'_>>> {
    separated_list1(newline, parse_sue).parse(input)
}

pub fn part1(input: &str) -> eyre::Result<u16> {
    let (_, aunts) = parse(input).map_err(|e| eyre::format_err!("{e:?}"))?;

    aunts
        .iter()
        .find(|aunt| {
            aunt.things
                .iter()
                .all(|(thing, amount)| TAPE.get(thing).is_some_and(|a| a == amount))
        })
        .map(|aunt| aunt.id)
        .ok_or(eyre::format_err!("no aunt found"))
}
pub fn part2(input: &str) -> eyre::Result<u16> {
    let (_, aunts) = parse(input).map_err(|e| eyre::format_err!("{e:?}"))?;

    aunts
        .iter()
        .find(|aunt| {
            aunt.things.iter().all(|(thing, amount)| {
                TAPE.get(thing).is_some_and(|a| match *thing {
                    "cats" | "trees" => amount > a,
                    "pomeranians" | "goldfish" => amount < a,
                    _ => amount == a,
                })
            })
        })
        .map(|aunt| aunt.id)
        .ok_or(eyre::format_err!("no aunt found"))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Sue 100: abc: 1, cde: 2\nSue 101: abc: 1, cde: 2\n";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 0);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }

    #[test]
    fn parse_works() -> eyre::Result<()> {
        super::parse(INPUT)?;
        Ok(())
    }
}
