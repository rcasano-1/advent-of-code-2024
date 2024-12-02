const EXAMPLE_INPUT: &str = include_str!("test-input.txt");

fn parse_and_sort(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

fn part_1(input: &str) -> u64 {
    let (left, right) = parse_and_sort(input);

    let total_distance: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    total_distance
}

fn part_2(input: &str) -> u64 {
    let (left, right) = parse_and_sort(input);

    let total_similarity: u64 = left
        .iter()
        .map(|l| l * (right.iter().filter(|r| *r == l).count() as u64))
        .sum();

    total_similarity
}
fn main() {
    println!("Part 1 Solution: {}", part_1(EXAMPLE_INPUT));

    println!("Part 2 Solution: {}", part_2(EXAMPLE_INPUT));
}
