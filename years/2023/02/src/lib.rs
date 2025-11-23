use std::ops::{Index, IndexMut};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            _ => panic!("invalid color"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Cubes([usize; 3]);
impl Cubes {
    fn iter<'a>(&'a self) -> impl Iterator<Item = Cube> + 'a {
        self.0.iter().enumerate().map(|(i, amount)| Cube {
            amount: *amount,
            color: Color::from(i as u8),
        })
    }

    fn power(&self) -> u32 {
        self.iter().map(|c| c.amount as u32).product()
    }
}

impl<'a> FromIterator<&'a Cube> for Cubes {
    fn from_iter<T: IntoIterator<Item = &'a Cube>>(iter: T) -> Self {
        let mut res = Self([0; 3]);
        for cube in iter {
            res[cube.color] = cube.amount;
        }
        res
    }
}

impl Index<Color> for Cubes {
    type Output = usize;

    fn index(&self, index: Color) -> &Self::Output {
        &self.0[index as u8 as usize]
    }
}
impl IndexMut<Color> for Cubes {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.0[index as u8 as usize]
    }
}

#[derive(Debug)]
struct Cube {
    color: Color,
    amount: usize,
}

#[derive(Debug)]
struct Revealed {
    cubes: Cubes,
}

#[derive(Debug)]
struct Game {
    id: u32,
    revealed: Vec<Revealed>,
}

impl Game {
    fn is_possible(&self, cubes: &[Cube]) -> bool {
        for cube in cubes {
            for revealed in &self.revealed {
                for revealed_cube in revealed.cubes.iter() {
                    if revealed_cube.color == cube.color && revealed_cube.amount > cube.amount {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn min_required_cubes(&self) -> Cubes {
        let mut min = Cubes::default();
        for revealed in &self.revealed {
            for cube in revealed.cubes.iter() {
                if min[cube.color] < cube.amount {
                    min[cube.color] = cube.amount;
                }
            }
        }

        min
    }
}

mod parse {
    use aoc_util::parse::nom::{
        bytes::complete::tag,
        character::complete::{digit1, newline, space0, space1},
        combinator::map_res,
        error::{Error, ErrorKind},
        multi::separated_list0,
        sequence::{preceded, separated_pair},
        Err, IResult, Parser,
    };

    use super::*;

    fn parse_color(input: &str) -> IResult<&str, Color> {
        const COLORS: [&str; 3] = ["red", "green", "blue"];

        match COLORS
            .iter()
            .enumerate()
            .find_map(|(i, color)| input.strip_prefix(color).map(|s| (i, s)))
        {
            Some((i, input)) => Ok((input, Color::from(i as u8))),
            None => Err(Err::Error(Error::new(input, ErrorKind::Fail))),
        }
    }

    fn parse_cube(input: &str) -> IResult<&str, Cube> {
        let (input, _) = space0(input)?;
        let (input, amount) = map_res(digit1, str::parse::<usize>).parse(input)?;
        let (input, _) = space1(input)?;
        let (input, color) = parse_color(input)?;
        Ok((input, Cube { color, amount }))
    }

    fn parse_revealed(input: &str) -> IResult<&str, Revealed> {
        let (input, cubes) = separated_list0(tag(","), parse_cube).parse(input)?;
        Ok((
            input,
            Revealed {
                cubes: cubes.iter().collect(),
            },
        ))
    }

    fn parse_revealed_list(input: &str) -> IResult<&str, Vec<Revealed>> {
        let (input, revealed) = separated_list0(tag(";"), parse_revealed).parse(input)?;
        Ok((input, revealed))
    }

    fn parse_game_id(input: &str) -> IResult<&str, u32> {
        map_res(preceded(tag("Game "), digit1), str::parse).parse(input)
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        let (input, (id, revealed)) =
            separated_pair(parse_game_id, tag(":"), parse_revealed_list).parse(input)?;

        Ok((input, Game { id, revealed }))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        let (input, games) = separated_list0(newline, parse_game).parse(input)?;
        Ok((input, games))
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn parse_works() {
            let input = " 3 blue";
            let game = super::parse::parse_cube(input);
            assert!(game.is_ok());

            let input = "3 blue, 5 red, 6 green";
            let revealed = super::parse_revealed(input);
            assert!(revealed.is_ok());

            let input = "Game 1: 3 blue, 4 red, 6 green; 2 red, 4 green; 5 green";
            let fullgame = super::parse_game(input);
            assert!(fullgame.is_ok());
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let cubes = vec![
        Cube {
            color: Color::Red,
            amount: 12,
        },
        Cube {
            color: Color::Green,
            amount: 13,
        },
        Cube {
            color: Color::Blue,
            amount: 14,
        },
    ];
    let (_, games) = parse::parse(input).expect("parse games should work");

    games
        .iter()
        .filter_map(|game| game.is_possible(&cubes).then_some(game.id))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (_, games) = parse::parse(input).expect("parse games works");
    games
        .iter()
        .map(|g| g.min_required_cubes())
        .map(|c| c.power())
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 8);
    }

    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 2286)
    }

    #[test]
    fn parse_works() {
        let games = super::parse::parse(INPUT);
        assert!(games.is_ok());

        let (_, games) = games.unwrap();
        assert_eq!(games.len(), 5)
    }
}
