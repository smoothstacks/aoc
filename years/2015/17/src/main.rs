const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", aoc2015day17::part1(INPUT)?);
    println!("{}", aoc2015day17::part2(INPUT)?);
    Ok(())
}
