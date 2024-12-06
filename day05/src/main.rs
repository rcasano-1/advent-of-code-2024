use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut lines = input.lines();

    let page_ordering_rules: Vec<(u32, u32)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").expect("Should contain '|' character"))
        .map(|(num_1, num_2)| {
            (
                num_1.parse::<u32>().expect("Should be a valid u32 number"),
                num_2.parse::<u32>().expect("Should be a valid u32 number"),
            )
        })
        .collect();

    let updates: Vec<Vec<u32>> = lines
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u32>().expect("Should be a valid u32 number"))
                .collect()
        })
        .collect();

    (page_ordering_rules, updates)
}

fn build_page_ordering_map(page_ordering_rules: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    page_ordering_rules
        .iter()
        .fold(HashMap::new(), |mut map, &(key, value)| {
            map.entry(key).or_default().push(value);
            map.entry(value).or_default();
            map
        })
}

fn is_correct_ordering(page_ordering_map: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    let mut current_page = update[0];
    let mut is_correct_ordering = true;

    for next_page in update.iter().skip(1) {
        if let Some(possible_next_pages) = page_ordering_map.get(&current_page) {
            if !possible_next_pages.contains(next_page) {
                is_correct_ordering = false;
                break;
            }
        } else {
            is_correct_ordering = false;
            break;
        }
        current_page = *next_page;
    }
    is_correct_ordering
}

// Part 1
fn sum_middle_pages_of_correctly_ordered_updates(
    page_ordering_map: &HashMap<u32, Vec<u32>>,
    updates: &[Vec<u32>],
) -> u32 {
    updates
        .iter()
        .filter(|update| is_correct_ordering(page_ordering_map, update))
        .map(|correct_order_update| correct_order_update[correct_order_update.len() / 2])
        .sum()
}

fn part_1(input: &str) -> u32 {
    let (page_ordering_rules, updates) = parse_input(input);
    let page_ordering_map = build_page_ordering_map(&page_ordering_rules);
    sum_middle_pages_of_correctly_ordered_updates(&page_ordering_map, &updates)
}

// Part 2
fn sort_updates(update_to_fix: &[u32], page_ordering_map: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut sorted_update = update_to_fix.to_vec();

    sorted_update.sort_by(|a, b| {
        match page_ordering_map
            .get(a)
            .expect("Should be a valid page key.")
            .contains(b)
        {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        }
    });

    sorted_update
}

fn sum_middle_pages_of_sorted_invalid_updates(
    updates: &[Vec<u32>],
    page_ordering_map: &HashMap<u32, Vec<u32>>,
) -> u32 {
    let mut sum = 0;

    for update in updates {
        if !is_correct_ordering(page_ordering_map, update) {
            let sorted_update = sort_updates(update, page_ordering_map);
            sum += sorted_update[sorted_update.len() / 2];
        }
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let (page_ordering_rules, updates) = parse_input(input);
    let page_ordering_map = build_page_ordering_map(&page_ordering_rules);
    sum_middle_pages_of_sorted_invalid_updates(&updates, &page_ordering_map)
}

fn main() {
    println!("Part 1 - Sum of middle pages: {}", part_1(EXAMPLE));
    println!(
        "Part 2 - Sum of middle fixed invalid pages: {}",
        part_2(EXAMPLE)
    );
}
