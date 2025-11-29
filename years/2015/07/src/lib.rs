use std::collections::HashMap;

use aoc_util::parse::{
    nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, alphanumeric1},
        combinator::map,
        sequence::separated_pair,
        IResult, Parser,
    },
    parse_num,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_more::Display)]
enum Command<'a> {
    #[display("SET {_0}")]
    Set(Value<'a>),
    #[display("{_0} AND {_1}")]
    And(Value<'a>, Value<'a>),
    #[display("{_0} OR {_1}")]
    Or(Value<'a>, Value<'a>),
    #[display("{_0} LSHIFT {_1}")]
    LShift(Value<'a>, u8),
    #[display("{_0} RSHIFT {_1}")]
    RShift(Value<'a>, u8),
    #[display("NOT {_0}")]
    Not(Value<'a>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_more::Display)]
enum Value<'a> {
    Ref(&'a str),
    Number(u16),
}

fn parse_command(input: &str) -> IResult<&str, Command<'_>> {
    let parse_value = || {
        alt((
            map(alpha1, |s| Value::Ref(s)),
            map(parse_num, |v| Value::Number(v)),
        ))
    };

    alt((
        map(
            separated_pair(parse_value(), tag(" AND "), parse_value()),
            |(a, b)| Command::And(a, b),
        ),
        map(
            separated_pair(parse_value(), tag(" OR "), parse_value()),
            |(a, b)| Command::Or(a, b),
        ),
        map(
            separated_pair(parse_value(), tag(" LSHIFT "), parse_num),
            |(a, b)| Command::LShift(a, b),
        ),
        map(
            separated_pair(parse_value(), tag(" RSHIFT "), parse_num),
            |(a, b)| Command::RShift(a, b),
        ),
        map(tag("NOT ").and(parse_value()), |(_, v)| Command::Not(v)),
        map(parse_value(), |v| Command::Set(v)),
    ))
    .parse(input)
}
fn parse_instruction(input: &str) -> IResult<&str, (Command<'_>, &str)> {
    separated_pair(parse_command, tag(" -> "), alphanumeric1).parse(input)
}

fn resolve<'a>(
    value: &Value<'a>,
    instructions: &HashMap<&'a str, Command<'a>>,
    state: &mut HashMap<&'a str, u16>,
) -> Result<u16, eyre::Error> {
    match value {
        Value::Ref(w) => calculate_wire(instructions, state, w),
        Value::Number(value) => Ok(*value),
    }
}

fn calculate_wire<'a>(
    instructions: &HashMap<&'a str, Command<'a>>,
    state: &mut HashMap<&'a str, u16>,
    wire: &'a str,
) -> eyre::Result<u16> {
    if let Some(value) = state.get(wire) {
        return Ok(*value);
    }

    let command = instructions
        .get(wire)
        .ok_or(eyre::format_err!("no wire: {wire}"))?;

    let value = match command {
        Command::Set(value) => resolve(value, &instructions, state)?,
        Command::And(left, right) => {
            let left = resolve(left, &instructions, state)?;
            let right = resolve(right, &instructions, state)?;
            left & right
        }
        Command::Or(left, right) => {
            let left = resolve(left, &instructions, state)?;
            let right = resolve(right, &instructions, state)?;
            left | right
        }
        Command::LShift(value, amount) => {
            let value = resolve(value, &instructions, state)?;
            value << amount
        }
        Command::RShift(value, amount) => {
            let value = resolve(value, &instructions, state)?;
            value >> amount
        }
        Command::Not(value) => {
            let value = resolve(value, &instructions, state)?;
            !value
        }
    };

    state.insert(wire, value);

    Ok(value)
}

fn calculate_wires<'a>(input: &'a str, state: &mut HashMap<&'a str, u16>) -> eyre::Result<()> {
    let instructions = input
        .lines()
        .filter_map(|line| {
            parse_instruction(line)
                .map(|(_, (command, wire))| (wire, command))
                .ok()
        })
        .collect::<HashMap<&str, Command<'_>>>();

    for wire in instructions.keys().copied() {
        calculate_wire(&instructions, state, wire)?;
    }

    Ok(())
}

pub fn part1(input: &str) -> eyre::Result<u16> {
    let mut wires = HashMap::new();
    calculate_wires(input, &mut wires)?;
    Ok(wires["a"])
}

pub fn part2(input: &str) -> eyre::Result<u16> {
    let mut wires = HashMap::new();
    let a = part1(input)?;

    wires.insert("b", a);
    calculate_wires(input, &mut wires)?;

    Ok(wires["a"])
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "123 -> x
456 -> y
x AND y -> a
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 72);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 72);
        Ok(())
    }
}
