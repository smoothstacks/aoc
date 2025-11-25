use aoc_util::parse::nom::{
    bytes::tag,
    character::complete::{alpha1, newline, u16},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use itertools::Itertools;

#[derive(Debug)]
struct Reindeer {
    speed: u16,
    fly_seconds: u16,
    rest_seconds: u16,
}

#[derive(Debug)]
struct State {
    seconds: u16,
    flying: bool,
    distance: u16,
    score: u16,
}

fn parse(input: &str) -> IResult<&str, Vec<Reindeer>> {
    let reindeer = (
        alpha1,
        tag(" can fly "),
        terminated(u16, tag(" km/s")),
        preceded(tag(" for "), u16),
        tag(" seconds, but then must rest for "),
        terminated(u16, tag(" seconds.")),
    )
        .map(|(_, _, speed, fly, _, rest)| Reindeer {
            speed,
            fly_seconds: fly,
            rest_seconds: rest,
        });

    separated_list1(newline, reindeer).parse(input)
}

fn run_race(reindeer: &Vec<Reindeer>, mut seconds: u16) -> Vec<State> {
    let mut states = reindeer
        .iter()
        .map(|r| State {
            seconds: r.fly_seconds,
            flying: true,
            distance: 0,
            score: 0,
        })
        .collect_vec();

    assert_eq!(reindeer.len(), states.len());

    while seconds > 0 {
        for (reindeer, state) in reindeer.iter().zip(&mut states) {
            if state.flying {
                state.distance += reindeer.speed;
            }

            state.seconds -= 1;

            if state.seconds == 0 {
                state.seconds = if state.flying {
                    reindeer.rest_seconds
                } else {
                    reindeer.fly_seconds
                };
                state.flying = !state.flying;
            }
        }

        for leader in states.iter_mut().max_set_by_key(|k| k.distance) {
            leader.score += 1;
        }

        seconds -= 1;
    }

    states
}

pub fn part1(input: &str) -> eyre::Result<u16> {
    let (_, reindeer) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;
    let results = run_race(&reindeer, 2503);
    Ok(results.iter().map(|r| r.distance).max().unwrap())
}
pub fn part2(input: &str) -> eyre::Result<u16> {
    let (_, reindeer) = parse(input).map_err(|e| eyre::format_err!("parse error: {e}"))?;
    let results = run_race(&reindeer, 2503);
    Ok(results.iter().map(|r| r.score).max().unwrap())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 2660);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 2503);
        Ok(())
    }
}
