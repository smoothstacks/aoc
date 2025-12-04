use aoc_util::grid::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn part1(input: &str) -> eyre::Result<usize> {
    let grid = Grid::from_str(input)?;
    Ok(path_length(grid))
}
pub fn part2(_input: &str) -> eyre::Result<u32> {
    Ok(0)
}

fn path_length(input: Grid<char>) -> usize {
    let mut unique = HashSet::new();

    // map from position to direction
    let mut history = HashMap::new();

    let mut position = input
        .position_of(&'^')
        .expect("failed to find start position");
    let mut direction = Vector::new(0, -1);

    unique.insert(position);
    history.insert(position, direction);

    loop {
        let next = position + direction;
        let Some(c) = input.get(next) else {
            break;
        };

        match c {
            '.' | '^' => {
                position = next;
                unique.insert(position);
            }
            '#' => {
                // change direction
                direction = Vector::new(-direction.y, direction.x);
                continue;
            }
            _ => unreachable!(),
        }
    }

    unique.len()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 41);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
