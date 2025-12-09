use aoc_util::{
    euclid,
    parse::nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::{i64, newline},
        multi::separated_list1,
        sequence::separated_pair,
    },
};
use itertools::Itertools;

type Vec2 = euclid::default::Point2D<i64>;

fn parse(input: &str) -> IResult<&str, Vec<Vec2>> {
    separated_list1(
        newline,
        separated_pair(i64, tag(","), i64).map(|(a, b)| euclid::point2(a, b)),
    )
    .parse(input)
}

pub fn part1(input: &str) -> eyre::Result<i64> {
    let (_, positions) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;

    Ok(positions
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a != b)
        .map(|(a, b)| {
            // expand by +1 each direction to include the corners
            let size: euclid::default::Size2D<i64> =
                euclid::size2((a.x - b.x).abs() + 1, (a.y - b.y).abs() + 1);
            euclid::Box2D::from_size(size).area()
        })
        .max()
        .unwrap())
}
pub fn part2(input: &str) -> eyre::Result<i64> {
    let (_, positions) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;

    // loops over triplets, looking for concave vs convex corners
    // TODO: this assumes that you can only build rectangles from these
    // triplets, but that might not be true!
    Ok(positions
        .iter()
        .circular_tuple_windows()
        .filter(|(a, m, b)| {
            let ab = **b - **a;
            let am = **m - **a;
            let l = am.cross(ab);
            l > 0
        })
        .map(|(a, _, b)| {
            let size: euclid::default::Size2D<i64> =
                euclid::size2((a.x - b.x).abs(), (a.y - b.y).abs());
            euclid::Box2D::from_size(size).area()
        })
        .max()
        .unwrap())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 50);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 24);
        Ok(())
    }
}
