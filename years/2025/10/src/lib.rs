use aoc_util::parse::nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, space1, u16, usize},
    multi::{many1, separated_list1},
    sequence::delimited,
};
use itertools::Itertools;
use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct LightDiagram {
    count: usize,
    state: u16,
}
impl From<Vec<bool>> for LightDiagram {
    fn from(value: Vec<bool>) -> Self {
        let count = value.len();
        assert!(count < 16);
        let mut state = 0;
        for (i, v) in value.into_iter().enumerate() {
            if v {
                state |= 1 << i;
            }
        }
        Self { count, state }
    }
}
impl Display for LightDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(18);
        out.push('[');
        let mut state = self.state;
        for _ in 0..self.count {
            if (state & 1) > 0 {
                out.push('#')
            } else {
                out.push('.')
            }

            state >>= 1;
        }
        out.push(']');
        write!(f, "{}", out)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct WiringSchematic(u16);
impl From<Vec<usize>> for WiringSchematic {
    fn from(value: Vec<usize>) -> Self {
        let mut out = 0;
        for v in value {
            assert!(v < 16);
            out |= 1 << v;
        }
        Self(out)
    }
}
impl Display for WiringSchematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut numbers = Vec::with_capacity(self.0.count_ones() as usize);
        for i in 0..16 {
            if (self.0 >> i) & 1 >= 1 {
                numbers.push(i.to_string());
            }
        }
        write!(f, "({})", numbers.join(","))
    }
}

#[derive(Default, Debug)]
struct Machine {
    light_diagram: LightDiagram,
    wiring_schematics: Vec<WiringSchematic>,
    _joltage_requirement: Vec<u16>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut schematics = String::new();
        for schematic in &self.wiring_schematics {
            schematics.push_str(&format!(" {schematic}"));
        }
        write!(f, "{}{}", self.light_diagram, schematics,)?;
        Ok(())
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    let parse_state = delimited(
        tag("["),
        many1(alt((char('.'), char('#'))).map(|c| c == '#')),
        tag("] "),
    )
    .map(LightDiagram::from);
    let parse_schematics = separated_list1(
        space1,
        delimited(tag("("), separated_list1(char(','), usize), tag(")")).map(WiringSchematic::from),
    );
    let parse_joltage = delimited(tag(" {"), separated_list1(char(','), u16), tag("}"));

    let parse_machine = parse_state.and(parse_schematics).and(parse_joltage).map(
        |((light_diagram, wiring_schematics), _joltage_requirement)| Machine {
            light_diagram,
            wiring_schematics,
            _joltage_requirement,
        },
    );

    separated_list1(newline, parse_machine).parse(input)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let (_, machines) = parse(input).expect("parse works");
    let mut sum = 0;
    for machine in machines {
        for i in 0.. {
            // search all permutations of this length for one that xors to the answer
            let mut options = machine.wiring_schematics.iter().combinations(i);
            let option = options.find(|v| {
                let folded = v.into_iter().fold(0, |a, b| b.0 ^ a);
                folded == machine.light_diagram.state
            });
            if let Some(_) = option {
                sum += i;
                break;
            }
        }
    }
    Ok(sum)
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 7);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }

    #[test]
    fn xor_works() {
        assert_eq!(
            vec![0b1000, 0b1010, 0b0100].iter().fold(0i32, |s, v| s ^ v),
            0b0110
        );
        assert_eq!(
            vec![0b1010, 0b1100, 0b0011, 0b0011]
                .iter()
                .fold(0i32, |s, v| s ^ v),
            0b0110
        );
    }
}
