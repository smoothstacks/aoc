use std::fmt::Debug;

mod parse {
    use aoc_util::parse::nom::{
        character::complete::{anychar, line_ending},
        error,
        multi::{many1, separated_list1},
        Err, IResult, Parser,
    };

    use super::*;

    impl Map {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            let (input, positions) =
                separated_list1(line_ending, many1(Tile::parse)).parse(input)?;
            Ok((input, Self { positions }))
        }
    }

    impl Tile {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            let (input, c) = anychar(input)?;
            let tile = match c {
                'S' => Tile::Start,
                '.' => Tile::Ground,
                '|' => Tile::Pipe(Pipe::Vertical),
                '-' => Tile::Pipe(Pipe::Horizontal),
                'L' => Tile::Pipe(Pipe::BendNorthEast),
                'J' => Tile::Pipe(Pipe::BendNorthWest),
                '7' => Tile::Pipe(Pipe::BendSouthWest),
                'F' => Tile::Pipe(Pipe::BendSouthEast),
                _ => return Err(Err::Error(error::make_error(input, error::ErrorKind::Char))),
            };
            Ok((input, tile))
        }
    }
}

struct Map {
    positions: Vec<Vec<Tile>>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.positions {
            for col in row {
                let c = match col {
                    Tile::Start => 'S',
                    Tile::Pipe(p) => *p as u8 as char,
                    Tile::Ground => '.',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Map {
    fn start(&self) -> (usize, usize) {
        self.positions
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, tile)| **tile == Tile::Start)
                    .map(|(x, _)| (x, y))
            })
            .unwrap()
    }

    fn tile_at(&self, pos: (usize, usize)) -> &Tile {
        &self.positions[pos.1][pos.0]
    }

    fn rows(&self) -> usize {
        self.positions.len()
    }

    fn columns(&self) -> usize {
        self.positions[0].len()
    }
}

struct LoopCursor<'a> {
    map: &'a Map,
    position: (usize, usize),
    facing: Direction,
}

impl LoopCursor<'_> {
    pub fn next(&mut self) {
        match self.map.tile_at(self.position) {
            Tile::Start => todo!(),
            Tile::Pipe(pipe) => match pipe {
                Pipe::BendNorthEast => match self.facing {
                    Direction::South => self.facing = Direction::East,
                    Direction::West => self.facing = Direction::North,
                    _ => panic!("impossible move"),
                },
                Pipe::BendNorthWest => match self.facing {
                    Direction::South => self.facing = Direction::West,
                    Direction::East => self.facing = Direction::North,
                    _ => panic!("impossible move"),
                },
                Pipe::BendSouthWest => match self.facing {
                    Direction::North => self.facing = Direction::West,
                    Direction::East => self.facing = Direction::South,
                    _ => panic!("impossible move"),
                },
                Pipe::BendSouthEast => match self.facing {
                    Direction::North => self.facing = Direction::East,
                    Direction::West => self.facing = Direction::South,
                    _ => panic!("impossible move"),
                },
                _ => {}
            },
            Tile::Ground => panic!("shouldn't be able to get to ground"),
        }

        match self.facing {
            Direction::North => self.position.1 += 1,
            Direction::South => self.position.1 -= 1,
            Direction::East => self.position.0 += 1,
            Direction::West => self.position.0 -= 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Pipe {
    Vertical = b'|',
    Horizontal = b'-',
    BendNorthEast = b'L',
    BendNorthWest = b'J',
    BendSouthWest = b'7',
    BendSouthEast = b'F',
}

impl Pipe {
    fn connects(&self, direction: Direction) -> bool {
        match self {
            Pipe::Vertical => matches!(direction, Direction::North | Direction::South),
            Pipe::Horizontal => matches!(direction, Direction::East | Direction::West),
            Pipe::BendNorthEast => matches!(direction, Direction::North | Direction::East),
            Pipe::BendNorthWest => matches!(direction, Direction::North | Direction::West),
            Pipe::BendSouthEast => matches!(direction, Direction::South | Direction::East),
            Pipe::BendSouthWest => matches!(direction, Direction::South | Direction::West),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Pipe(Pipe),
    Ground,
}

enum Direction {
    North,
    South,
    East,
    West,
}

pub fn part1(input: &str) -> u32 {
    let (_, map) = Map::parse(input).expect("should parse the map");
    0
}
pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 0);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
