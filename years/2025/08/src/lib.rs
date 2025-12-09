use std::{collections::HashSet, ops::ControlFlow};

use aoc_util::{
    euclid,
    parse::nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::{i64, newline},
        multi::separated_list1,
    },
};
use eyre::ContextCompat;
use itertools::Itertools;

type Vec3 = euclid::default::Vector3D<i64>;

fn parse(input: &str) -> IResult<&str, Vec<Vec3>> {
    separated_list1(
        newline,
        separated_list1(tag(","), i64).map(|v| {
            assert_eq!(v.len(), 3);
            Vec3::new(v[0], v[1], v[2])
        }),
    )
    .parse(input)
}

pub fn circuits(
    positions: &Vec<Vec3>,
    mut monitor: impl FnMut(usize, (Vec3, Vec3), &Vec<HashSet<Vec3>>) -> ControlFlow<()>,
) -> eyre::Result<Vec<HashSet<Vec3>>> {
    let position_pairs = positions
        .iter()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| (**b - **a).square_length())
        .collect_vec();

    let mut circuits = Vec::<HashSet<Vec3>>::new();
    for position in positions {
        circuits.push(HashSet::from_iter([*position]));
    }

    let mut iterations = 0;
    for (a, b) in position_pairs.into_iter() {
        // return the circuit index and circuit containing the given vector
        fn find_circuit_containing(v: &Vec3, circuits: &Vec<HashSet<Vec3>>) -> Option<usize> {
            circuits.iter().position(|c| c.contains(v))
        }

        let ca = find_circuit_containing(a, &circuits);
        let cb = find_circuit_containing(b, &circuits);
        match (ca, cb) {
            // both exist in the same circuit
            (Some(ia), Some(ib)) if ia == ib => {}
            // both exist but in a different circuit
            (Some(mut ia), Some(ib)) => {
                if ia > ib {
                    ia -= 1;
                }
                let removed = circuits.remove(ib);
                circuits[ia].extend(removed);
            }
            _ => unreachable!("all positions have been put in single node circuits"),
        };

        iterations += 1;
        match monitor(iterations, (*a, *b), &circuits) {
            ControlFlow::Continue(_) => {}
            ControlFlow::Break(_) => break,
        }
    }

    Ok(circuits)
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    #[cfg(test)]
    const ITERATIONS: usize = 10;
    #[cfg(not(test))]
    const ITERATIONS: usize = 1000;

    let (_, positions) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;

    let mut circuits = circuits(&positions, |iterations, _, _| match iterations {
        ITERATIONS => ControlFlow::Break(()),
        _ => ControlFlow::Continue(()),
    })?;

    circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    Ok(circuits.iter().take(3).map(|c| c.len() as u32).product())
}
pub fn part2(input: &str) -> eyre::Result<i64> {
    let (_, positions) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;

    let mut answer = None;
    circuits(&positions, |_, pair, circuits| {
        if circuits.len() == 1 {
            answer = Some(pair.0.x * pair.1.x);
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    })?;

    Ok(answer.wrap_err("no answer found")?)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 40);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 25272);
        Ok(())
    }
}
