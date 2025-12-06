const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main()
}

#[divan::bench]
fn part1() {
    aoc2024day04::part1(INPUT).unwrap();
}
#[divan::bench]
fn part2() {
    aoc2024day04::part2(INPUT).unwrap();
}
