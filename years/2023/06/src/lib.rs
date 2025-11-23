mod parse {
    use aoc_util::parse::{
        nom::{
            bytes::complete::tag,
            character::complete::{digit1, newline, space1},
            multi::separated_list1,
            sequence::preceded,
            IResult, Parser,
        },
        parse_num,
    };

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Race>> {
        let (input, times) =
            preceded(tag("Time:"), separated_list1(space1, parse_num)).parse(input)?;
        let (input, _) = newline(input)?;
        let (input, distances) =
            preceded(tag("Distance:"), separated_list1(space1, parse_num)).parse(input)?;

        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok((input, races))
    }

    pub fn parse_single(input: &str) -> IResult<&str, Race> {
        let (input, times) =
            preceded((tag("Time:"), space1), separated_list1(space1, digit1)).parse(input)?;
        let (input, _) = newline(input)?;
        let (input, distances) =
            preceded((tag("Distance:"), space1), separated_list1(space1, digit1)).parse(input)?;

        let time = times.join("").parse::<u64>().expect("times should join");
        let distance = distances
            .join("")
            .parse::<u64>()
            .expect("distances should join");

        Ok((input, Race { time, distance }))
    }
}

fn total_distance(max_time: u64, button_held: u64) -> u64 {
    // assert!(button_held < max_time);
    (max_time - button_held) * button_held
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_times(&self) -> impl Iterator<Item = u64> + '_ {
        (1..self.time)
            .map(|t| total_distance(self.time, t))
            .filter(|d| *d > self.distance)
    }
}

pub fn part1(input: &str) -> u64 {
    let (_, races) = parse::parse(input).expect("parse works");
    races
        .iter()
        .map(|r| r.winning_times().count() as u64)
        .product()
}
pub fn part2(input: &str) -> u64 {
    let (_, race) = parse::parse_single(input).expect("parse works");
    race.winning_times().count() as u64
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        assert_eq!(super::part1(INPUT), 288);
    }
    #[test]
    fn part2_works() {
        assert_eq!(super::part2(INPUT), 71503);
    }
}
