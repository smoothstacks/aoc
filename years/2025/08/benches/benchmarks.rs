const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main()
}

#[divan::bench]
fn part1() {
    aoc2025day08::part1(INPUT).unwrap();
}
#[divan::bench]
fn part2() {
    aoc2025day08::part2(INPUT).unwrap();
}
