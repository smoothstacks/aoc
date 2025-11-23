use std::collections::HashMap;

mod parse {
    use aoc_util::parse::nom::{
        character::complete::{anychar, digit1, newline, space1},
        combinator::{map_opt, map_res},
        multi::{count, separated_list1},
        sequence::separated_pair,
        IResult, Parser,
    };

    use super::*;

    impl Card {
        pub fn parse(input: &str) -> IResult<&str, Card> {
            map_opt(anychar, |label| {
                Card::ALL
                    .iter()
                    .position(|other| *other == label)
                    .map(|rank| Card { label, rank })
            })
            .parse(input)
        }
    }

    impl Hand {
        pub fn parse(input: &str) -> IResult<&str, Hand> {
            let (input, cards_vec) = count(Card::parse, 5).parse(input)?;
            let mut cards = [Card::default(); 5];
            cards.copy_from_slice(&cards_vec);
            Ok((input, Hand::new(cards)))
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
        separated_list1(
            newline,
            separated_pair(Hand::parse, space1, map_res(digit1, str::parse::<u32>)),
        )
        .parse(input)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Card {
    label: char,
    rank: usize,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl Card {
    const ALL: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],

    distinct_labels: usize,
    max_label_count: usize,
    label_counts: HashMap<char, usize>,
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let mut label_counts = HashMap::new();
        for card in &cards {
            label_counts
                .entry(card.label)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let distinct_labels = label_counts.len();
        let max_label_count = *label_counts.values().max().unwrap();

        Self {
            cards,
            label_counts,
            distinct_labels,
            max_label_count,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distinct_labels
            .cmp(&self.distinct_labels)
            .then(self.max_label_count.cmp(&other.max_label_count))
            .then(self.cards.cmp(&other.cards))
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{:?}", card)?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> u32 {
    let (_, mut hands) = parse::parse(input).expect("parse works");
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1) as u32)
        .sum()
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::Hand;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 6440);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 0);
    }

    #[test]
    fn hand_tests() {
        let first = Hand::parse("33332").unwrap().1;
        let second = Hand::parse("2AAAA").unwrap().1;

        assert!(first > second);
    }
}
