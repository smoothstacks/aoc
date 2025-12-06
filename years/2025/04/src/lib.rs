use std::str::FromStr;

use aoc_util::{euclid, grid::*};
use itertools::{iproduct, Itertools};

fn accessible(grid: &Grid<char>) -> impl Iterator<Item = Position> + use<'_> {
    grid.find_all(&'@').filter(|p| {
        let positions = iproduct!(-1..=1, -1..=1)
            .filter(|p| p.0 != 0 || p.1 != 0)
            .map(|(x, y)| euclid::vec2(x, y));

        grid.iter_pattern(*p, positions)
            .filter(|(_, value)| **value == '@')
            .count()
            < 4
    })
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let grid = Grid::from_str(input)?;
    Ok(accessible(&grid).count())
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let mut grid = Grid::from_str(input)?;
    let mut total = 0;
    loop {
        let accessible = accessible(&grid).collect_vec();
        if accessible.len() == 0 {
            break;
        }

        for p in accessible {
            if let Some(c) = grid.get_mut(p) {
                total += 1;
                *c = '.';
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 13);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 43);
        Ok(())
    }
}
