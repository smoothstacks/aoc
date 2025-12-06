const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main()
}

#[divan::bench]
fn part1() {
    aoc2015day07::part1(INPUT).unwrap();
}
#[divan::bench]
fn part2() {
    aoc2015day07::part2(INPUT).unwrap();
}
