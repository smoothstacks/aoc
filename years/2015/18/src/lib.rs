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

fn run(current: &Grid, next: &mut Grid, corners: bool) {
    let (width, height) = current.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pos = glam::u16vec2(x, y);
            let neighbours = iproduct!(-1..=1, -1..=1)
                .map(|(x, y)| glam::i16vec2(x, y))
                .filter(|x| *x != glam::I16Vec2::ZERO)
                .filter_map(|delta| {
                    let neighbour = pos.checked_add_signed(delta)?;
                    current.get(neighbour).map(|_| neighbour)
                });

            let is_corner = neighbours.clone().count() == 3;
            let lit_neighbours = neighbours
                .clone()
                .filter_map(|p| current.get(p).filter(|b| *b))
                .count();

            let state = current.get(pos).expect("out of range");

            let lit = if corners && is_corner {
                true
            } else {
                match state {
                    true => lit_neighbours == 2 || lit_neighbours == 3,
                    false => lit_neighbours == 3,
                }
            };

            *next.get_mut(pos).expect("out of range") = lit;
        }
    }
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let mut grid = parse(input);

    let mut current = &mut grid.clone();
    let mut next = &mut grid;

    #[cfg(not(test))]
    let range = 0..100;
    #[cfg(test)]
    let range = 0..4;

    for _ in range {
        run(current, next, false);
        std::mem::swap(&mut current, &mut next);
    }

    Ok(current.count())
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let mut grid = parse(input);

    let mut current = &mut grid.clone();
    let mut next = &mut grid;

    #[cfg(not(test))]
    let range = 0..100;
    #[cfg(test)]
    let range = 0..5;

    for _ in range {
        run(current, next, true);
        std::mem::swap(&mut current, &mut next);
    }

    Ok(current.count())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_works() -> eyre::Result<()> {
        const INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        assert_eq!(super::part1(INPUT)?, 4);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        const INPUT: &str = "##.#.#
...##.
#....#
..#...
#.#..#
####.#";

        assert_eq!(super::part2(INPUT)?, 17);
        Ok(())
    }
}
