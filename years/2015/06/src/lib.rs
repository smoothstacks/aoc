use aoc_util::parse::nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, value},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Copy, Clone, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
#[derive(Copy, Clone, Debug)]
struct Instruction {
    action: Action,
    from: glam::USizeVec2,
    to: glam::USizeVec2,
}

fn parse_digit(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(i)
}
fn parse_coordinate(i: &str) -> IResult<&str, glam::USizeVec2> {
    let (i, (l, r)) = separated_pair(parse_digit, tag(","), parse_digit).parse(i)?;
    Ok((i, glam::USizeVec2::new(l, r)))
}
fn parse_pair(i: &str) -> IResult<&str, (glam::USizeVec2, glam::USizeVec2)> {
    separated_pair(parse_coordinate, tag(" through "), parse_coordinate).parse(i)
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, action) = alt((
            value(Action::TurnOn, tag("turn on ")),
            value(Action::TurnOff, tag("turn off ")),
            value(Action::Toggle, tag("toggle ")),
        ))
        .parse(input)?;
        let (rest, (from, to)) = parse_pair(input)?;

        Ok((rest, Self { action, from, to }))
    }
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut lights = [[false; 1000]; 1000];
    let instructions = input
        .lines()
        .filter_map(|l| Instruction::parse(l).map(|r| r.1).ok());
    for instruction in instructions {
        for row in &mut lights[instruction.from.x..=instruction.to.x] {
            for cell in &mut row[instruction.from.y..=instruction.to.y] {
                match instruction.action {
                    Action::TurnOn => *cell = true,
                    Action::TurnOff => *cell = false,
                    Action::Toggle => *cell = !*cell,
                }
            }
        }
    }

    Ok(lights.iter().flatten().filter(|v| **v).count())
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let mut lights = [[0u32; 1000]; 1000];
    let instructions = input
        .lines()
        .filter_map(|l| Instruction::parse(l).map(|r| r.1).ok());
    for instruction in instructions {
        for row in &mut lights[instruction.from.x..=instruction.to.x] {
            for cell in &mut row[instruction.from.y..=instruction.to.y] {
                match instruction.action {
                    Action::TurnOn => *cell += 1,
                    Action::TurnOff => *cell = cell.saturating_sub(1),
                    Action::Toggle => *cell += 2,
                }
            }
        }
    }

    Ok(lights.iter().flatten().sum::<u32>() as u32)
}
