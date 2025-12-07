use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub type Position = euclid::default::Vector2D<isize>;
pub type Vector = euclid::default::Vector2D<isize>;

type Size = euclid::default::Size2D<usize>;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    dimensions: Size,
    data: Vec<T>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
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

        Ok(Self { dimensions, data })
    }

    pub fn empty() -> Self {
        Self {
            dimensions: Size::zero(),
            data: vec![],
        }
    }

    pub fn get(&self, p: Position) -> Option<&T> {
        self.data.get(self.idx(p)?)
    }
    pub fn get_mut(&mut self, p: Position) -> Option<&mut T> {
        let idx = self.idx(p)?;
        self.data.get_mut(idx)
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

    pub fn row_iter(&self, row: usize) -> impl DoubleEndedIterator<Item = &T> {
        let start = row * self.dimensions.width;
        let end = start + self.dimensions.width;
        self.data[row * self.dimensions.width..end].iter()
    }
    pub fn column_iter(&self, column: usize) -> impl DoubleEndedIterator<Item = &T> {
        (0..self.dimensions.height).filter_map(move |row| {
            let idx = self.idx(Position::new(column as isize, row as isize))?;
            Some(&self.data[idx])
        })
    }

    pub fn enumerate_column(
        &self,
        column: usize,
    ) -> impl DoubleEndedIterator<Item = (Position, &T)> {
        (0..self.dimensions.height).filter_map(move |row| {
            let pos = Position::new(column as isize, row as isize);
            let idx = self.idx(pos)?;
            Some((pos, &self.data[idx]))
        })
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

    pub fn iter_direction(
        &self,
        mut pos: Position,
        direction: Vector,
    ) -> impl Iterator<Item = (Position, &T)> {
        std::iter::from_fn(move || {
            pos += direction;
            Some((pos, self.get(pos)?))
        })
    }

    pub fn add_row(&mut self, mut data: Vec<T>) -> Result<(), Error> {
        match self.dimensions.width {
            0 => self.dimensions.width = data.len(),
            x if x != data.len() => return Err(Error::DataSizeMismatch),
            _ => {}
        }

        self.data.append(&mut data);
        self.dimensions.height += 1;
        Ok(())
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            dimensions: self.dimensions,
            data: self.data.iter().map(f).collect(),
        }
    }

    fn idx(&self, n: Position) -> Option<usize> {
        self.is_valid_position(n)
            .then(|| n.y as usize * self.dimensions.width + n.x as usize)
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
            data: vec![value; dimensions.width * dimensions.height],
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
        assert_eq!(&grid.data, &['a', 'b', 'c', 'd']);
    }

    #[test]
    fn get() {
        let grid = Grid::new(euclid::size2(2, 2), vec![1, 2, 3, 4]).unwrap();
        assert_eq!(grid.get(euclid::vec2(0, 0)), Some(&1));
        assert_eq!(grid.get(euclid::vec2(1, 0)), Some(&2));
        assert_eq!(grid.get(euclid::vec2(0, 1)), Some(&3));
        assert_eq!(grid.get(euclid::vec2(1, 1)), Some(&4));
    }

    #[test]
    fn add_row() -> Result<(), super::Error> {
        let mut grid = Grid::empty();
        grid.add_row(vec![1, 2])?;
        assert_eq!(grid.get(euclid::vec2(0, 0)), Some(&1));
        assert_eq!(grid.get(euclid::vec2(1, 0)), Some(&2));
        assert_eq!(grid.get(euclid::vec2(0, 1)), None);
        assert_eq!(grid.get(euclid::vec2(2, 0)), None);

        Ok(())
    }
}
