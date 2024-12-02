const EXAMPLE_INPUT: &str = include_str!("test-input.txt");

fn is_safe(report: &[u32]) -> bool {
    let mut is_ascending = false;
    let mut is_descending = false;

    for pair in report.windows(2) {
        match pair[0].cmp(&pair[1]) {
            std::cmp::Ordering::Less => is_ascending = true,
            std::cmp::Ordering::Greater => is_descending = true,
            std::cmp::Ordering::Equal => return false,
        }

        if pair[0].abs_diff(pair[1]) > 3 {
            return false;
        }
    }

    if is_ascending && is_descending {
        return false;
    }

    true
}

fn part_1(input: &str) -> u32 {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let start_time = std::time::Instant::now();

    let safe_reports = reports.iter().filter(|report| is_safe(report)).count() as u32;

    let elapsed = start_time.elapsed();
    println!("Part 1 time: {:?}", elapsed);

    safe_reports
}

fn part_2(input: &str) -> u32 {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let start_time = std::time::Instant::now();

    let mut safe_reports = 0;

    for report in reports {
        // Report is already safe without removing any levels
        if is_safe(&report) {
            safe_reports += 1;
        } else {
            // Try removing one level and check if the report is safe
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                if is_safe(&new_report) {
                    safe_reports += 1;
                    // Only need to count one level removal per report, if it makes the report safe
                    break;
                }
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("Part 2 time: {:?}", elapsed);

    safe_reports
}

fn main() {
    println!("Part 1: {}", part_1(EXAMPLE_INPUT));

    println!("Part 2: {}", part_2(EXAMPLE_INPUT));
}
