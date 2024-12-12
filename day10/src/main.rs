const EXAMPLE: &str = include_str!("example.txt");

// Common functions
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be a digit between 0 and 9") as u8)
                .collect()
        })
        .collect()
}

fn get_trailheads(topographic_map: &[Vec<u8>]) -> Vec<[usize; 2]> {
    topographic_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &cell)| {
                    if cell == 0 {
                        Some([x, y])
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

// Returns valid next positions for a given target position, counting by 1 from 1 to 9
fn get_valid_neighbor_positions(
    map: &[Vec<u8>],
    [x, y]: [usize; 2],
    target: u8,
) -> Vec<[usize; 2]> {
    [
        [x + 1, y],             // Right
        [x, y + 1],             // Down
        [x.wrapping_sub(1), y], // Left
        [x, y.wrapping_sub(1)], // Up
    ]
    .into_iter()
    .filter_map(|[nx, ny]| {
        map.get(ny)
            .and_then(|row| row.get(nx))
            .filter(|&&c| c == target)
            .map(|_| [nx, ny])
    })
    .collect()
}

// Part 1 functions
fn calculate_trailhead_scores(topographic_map: &[Vec<u8>]) -> usize {
    let mut trailhead_scores: usize = 0;
    let starting_positions: Vec<[usize; 2]> = get_trailheads(topographic_map);

    for starting_position in &starting_positions {
        let mut valid_positions: Vec<[usize; 2]> = Vec::from([*starting_position]);

        // Start from 1 because we already have the trailhead
        for target in 1_u8..=9 {
            let mut next_valid_positions: Vec<[usize; 2]> = valid_positions
                .into_iter()
                .flat_map(|[x, y]| get_valid_neighbor_positions(topographic_map, [x, y], target))
                .collect();

            // Sort "trail"
            next_valid_positions.sort_unstable();
            // If the "elevation" remains the same between two positions, we can remove "duplicates"
            next_valid_positions.dedup();
            valid_positions = next_valid_positions;
        }

        // Count the number of valid positions or "trails" for this trailhead
        trailhead_scores += valid_positions.len();
    }

    trailhead_scores
}

fn part_1(input: &str) -> usize {
    let topographic_map: Vec<Vec<u8>> = parse_input(input);

    let start_time = std::time::Instant::now();
    let sum_scores = calculate_trailhead_scores(&topographic_map);
    println!("Part 1 time: {:?}", start_time.elapsed());

    sum_scores
}

// Part 2 functions
fn traverse(topographic_map: &[Vec<u8>], [x, y]: [usize; 2], target: u8) -> u64 {
    // If we reach the end of the trail, return 1
    if topographic_map[y][x] == 9 {
        return 1;
    }

    get_valid_neighbor_positions(topographic_map, [x, y], target)
        .iter()
        .map(|&[x, y]| traverse(topographic_map, [x, y], target + 1))
        .sum()
}

fn calculate_trailhead_ratings(topographic_map: &[Vec<u8>]) -> u64 {
    get_trailheads(topographic_map)
        .iter()
        .map(|&[x, y]| traverse(topographic_map, [x, y], 1))
        .sum()
}

fn part_2(input: &str) -> u64 {
    let topographic_map: Vec<Vec<u8>> = parse_input(input);

    let start_time = std::time::Instant::now();
    let sum_ratings = calculate_trailhead_ratings(&topographic_map);
    println!("Part 2 time: {:?}", start_time.elapsed());

    sum_ratings
}

fn main() {
    println!("Part 1: {}", part_1(EXAMPLE));

    println!("Part 2: {}", part_2(EXAMPLE));
}
