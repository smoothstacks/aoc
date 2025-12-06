const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main()
}

#[divan::bench]
fn part1() {
    {{project-name}}::part1(INPUT).unwrap();
}
#[divan::bench]
fn part2() {
    {{project-name}}::part2(INPUT).unwrap();
}
