use itertools::Itertools;
use std::cmp::Ordering;

type Reports = Vec<Vec<i32>>;

fn is_safe<'a>(report: impl Iterator<Item = &'a i32>) -> bool {
    let mut gradient: Option<Ordering> = None;

    for (left, right) in report.tuple_windows() {
        let diff = (left - right).abs();
        let ord = left.cmp(&right);

        let safe = match ord {
            // equal is never safe
            Ordering::Equal => false,
            Ordering::Greater | Ordering::Less => {
                // store gradient for next iteration
                gradient.get_or_insert(ord);

                // safe if diff is between 1 and 3 and gradient is the same
                diff >= 1 && diff <= 3 && Some(ord) == gradient
            }
        };

        if !safe {
            return false;
        }
    }

    return true;
}

fn parse(input: &str) -> eyre::Result<Reports> {
    let mut reports = Reports::new();

    for line in input.lines() {
        let line = line.trim();
        let report = line
            .split(" ")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        reports.push(report);
    }

    Ok(reports)
}

pub fn part1(input: &str) -> eyre::Result<usize> {
    let reports = parse(input)?;
    Ok(reports
        .iter()
        .filter(|report| is_safe(report.iter()))
        .count())
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let reports = parse(input)?;

    let mut total = 0;
    for report in reports {
        if is_safe(report.iter()) {
            total += 1;
        } else {
            // brute force solution as i'm not smart enough!
            for i in 0..report.len() {
                let itr = report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, v)| v);

                if is_safe(itr) {
                    total += 1;
                    break;
                }
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 2);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 4);
        Ok(())
    }
}
