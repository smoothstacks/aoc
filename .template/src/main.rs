const INPUT: &str = include_str!("../input.txt");

fn main() -> eyre::Result<()> {
    println!("{}", {{project-name}}::part1(INPUT)?);
    println!("{}", {{project-name}}::part2(INPUT)?);
    Ok(())
}
