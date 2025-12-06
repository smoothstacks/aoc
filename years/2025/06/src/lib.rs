use aoc_util::{grid::Grid, math::num_from_iter};
use itertools::Itertools;

#[derive(Debug)]
struct Operation {
    numbers: Grid<Option<u8>>,
    operation: char,
}

impl Operation {
    fn human_result(&self) -> u64 {
        let dimensions = self.numbers.get_dimensions();

        let numbers = (0..dimensions.height).map(|row: usize| -> u64 {
            let digits = self.numbers.row_iter(row).filter_map(|u| *u).rev();
            num_from_iter(digits)
        });

        match self.operation {
            '*' => numbers.product(),
            '+' => numbers.sum(),
            _ => unreachable!(),
        }
    }

    fn cephalopod_result(&self) -> u64 {
        let dimensions = self.numbers.get_dimensions();

        let numbers = (0..dimensions.width).map(|column| -> u64 {
            let digits = self.numbers.column_iter(column).filter_map(|u| *u).rev();
            num_from_iter(digits)
        });

        match self.operation {
            '*' => numbers.product(),
            '+' => numbers.sum(),
            _ => unreachable!(),
        }
    }
}

fn parse(mut input: &str) -> Vec<Operation> {
    input = input.trim_matches('\n');

    // operation is always left-aligned, so we can rely on
    // the gaps between them to determine the column widths
    let last_line_start = input.rfind('\n').expect("should have more than one line");
    let mut operations_line = &input[last_line_start + 1..];

    let is_operation = |c| c == '+' || c == '*';

    let mut cursor = operations_line.len();
    let mut columns = Vec::new();

    while let Some(p) = operations_line.rfind(is_operation) {
        let operation = operations_line[p..]
            .chars()
            .next()
            .expect("should have an operation");
        let col_width = cursor - p;

        columns.push((operation, col_width));

        // skip the spaces in between
        cursor = p.saturating_sub(1);
        operations_line = &operations_line[..cursor];
    }
    columns.reverse();

    let mut operands = vec![Grid::<Option<u8>>::empty(); columns.len()];

    let numbers_lines = &input[..last_line_start];
    // now we want to iterate over each numbers line
    for line in numbers_lines.lines() {
        // cursor to will be at the start of each column
        let mut cursor = 0;

        for (i, (_, width)) in columns.iter().enumerate() {
            let cells = line[cursor..cursor + width]
                .chars()
                .map(|c| c.to_digit(10).map(|u| u as u8))
                .collect_vec();

            assert_eq!(cells.len(), *width);

            operands[i]
                .add_row(cells)
                .expect("column width should be consistent");

            cursor += width + 1;
        }
    }

    columns
        .into_iter()
        .zip(operands)
        .into_iter()
        .map(|((operation, _), numbers)| Operation { numbers, operation })
        .collect_vec()
}

pub fn part1(input: &str) -> eyre::Result<u64> {
    let operations = parse(input);
    Ok(operations.iter().map(Operation::human_result).sum())
}
pub fn part2(input: &str) -> eyre::Result<u64> {
    let operations = parse(input);
    Ok(operations.iter().map(Operation::cephalopod_result).sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 4277556);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 3263827);
        Ok(())
    }
}
