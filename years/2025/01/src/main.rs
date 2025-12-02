const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", aoc2025day01::part1(INPUT)?);
    println!("{}", aoc2025day01::part2(INPUT)?);
    Ok(())
}
