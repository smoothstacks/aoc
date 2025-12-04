use aoc_util::grid::*;
use std::{collections::HashSet, str::FromStr};

fn height_at(grid: &Grid<char>, p: Position) -> Option<u32> {
    grid.get(p).and_then(|c| c.to_digit(10))
}

fn valid_directions<'a>(
    grid: &'a Grid<char>,
    pos: Position,
    current: u32,
) -> impl Iterator<Item = Position> + 'a {
    const DIRECTIONS: [Vector; 4] = [
        Vector::new(0, 1),
        Vector::new(1, 0),
        Vector::new(0, -1),
        Vector::new(-1, 0),
    ];

    DIRECTIONS
        .iter()
        .filter_map(move |dir| {
            let next = height_at(grid, pos + *dir);
            match next {
                Some(n) if n.saturating_sub(current) == 1 => Some(dir),
                _ => None,
            }
        })
        .copied()
}

fn find_unique_peaks(grid: &Grid<char>, from: Position, found: &mut HashSet<Position>) {
    let current = height_at(grid, from);

    match current {
        Some(9) => {
            found.insert(from);
        }
        Some(n) => {
            for dir in valid_directions(grid, from, n) {
                find_unique_peaks(grid, from + dir, found);
            }
        }
        _ => {}
    };
}

fn find_unique_trails(grid: &Grid<char>, from: Position) -> u32 {
    let current = height_at(grid, from);
    match current {
        Some(9) => 1,
        Some(n) => {
            let mut total = 0;
            for dir in valid_directions(grid, from, n) {
                total += find_unique_trails(grid, from + dir);
            }
            total
        }
        None => return 0,
    }
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    let grid = Grid::from_str(input)?;
    let start_positions = grid.find_all(&'0');

    let mut total = 0;
    for pos in start_positions {
        let mut found = HashSet::new();
        find_unique_peaks(&grid, pos, &mut found);
        total += found.len();
    }

    Ok(total as u32)
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let grid = Grid::from_str(input)?;
    let start_positions = grid.find_all(&'0');

    let mut total = 0;
    for pos in start_positions {
        total += find_unique_trails(&grid, pos);
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 36);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 81);
        Ok(())
    }
}
