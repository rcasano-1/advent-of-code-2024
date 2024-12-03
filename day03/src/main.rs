use regex::Regex;

const PART_1_EXAMPLE_INPUT: &str = include_str!("test-input.txt");
const PART_2_EXAMPLE_INPUT: &str = include_str!("test-input-part-2.txt");

fn part_1(input: &str) -> u64 {
    let pattern = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    let start_time = std::time::Instant::now();
    let matches = pattern
        .find_iter(input)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let mut multi_sum: u64 = 0;

    for pair in matches {
        let nums = pair
            .rsplit(['m', 'u', 'l', '(', ',', ')'])
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        multi_sum += nums[0] * nums[1];
    }

    let elapsed = start_time.elapsed();
    println!("Part 1 time: {:?}", elapsed);

    multi_sum
}

fn part_2(input: &str) -> u64 {
    let mul_pattern = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    let start_time = std::time::Instant::now();
    let mut multi_sum: u64 = 0;

    let mut data = input;
    let mut parts: Vec<&str> = Vec::new();

    while let Some((before_dont, rest)) = data.split_once("don't()") {
        parts.push(before_dont);

        data = if let Some((_dont_data, rest_after_do)) = rest.split_once("do()") {
            rest_after_do
        } else {
            break;
        }
    }
    if !data.is_empty() {
        parts.push(data);
    }

    for part in parts {
        let matches = mul_pattern
            .find_iter(part)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        for pair in matches {
            let nums = pair
                .rsplit(['m', 'u', 'l', '(', ',', ')'])
                .filter_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<u64>>();

            multi_sum += nums[0] * nums[1];
        }
    }

    let elapsed = start_time.elapsed();
    println!("Part 2 time: {:?}", elapsed);

    multi_sum
}

fn main() {
    println!("Part 1: {}", part_1(PART_1_EXAMPLE_INPUT));
    println!("Part 2: {}", part_2(PART_2_EXAMPLE_INPUT));
}
