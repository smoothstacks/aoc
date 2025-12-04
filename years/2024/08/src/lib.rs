use aoc_util::euclid;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> eyre::Result<usize> {
    let map = Map::new(input);
    Ok(map.antinodes(false).count())
}
pub fn part2(input: &str) -> eyre::Result<usize> {
    let map = Map::new(input);
    Ok(map.antinodes(true).count())
}

#[derive(Debug, Clone, Copy, Hash)]
struct CoordPair(
    euclid::default::Vector2D<isize>,
    euclid::default::Vector2D<isize>,
);

struct Map<'a> {
    lines: Vec<&'a str>,
    frequencies: HashSet<char>,
}
impl<'a> Map<'a> {
    pub fn new(map: &'a str) -> Self {
        let lines = map.lines().collect_vec();
        let frequences = lines
            .iter()
            .map(|l| l.chars())
            .flatten()
            .filter(|c| *c != '.')
            .collect();
        Self {
            lines,
            frequencies: frequences,
        }
    }

    pub fn antennae(
        &self,
        frequency: char,
    ) -> impl Iterator<Item = euclid::default::Vector2D<isize>> + Clone + '_ {
        self.lines.iter().enumerate().flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == frequency {
                    Some(euclid::vec2(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
    }

    pub fn is_valid_position(
        &self,
        euclid::default::Vector2D { x, y, .. }: euclid::default::Vector2D<isize>,
    ) -> bool {
        x >= 0
            && y >= 0
            && y < self.lines.len() as isize
            && x < self.lines[y as usize].len() as isize
    }

    pub fn antinodes(
        &self,
        resonance: bool,
    ) -> impl Iterator<Item = euclid::default::Vector2D<isize>> + Clone + '_ {
        self.frequencies
            .iter()
            .flat_map(move |frequency| {
                let antennae = self.antennae(*frequency);
                antennae
                    .clone()
                    .cartesian_product(antennae)
                    .filter_map(|(a, b)| (a != b).then(|| CoordPair(a, b)))
                    .flat_map(move |CoordPair(a, b): CoordPair| {
                        let diff = b - a;

                        let mut out = Vec::with_capacity(1);
                        if resonance {
                            for i in 1.. {
                                let next = a + (diff * i);
                                if !self.is_valid_position(next) {
                                    break;
                                }
                                out.push(next);
                            }
                        } else {
                            if self.is_valid_position(a - diff) {
                                out.push(a - diff);
                            }
                        }

                        out
                    })
            })
            .unique()
    }
}

impl std::fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print = |resonance: bool| {
            let antinodes = self.antinodes(resonance).collect_vec();

            for (x, line) in self.lines.iter().enumerate() {
                for (y, c) in line.chars().enumerate() {
                    let coord = (y as isize, x as isize).into();
                    if antinodes.contains(&coord) {
                        write!(f, "#")?;
                    } else if self.frequencies.contains(&c) {
                        write!(f, "{c}")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }

            writeln!(f)
        };

        print(false)?;
        print(true)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 14);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 34);
        Ok(())
    }
}
