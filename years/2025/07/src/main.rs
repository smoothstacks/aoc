const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", aoc2025day07::part1(INPUT)?);
    println!("{}", aoc2025day07::part2(INPUT)?);
    Ok(())
}
