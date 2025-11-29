use std::fmt::Debug;

use aoc_util::glam;
use itertools::{iproduct, Itertools};

#[derive(Clone)]
struct Grid {
    entries: Vec<Vec<bool>>,
}
impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.entries {
            for cell in row {
                write!(f, "{}", if *cell { '#' } else { '.' })?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Grid {
    fn get(&self, i: glam::U16Vec2) -> Option<bool> {
        self.entries.get(i.y as usize)?.get(i.x as usize).copied()
    }
    fn get_mut(&mut self, i: glam::U16Vec2) -> Option<&mut bool> {
        self.entries.get_mut(i.y as usize)?.get_mut(i.x as usize)
    }

    fn count(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|e| e.iter())
            .filter(|b| **b)
            .count()
    }

    fn dimensions(&self) -> (u16, u16) {
        (
            self.entries.first().map(|v| v.len()).unwrap_or_default() as u16,
            self.entries.len() as u16,
        )
    }
}

fn parse(input: &str) -> Grid {
    let entries = input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    Grid { entries }
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut grid = parse(input);

    let (width, height) = grid.dimensions();

    let mut current = &mut grid.clone();
    let mut next = &mut grid;
    for _ in 0..100 {
        for y in 0..height {
            for x in 0..width {
                let pos = glam::u16vec2(x, y);
                let around = iproduct!(-1..=1, -1..=1)
                    .map(|(x, y)| glam::i16vec2(x, y))
                    .filter(|x| *x != glam::I16Vec2::ZERO)
                    .collect_vec();
                let count = around
                    .into_iter()
                    .filter_map(|delta| current.get(pos.checked_add_signed(delta)?).filter(|b| *b))
                    .count();

                let state = current.get(pos).expect("out of range");

                *next.get_mut(pos).expect("out of range") = match state {
                    true => count == 2 || count == 3,
                    false => count == 3,
                }
            }
        }

        std::mem::swap(&mut current, &mut next);
    }

    Ok(current.count())
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 4);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
