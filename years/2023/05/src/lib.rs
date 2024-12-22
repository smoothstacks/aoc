use std::ops::Range;

mod parse {
    use std::str::FromStr;

    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, newline, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::{preceded, separated_pair, tuple},
        IResult,
    };

    use super::*;

    fn parse_num<T: FromStr>(input: &str) -> IResult<&str, T> {
        map_res(digit1, str::parse::<T>)(input)
    }

    fn parse_seeds(input: &str) -> IResult<&str, Vec<isize>> {
        preceded(tag("seeds: "), separated_list1(space1, parse_num::<isize>))(input)
    }

    fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
        let (input, destination_start) = parse_num::<isize>(input)?;
        let (input, _) = space1(input)?;
        let (input, source_start) = parse_num::<isize>(input)?;
        let (input, _) = space1(input)?;
        let (input, length) = parse_num::<isize>(input)?;

        let source = source_start..source_start + length;
        let offset = destination_start - source_start;

        Ok((input, Mapping { source, offset }))
    }

    fn parse_category_map(input: &str) -> IResult<&str, Vec<Mapping>> {
        let (input, (_, _)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
        let (input, _) = tuple((tag(" map:"), newline))(input)?;
        let (input, mappings) = separated_list1(newline, parse_mapping)(input)?;
        Ok((input, mappings))
    }

    pub fn parse(input: &str) -> IResult<&str, Almanac> {
        let (input, seeds) = parse_seeds(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, maps) = separated_list1(tag("\n\n"), parse_category_map)(input)?;

        Ok((input, Almanac { seeds, maps }))
    }
}

#[derive(Debug)]
struct Mapping {
    source: Range<isize>,
    offset: isize,
}

impl Mapping {
    pub fn contains(&self, value: isize) -> bool {
        self.source.contains(&value)
    }

    pub fn map(&self, source: isize) -> Option<isize> {
        self.source
            .contains(&source)
            .then_some(source + self.offset)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<isize>,
    maps: Vec<Vec<Mapping>>,
}

impl Almanac {
    pub fn seed_ranges(&self) -> Vec<Range<isize>> {
        self.seeds
            .chunks(2)
            .map(|c| c[0]..c[0] + c[1] + 1)
            .collect()
    }

    pub fn apply_seed(&self, seed: isize) -> isize {
        let mut value = seed;
        for mapping in &self.maps {
            'range: for m in mapping {
                if m.contains(value) {
                    value = m.map(value).unwrap();
                    break 'range;
                }
            }
        }
        value
    }
}

pub fn part1(input: &str) -> u32 {
    let (_, almanac) = parse::parse(input).expect("parse works");
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.apply_seed(*seed))
        .min()
        .unwrap() as u32
}

use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn part2(input: &str) -> u64 {
    let (_, almanac) = parse::parse(input).expect("parse works");

    let seeds = almanac.seed_ranges();
    let count: u64 = seeds.clone().into_iter().map(|r| r.count() as u64).sum();

    seeds
        .into_par_iter()
        .flatten()
        .map(|seed| almanac.apply_seed(seed))
        .progress_count(count)
        .min()
        .unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 35);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 46);
    }

    #[test]
    fn map_works() {
        let mapping = Mapping {
            source: 10..21,
            offset: 10,
        };

        assert_eq!(mapping.map(1), None);
        assert_eq!(mapping.map(10), Some(20));
        assert_eq!(mapping.map(15), Some(25));
        assert_eq!(mapping.map(20), Some(30));
        assert_eq!(mapping.map(25), None);

        let mapping = Mapping {
            source: 10..21,
            offset: -10,
        };
        assert_eq!(mapping.map(1), None);
        assert_eq!(mapping.map(10), Some(0));
        assert_eq!(mapping.map(15), Some(5));
        assert_eq!(mapping.map(20), Some(10));
        assert_eq!(mapping.map(25), None);
    }
}
