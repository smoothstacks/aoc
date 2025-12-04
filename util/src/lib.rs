pub use euclid;
pub use glam;

pub mod math {
    use num::Unsigned;

    pub fn num_digits<T>(mut n: T) -> usize
    where
        T: num::PrimInt + Unsigned,
    {
        let ten = T::from(10u8).expect("primitive ints can be created from u8");

        let mut count = 0;
        while n > num::zero() {
            n = n / ten;
            count += 1;
        }

        count
    }

    pub fn split_num_at<T>(n: T, at: usize) -> (T, T)
    where
        T: num::PrimInt + Unsigned,
    {
        let ten = T::from(10u8).expect("primitive ints can be created from u8");
        let pow = ten.pow(at as u32);
        let right = n % pow;
        let left = (n - right) / pow;
        (left, right)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn num_digits() {
            assert_eq!(super::num_digits(999u64), 3);
            assert_eq!(super::num_digits(999u32), 3);
            assert_eq!(super::num_digits(999u16), 3);
            assert_eq!(super::num_digits(111u8), 3);
            assert_eq!(super::num_digits(8291469824u64), 10);
        }

        #[test]
        fn split_num_at() {
            assert_eq!(super::split_num_at(123456u32, 3), (123, 456));
            assert_eq!(super::split_num_at(0u32, 3), (0, 0));
            assert_eq!(super::split_num_at(10u32, 1), (1, 0));
            assert_eq!(super::split_num_at(1000u32, 2), (10, 0));
        }
    }
}

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

    pub type Position = euclid::default::Vector2D<isize>;
    pub type Vector = euclid::default::Vector2D<isize>;

    type Size = euclid::default::Size2D<usize>;

    pub struct Grid<T> {
        dimensions: Size,
        data: Box<[T]>,
    }

    impl<T: Display> Display for Grid<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.dimensions.width {
                for x in 0..self.dimensions.height {
                    write!(
                        f,
                        "|{}",
                        self.get(euclid::vec2(x as isize, y as isize)).unwrap()
                    )?;
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
        pub fn new(dimensions: Size, data: Vec<T>) -> Result<Self, Error> {
            if data.len() != dimensions.width * dimensions.height {
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
            p.x >= 0
                && (p.x as usize) < self.dimensions.width
                && (p.y >= 0)
                && (p.y as usize) < self.dimensions.height
        }

        pub fn get_dimensions(&self) -> Size {
            self.dimensions
        }

        pub fn position_iter(&self) -> impl Iterator<Item = Position> + use<T> {
            let [dx, dy] = self.dimensions.to_array();
            (0..dy).flat_map(move |y| (0..dx).map(move |x| euclid::vec2(x as isize, y as isize)))
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
                let next = from + p;
                Some((next, self.get(next)?))
            })
        }

        fn idx(&self, n: Position) -> Option<usize> {
            self.is_valid_position(n)
                .then(|| n.y as usize * self.dimensions.height + n.x as usize)
        }
        fn pos(&self, idx: usize) -> Option<Position> {
            (idx < self.data.len()).then_some({
                let x = idx % self.dimensions.height;
                let y = idx / self.dimensions.height;
                euclid::vec2(x as isize, y as isize)
            })
        }
    }

    impl<T: Clone> Grid<T> {
        pub fn splat(dimensions: Size, value: T) -> Self {
            Self {
                dimensions,
                data: vec![value; dimensions.width * dimensions.height].into_boxed_slice(),
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

        pub fn position_of(&self, cmp: &T) -> Option<Position> {
            for (i, t) in self.data.iter().enumerate() {
                if cmp == t {
                    return self.pos(i);
                }
            }

            return None;
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

            Self::new(euclid::size2(x, y), data)
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
            assert_eq!(grid.dimensions, euclid::size2(2, 2));
            assert_eq!(grid.data.as_ref(), &['a', 'b', 'c', 'd']);
        }

        #[test]
        fn get() {
            let grid = Grid::new(euclid::size2(2, 2), vec![1, 2, 3, 4]).unwrap();
            assert_eq!(grid.get(euclid::vec2(0, 0)), Some(&1));
            assert_eq!(grid.get(euclid::vec2(1, 0)), Some(&2));
            assert_eq!(grid.get(euclid::vec2(0, 1)), Some(&3));
            assert_eq!(grid.get(euclid::vec2(1, 1)), Some(&4));
        }
    }
}
