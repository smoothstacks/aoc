use std::collections::HashMap;

use aoc_util::parse::nom::{
    bytes::complete::tag,
    character::{
        char,
        complete::{alpha1, i8, newline},
    },
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use itertools::Itertools;

// name -> (category -> score)
type Ingredients<'a> = Vec<(&'a str, HashMap<&'a str, i8>)>;

fn parse(input: &str) -> IResult<&str, Ingredients<'_>> {
    separated_list1(
        newline,
        (
            terminated(alpha1, tag(": ")),
            separated_list1(tag(", "), separated_pair(alpha1, char(' '), i8))
                .map(HashMap::from_iter),
        ),
    )
    .parse(input)
}

fn evaluate(ingredients: &Ingredients<'_>, amounts: &[i64]) -> (i64, i64) {
    let mut category_scores = HashMap::new();
    let mut calories = 0i64;

    for ((_, categories), amount) in ingredients.iter().zip(amounts) {
        for (category, score) in categories.iter().filter(|c| *c.0 != "calories") {
            let next = amount * *score as i64;
            category_scores
                .entry(category)
                .and_modify(|v| *v += next)
                .or_insert(amount * *score as i64);
        }

        calories += categories.get("calories").copied().unwrap_or_default() as i64 * *amount;
    }

    (
        category_scores.values().map(|v| (*v).max(0)).product(),
        calories,
    )
}

pub fn part1(input: &str) -> eyre::Result<i64> {
    let (_, ingredients) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    let best = (0..=100i64)
        .permutations(ingredients.len())
        .filter(|a| a.iter().sum::<i64>() == 100)
        .map(|amounts| evaluate(&ingredients, &amounts))
        .max_by_key(|s| s.0)
        .unwrap();

    Ok(best.0)
}
pub fn part2(input: &str) -> eyre::Result<i64> {
    let (_, ingredients) = parse(input).map_err(|e| eyre::format_err!("{e}"))?;

    let best = (0..=100i64)
        .permutations(ingredients.len())
        .filter(|a| a.iter().sum::<i64>() == 100)
        .map(|amounts| evaluate(&ingredients, &amounts))
        .filter(|(_, calories)| *calories == 500)
        .max_by_key(|(score, _)| *score)
        .unwrap();

    Ok(best.0)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Ingredients;

    const INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    fn expected() -> Ingredients<'static> {
        vec![
            (
                "Butterscotch",
                HashMap::from_iter([
                    ("capacity", -1),
                    ("durability", -2),
                    ("flavor", 6),
                    ("texture", 3),
                    ("calories", 8),
                ]),
            ),
            (
                "Cinnamon",
                HashMap::from_iter([
                    ("capacity", 2),
                    ("durability", 3),
                    ("flavor", -2),
                    ("texture", -1),
                    ("calories", 3),
                ]),
            ),
        ]
    }

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 62842880);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 57600000);
        Ok(())
    }
    #[test]
    fn parse_works() -> eyre::Result<()> {
        assert_eq!(super::parse(INPUT)?.1, expected());
        Ok(())
    }
    #[test]
    fn evluate_works() -> eyre::Result<()> {
        let score = super::evaluate(&expected(), &[44, 56]).0;
        assert_eq!(score, 62842880);
        Ok(())
    }
}
