const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", aoc2024day06::part1(INPUT)?);
    println!("{}", aoc2024day06::part2(INPUT)?);
    Ok(())
}
