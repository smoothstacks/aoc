use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn part1(input: &'static str) -> eyre::Result<u64> {
    let (_, equations) = parse(input)?;

    Ok(equations
        .iter()
        .filter(|e| is_solvable(e, false))
        .map(|e| e.result)
        .sum())
}
pub fn part2(input: &'static str) -> eyre::Result<u64> {
    let (_, equations) = parse(input)?;
    Ok(equations
        .iter()
        .filter(|e| is_solvable(e, true))
        .map(|e| e.result)
        .sum())
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    use aoc_util::parse::parse_num;
    separated_list1(
        newline,
        map(
            separated_pair(parse_num, tag(": "), separated_list1(space1, parse_num)),
            |(result, operands)| Equation { result, operands },
        ),
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::FromRepr)]
#[repr(usize)]
enum Operator {
    Add = 0,
    Mul,
    Concat,
}
impl Operator {
    fn all(count: usize, include_concat: bool) -> Vec<Vec<Self>> {
        let max = if include_concat { 3 } else { 2 };
        (0..count)
            .map(|_| (0..max))
            .multi_cartesian_product()
            .map(|v| {
                v.into_iter()
                    .map(|r| Operator::from_repr(r).unwrap())
                    .collect_vec()
            })
            .collect_vec()
    }
}

fn is_solvable(equation: &Equation, include_concat: bool) -> bool {
    Operator::all(equation.operands.len() - 1, include_concat)
        .into_iter()
        .any(|operators| {
            let result = equation.operands.iter().skip(1).enumerate().fold(
                equation.operands[0],
                |acc, (i, operand)| {
                    let op = operators[i];
                    match op {
                        Operator::Add => acc + *operand,
                        Operator::Mul => acc * *operand,
                        Operator::Concat => (acc * 10u64.pow(operand.ilog10() + 1)) + operand,
                    }
                },
            );

            result == equation.result
        })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 3749);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 11387);
        Ok(())
    }
}
