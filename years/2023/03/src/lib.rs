use std::ops::RangeInclusive;

mod parse {
    use aoc_util::parse::nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::{anychar, digit1, newline},
        combinator::{consumed, map_res, opt},
    };

    use super::*;
    pub fn parse(mut input: &str) -> IResult<&str, Schematic> {
        let mut row = 0;
        let mut col = 0;

        let mut parts = vec![];
        let mut symbols = vec![];

        loop {
            if input.len() == 0 {
                break;
            }

            if let (next, Some((consumed, value))) =
                opt(consumed(map_res(digit1, str::parse::<u32>))).parse(input)?
            {
                parts.push(PartNumber {
                    value,
                    row,
                    columns: col..=col + consumed.len() - 1,
                });
                col += consumed.len();
                input = next;
                continue;
            }

            if let (next, Some(_)) = opt(tag(".")).parse(input)? {
                col += 1;
                input = next;
                continue;
            }

            if let (next, Some(_)) = opt(newline).parse(input)? {
                row += 1;
                col = 0;
                input = next;
                continue;
            }

            if let (next, Some(value)) = opt(anychar).parse(input)? {
                symbols.push(Symbol {
                    value,
                    row,
                    column: col,
                });
                col += 1;
                input = next;
            }
        }

        Ok((input, Schematic { parts, symbols }))
    }
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    row: usize,
    columns: RangeInclusive<usize>,
}

impl PartNumber {
    pub fn adjacent_to(&self, symbol: &Symbol) -> bool {
        let row_range = symbol.row.saturating_sub(1)..=symbol.row + 1;
        let col_range = symbol.column.saturating_sub(1)..=symbol.column + 1;

        row_range.contains(&self.row)
            && (col_range.contains(&self.columns.start())
                || col_range.contains(&self.columns.end()))
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    row: usize,
    column: usize,
}

impl Symbol {
    fn gear_ratio(&self, parts: &[PartNumber]) -> Option<u32> {
        if self.value == '*' {
            let adjacent_parts: Vec<_> =
                parts.iter().filter(|part| part.adjacent_to(self)).collect();

            (adjacent_parts.len() == 2).then_some(adjacent_parts.iter().map(|p| p.value).product())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn valid_part_numbers(&self) -> impl Iterator<Item = &PartNumber> {
        self.parts
            .iter()
            .filter(|part| self.symbols.iter().any(|symbol| part.adjacent_to(symbol)))
    }

    pub fn gear_ratio_sum(&self) -> u32 {
        self.symbols
            .iter()
            .filter_map(|symbol| symbol.gear_ratio(&self.parts))
            .sum()
    }
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    let (_, schematic) = parse::parse(input).expect("parse works");
    Ok(schematic.valid_part_numbers().map(|p| p.value).sum())
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let (_, schematic) = parse::parse(input).expect("parse works");
    Ok(schematic.gear_ratio_sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 4361);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 467835);
        Ok(())
    }

    #[test]
    fn parse_works() {
        let schematic = super::parse::parse(INPUT);
        assert!(schematic.is_ok());
    }
}
