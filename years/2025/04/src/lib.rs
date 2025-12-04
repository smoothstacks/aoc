use aoc_util::chargrid::{CharGrid, CharGridVec};
use itertools::iproduct;

pub fn part1(input: &str) -> eyre::Result<u32> {
    let grid = CharGrid::new(input);
    Ok(grid
        .find_all('@')
        .filter(|p| {
            let cursor = grid.cursor(*p);
            let positions = iproduct!(-1..=1, -1..=1)
                .filter(|p| p.0 != 0 || p.1 != 0)
                .map(|(x, y)| CharGridVec(x, y));
            positions
                .filter(|p| cursor.peek(*p).is_some_and(|c| c == '@'))
                .count()
                < 4
        })
        .count() as u32)
}
pub fn part2(_: &str) -> eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_works() -> eyre::Result<()> {
        assert_eq!(super::part1(INPUT)?, 13);
        Ok(())
    }
    #[test]
    fn part2_works() -> eyre::Result<()> {
        assert_eq!(super::part2(INPUT)?, 0);
        Ok(())
    }
}
