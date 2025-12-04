pub use euclid;
pub use glam;

pub mod parse {
    use std::str::FromStr;

    pub use nom;
    use nom::{AsChar, IResult, Input, Parser, character::complete::digit0, combinator::map_res};

    pub fn parse_num<I, T>(input: I) -> IResult<I, T>
    where
        I: Input + AsRef<str>,
        <I as Input>::Item: AsChar,
        T: FromStr,
    {
        map_res(digit0, |s: I| s.as_ref().parse::<T>()).parse(input)
    }
}

pub mod grid {
    use std::{fmt::Display, str::FromStr};

    use euclid;

    pub type Position = euclid::default::Vector2D<usize>;
    pub type Vector = euclid::default::Vector2D<isize>;

    pub struct Grid<T> {
        dimensions: Position,
        data: Box<[T]>,
    }

    impl<T: Display> Display for Grid<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.dimensions.y {
                for x in 0..self.dimensions.x {
                    write!(f, "|{}", self.get(euclid::vec2(x, y)).unwrap())?;
                }
                writeln!(f, "|")?;
            }

            Ok(())
        }
    }

    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub enum Error {
        NoData,
        DataSizeMismatch,
    }

    impl<T> Grid<T> {
        pub fn new(dimensions: Position, data: Vec<T>) -> Result<Self, Error> {
            if data.len() != dimensions.x * dimensions.y {
                return Err(Error::DataSizeMismatch);
            }

            Ok(Self {
                dimensions,
                data: data.into_boxed_slice(),
            })
        }

        pub fn get(&self, p: Position) -> Option<&T> {
            self.data.get(self.idx(p)?)
        }
        pub fn get_mut(&mut self, p: Position) -> Option<&mut T> {
            self.data.get_mut(self.idx(p)?)
        }

        pub fn is_valid_position(&self, p: Position) -> bool {
            p.x < self.dimensions.x && p.y < self.dimensions.y
        }

        pub fn get_dimensions(&self) -> Position {
            self.dimensions
        }

        pub fn position_iter(&self) -> impl Iterator<Item = Position> + use<T> {
            let [dx, dy] = self.dimensions.to_array();
            (0..dy).flat_map(move |y| (0..dx).map(move |x| euclid::vec2(x, y)))
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.data.iter()
        }
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
            self.data.iter_mut()
        }

        pub fn enumerate(&self) -> impl Iterator<Item = (Position, &T)> {
            self.position_iter().zip(self.iter())
        }
        pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
            self.position_iter().zip(self.iter_mut())
        }

        pub fn iter_pattern(
            &self,
            from: Position,
            pattern: impl Iterator<Item = Vector>,
        ) -> impl Iterator<Item = (Position, &T)> {
            pattern.filter_map(move |p| {
                let next = euclid::vec2(
                    from.x.checked_add_signed(p.x)?,
                    from.y.checked_add_signed(p.y)?,
                );

                Some((next, self.get(next)?))
            })
        }

        fn idx(&self, n: Position) -> Option<usize> {
            self.is_valid_position(n)
                .then_some(n.y * self.dimensions.y + n.x)
        }
        fn pos(&self, idx: usize) -> Option<Position> {
            (idx < self.data.len()).then_some({
                let x = idx % self.dimensions.y;
                let y = idx / self.dimensions.y;
                euclid::vec2(x, y)
            })
        }
    }

    impl<T: Clone> Grid<T> {
        pub fn splat(dimensions: Position, value: T) -> Self {
            Self {
                dimensions,
                data: vec![value; dimensions.x * dimensions.y].into_boxed_slice(),
            }
        }
    }

    impl<T: PartialEq> Grid<T> {
        pub fn find_all<'a, 'b>(
            &'a self,
            cmp: &'b T,
        ) -> impl Iterator<Item = Position> + use<'a, 'b, T> {
            // SAFETY: only check in self.pos is whether input is a valid index,
            //  we guarantee that by iterating data
            self.data
                .iter()
                .enumerate()
                .filter_map(move |(i, t)| (cmp == t).then_some(self.pos(i).unwrap()))
        }
    }

    impl FromStr for Grid<char> {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.is_empty() {
                return Err(Error::NoData);
            }

            let y = s.lines().count();
            let x = s
                .lines()
                .map(|l| l.chars().count())
                .max()
                .ok_or(Error::NoData)?;

            let data: Vec<_> = s.lines().flat_map(|c| c.chars()).collect();

            Self::new(euclid::vec2(x, y), data)
        }
    }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;

        use crate::grid::Grid;

        #[test]
        fn new_works() {
            let s = "ab\ncd";
            let grid = super::Grid::from_str(s).unwrap();
            assert_eq!(grid.dimensions, euclid::vec2(2, 2));
            assert_eq!(grid.data.as_ref(), &['a', 'b', 'c', 'd']);
        }

        #[test]
        fn get() {
            let grid = Grid::new(euclid::vec2(2, 2), vec![1, 2, 3, 4]).unwrap();
            assert_eq!(grid.get(euclid::vec2(0, 0)), Some(&1));
            assert_eq!(grid.get(euclid::vec2(1, 0)), Some(&2));
            assert_eq!(grid.get(euclid::vec2(0, 1)), Some(&3));
            assert_eq!(grid.get(euclid::vec2(1, 1)), Some(&4));
        }
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
