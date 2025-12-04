pub use glam;

pub mod parse {
    use std::str::FromStr;

    pub use nom;
    use nom::{character::complete::digit0, combinator::map_res, AsChar, IResult, Input, Parser};

    pub fn parse_num<I, T>(input: I) -> IResult<I, T>
    where
        I: Input + AsRef<str>,
        <I as Input>::Item: AsChar,
        T: FromStr,
    {
        map_res(digit0, |s: I| s.as_ref().parse::<T>()).parse(input)
    }
}

pub mod chargrid {
    use std::hash::Hash;

    #[derive(Debug, Clone, derive_more::From, derive_more::Deref)]
    pub struct CharGrid<'a>(&'a str);

    impl<'a> CharGrid<'a> {
        pub fn new(input: &'a str) -> Self {
            Self(input)
        }

        pub fn get(&self, CharGridVec(x, y): CharGridVec) -> Option<char> {
            self.0.lines().nth(y as usize)?.chars().nth(x as usize)
        }

        pub fn find(&self, c: char) -> Option<CharGridVec> {
            for (y, line) in self.0.lines().enumerate() {
                if let Some(x) = line.find(c) {
                    return Some(CharGridVec(x as isize, y as isize));
                }
            }

            None
        }

        pub fn find_all(&'a self, c: char) -> impl Iterator<Item = Position> + use<'a> {
            self.0.lines().enumerate().flat_map(move |(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c2)| {
                    if c == c2 {
                        Some(CharGridVec(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
        }

        pub fn is_valid_position(&self, pos: CharGridVec) -> bool {
            pos.0 >= 0
                && pos.1 >= 0
                && pos.1 < self.0.len() as isize
                && self
                    .0
                    .lines()
                    .nth(pos.1 as usize)
                    .is_some_and(|l| pos.0 < l.len() as isize)
        }

        pub fn cursor(&self, pos: CharGridVec) -> CharGridCursor<'_> {
            CharGridCursor::new(self, pos)
        }
    }

    pub struct CharGridCursor<'a> {
        grid: &'a CharGrid<'a>,
        pos: CharGridVec,
    }
    impl<'a> CharGridCursor<'a> {
        pub fn new(grid: &'a CharGrid<'a>, pos: CharGridVec) -> Self {
            Self { grid, pos }
        }

        pub fn get(&self) -> Option<char> {
            self.grid.get(self.pos)
        }

        pub fn move_to(&mut self, pos: CharGridVec) {
            self.pos = pos;
        }

        pub fn move_by(&mut self, dir: CharGridVec) {
            self.pos = self.pos + dir;
        }

        pub fn peek(&self, dir: CharGridVec) -> Option<char> {
            self.grid.get(self.pos + dir)
        }
    }

    #[derive(
        Hash,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        derive_more::From,
        derive_more::Add,
        derive_more::Display,
    )]
    #[display("(x={}, y={})", self.0, self.1)]
    pub struct CharGridVec(pub isize, pub isize);

    pub type Position = CharGridVec;
    pub type Direction = CharGridVec;

    impl Direction {
        pub fn rotate(&self, clockwise: bool) -> Self {
            if clockwise {
                Self(-self.1, self.0)
            } else {
                Self(self.1, -self.0)
            }
        }
    }

    impl std::ops::Mul<isize> for CharGridVec {
        type Output = Self;

        fn mul(self, rhs: isize) -> Self::Output {
            Self(self.0 * rhs, self.1 * rhs)
        }
    }
    impl std::ops::Sub<Self> for CharGridVec {
        type Output = Direction;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rotate() {
            let direction = CharGridVec(0, -1);
            assert_eq!(direction.rotate(true), CharGridVec(1, 0));
        }
    }
}
