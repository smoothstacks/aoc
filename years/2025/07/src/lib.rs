use aoc_util::grid::{Grid, Position, Vector};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const X: Vector = Vector::new(1, 0);

fn count_splits(grid: &Grid<char>, from: Position) -> usize {
    fn recurse(grid: &Grid<char>, from: Position, seen: &mut HashSet<Position>) -> usize {
        let mut below = grid.iter_direction(from, Vector::new(0, 1));
        match below.find(|(_, c)| **c == '^') {
            Some((position, _)) => {
                if seen.contains(&position) {
                    return 0;
                }
                seen.insert(position);

                1 + recurse(grid, position - X, seen) + recurse(grid, position + X, seen)
            }
            None => 0,
        }
    }

    let mut seen = HashSet::new();
    recurse(grid, from, &mut seen)
}

fn count_timelines(grid: &Grid<char>, from: Position) -> usize {
    fn recurse(grid: &Grid<char>, from: Position, cache: &mut HashMap<Position, usize>) -> usize {
        let mut below = grid.iter_direction(from, Vector::new(0, 1));
        match below.find(|(_, c)| **c == '^') {
            Some((position, _)) => {
                if cache.contains_key(&position) {
                    return cache[&position];
                }

                let value = recurse(grid, position - X, cache) + recurse(grid, position + X, cache);
                cache.insert(position, value);
                value
            }
            None => 1,
        }
    }

    let mut seen = HashMap::new();
    recurse(grid, from, &mut seen)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let grid = Grid::<char>::from_str(input)?;
    let start = grid
        .position_of(&'S')
        .ok_or(eyre::format_err!("no start found"))?;
    Ok(count_splits(&grid, start))
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let grid = Grid::<char>::from_str(input)?;
    let start = grid
        .position_of(&'S')
        .ok_or(eyre::format_err!("no start found"))?;
    Ok(count_timelines(&grid, start))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 21);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 40);
        Ok(())
    }
}
