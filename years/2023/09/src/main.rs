const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", aoc2023day09::part1(INPUT)?);
    println!("{}", aoc2023day09::part2(INPUT)?);
    Ok(())
}
