use std::collections::HashMap;

mod parse {

    use nom::{
        bytes::complete::{tag, take},
        character::complete::{anychar, newline},
        combinator::map_opt,
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair},
        IResult,
    };

    use super::*;

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        map_opt(anychar, |c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })(input)
    }

    fn parse_node(input: &str) -> IResult<&str, &str> {
        take(3usize)(input)
    }

    fn parse_node_pair(input: &str) -> IResult<&str, (&str, &str)> {
        delimited(
            tag("("),
            separated_pair(parse_node, tag(", "), parse_node),
            tag(")"),
        )(input)
    }

    fn parse_map_entry(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        separated_pair(parse_node, tag(" = "), parse_node_pair)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, (Map<'_>, Vec<Direction>)> {
        let (input, (directions, entries)) = separated_pair(
            many1(parse_direction),
            tag("\n\n"),
            separated_list1(newline, parse_map_entry),
        )(input)?;

        let map: HashMap<&str, [&str; 2]> = entries
            .into_iter()
            .map(|(k, (v1, v2))| (k, [v1, v2]))
            .collect();
        Ok((input, (Map(map), directions)))
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right = 1,
}

struct Map<'a>(HashMap<&'a str, [&'a str; 2]>);
impl Map<'_> {
    fn path_length(
        &self,
        start: &str,
        directions: &[Direction],
        end: impl Fn(&str) -> bool,
    ) -> usize {
        let mut direction = 0;
        let mut len = 0;
        let mut current = start;
        while !end(current) {
            current = self
                .0
                .get(current)
                .map(|options| options[directions[direction] as usize])
                .unwrap();

            assert!(current != "XXX");

            direction = (direction + 1) % directions.len();
            len += 1;
        }

        len
    }

    fn follow_paths_2(&self, directions: &[Direction]) -> usize {
        self.0
            .keys()
            .copied()
            .filter_map(|s| {
                if s.ends_with('A') {
                    Some(self.path_length(s, directions, |o| o.ends_with('Z')))
                } else {
                    None
                }
            })
            .inspect(|p| println!("{p}"))
            .product()
    }

    fn _follow_paths(&self, directions: &[Direction]) -> usize {
        let mut start_nodes: Vec<&str> = self
            .0
            .keys()
            .copied()
            .filter(|s| s.ends_with('A'))
            .collect();

        let mut len = 0;
        let mut direction = 0;
        loop {
            if start_nodes[2..3].iter().all(|s| s.ends_with('Z')) {
                break;
            }

            for node in &mut start_nodes[2..3] {
                *node = self
                    .0
                    .get(*node)
                    .map(|options| options[directions[direction] as usize])
                    .unwrap();
            }

            direction = (direction + 1) % directions.len();
            len += 1
        }

        len
    }
}

pub fn part1(input: &str) -> u32 {
    let (_, (map, directions)) = parse::parse(input).expect("parse works");
    map.path_length(START, &directions, |s| s == END) as u32
}
pub fn part2(input: &str) -> u32 {
    let (_, (map, directions)) = parse::parse(input).expect("parse works");
    map.follow_paths_2(&directions) as u32
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_works() {
        const INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        const INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(super::part1(INPUT_1), 2);
        assert_eq!(super::part1(INPUT_2), 6);
    }
    #[test]
    fn part2_works() {
        const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(super::part2(INPUT), 6);
    }
}
