use std::collections::HashSet;

mod parse {
    use aoc_util::parse::{
        nom::{
            bytes::complete::tag,
            character::complete::{char, line_ending, space0, space1, usize},
            multi::separated_list1,
            sequence::{preceded, separated_pair},
            IResult, Parser,
        },
        parse_num,
    };

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Card>> {
        fn parse_numbers_list(input: &str) -> IResult<&str, Vec<u32>> {
            let (input, _) = space0(input)?;
            separated_list1(space1, parse_num).parse(input)
        }

        fn parse_card(input: &str) -> IResult<&str, Card> {
            let (input, id) = preceded((tag("Card"), space1), usize).parse(input)?;
            let (input, _) = (char(':'), space0).parse(input)?;
            let (input, (numbers, winning_numbers)) =
                separated_pair(parse_numbers_list, tag(" | "), parse_numbers_list).parse(input)?;

            Ok((
                input,
                Card {
                    id: id - 1,
                    numbers: HashSet::from_iter(numbers.into_iter()),
                    winning_numbers: HashSet::from_iter(winning_numbers.into_iter()),
                },
            ))
        }

        separated_list1(line_ending, parse_card).parse(input)
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    pub fn match_count(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }
    pub fn points(&self) -> u32 {
        let count = self.match_count() as u32;
        if count == 0 {
            0
        } else {
            u32::pow(2, count - 1)
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let (_, cards) = parse::parse(input).expect("parse cards works");
    cards.iter().map(|card| card.points()).sum()
}
pub fn part2(input: &str) -> u32 {
    let (_, cards) = parse::parse(input).expect("parse cards works");
    let mut copies = vec![1u32; cards.len()];

    for card in &cards {
        let match_count = card.match_count();
        let start = card.id + 1;
        let end = (start + match_count).min(cards.len());

        let copy_count = copies[card.id];
        for copy in &mut copies[start..end] {
            *copy += copy_count;
        }
    }
    copies.iter().sum()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 13);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
