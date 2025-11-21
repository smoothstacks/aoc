use glam::I64Vec2;
use std::collections::HashSet;

fn directions(input: &str) -> impl Iterator<Item = I64Vec2> + use<'_> {
    input.chars().map(|c| match c {
        '^' => I64Vec2::Y,
        '>' => I64Vec2::X,
        'v' => I64Vec2::NEG_Y,
        '<' => I64Vec2::NEG_X,
        _ => I64Vec2::ZERO,
    })
}

fn visited(directions: impl Iterator<Item = I64Vec2>) -> HashSet<I64Vec2> {
    let mut pos = I64Vec2::new(0, 0);
    let mut seen = HashSet::new();
    for dir in directions {
        seen.insert(pos);
        pos += dir;
    }
    seen
}

pub fn part1(input: &str) -> eyre::Result<u32> {
    let seen = visited(directions(input));

    Ok(seen.len() as u32)
}
pub fn part2(input: &str) -> eyre::Result<u32> {
    let santa_directions = directions(input).step_by(2);
    let santa_seen = visited(santa_directions);
    let robot_directions = directions(input).skip(1).step_by(2);
    let robo_seen = visited(robot_directions);

    Ok(santa_seen.union(&robo_seen).count() as u32)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "^>v<";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 4);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 3);
        Ok(())
    }
}
