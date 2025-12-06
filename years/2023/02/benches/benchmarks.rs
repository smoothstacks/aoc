const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main()
}

#[divan::bench]
fn part1() {
    aoc2023day02::part1(INPUT).unwrap();
}
#[divan::bench]
fn part2() {
    aoc2023day02::part2(INPUT).unwrap();
}
