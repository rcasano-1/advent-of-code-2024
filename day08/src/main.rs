use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn group_by_same_frequency(antenna_map: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut hash_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in antenna_map.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char != '.' {
                hash_map.entry(char).or_default().push((x as i32, y as i32));
            }
        }
    }

    hash_map
}

fn count_antinodes(map: &HashMap<char, Vec<(i32, i32)>>, height: i32, width: i32) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // Iterate over all antenna positions
    for antenna_positions in map.values() {
        for (x1, y1) in antenna_positions {
            for (x2, y2) in antenna_positions {
                // Check if the two antenna positions are different
                if (x1, y1) != (x2, y2) {
                    // Calculate the antinode position
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    let (nx, ny) = (x1 + dx, y1 + dy);

                    // Check if the antinode is within the bounds of the map
                    if nx >= 0 && ny >= 0 && nx < width && ny < height {
                        antinodes.insert((nx, ny));
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn part_1(input: &str) -> usize {
    let antenna_map = parse_input(input);

    let start_time = std::time::Instant::now();
    let map = group_by_same_frequency(&antenna_map);
    let antinodes = count_antinodes(&map, antenna_map.len() as i32, antenna_map[0].len() as i32);
    println!("Part 1 time: {:?}", start_time.elapsed());

    antinodes
}

fn count_antinodes_any_position(
    map: &HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // Iterate over all antenna positions
    for antenna_positions in map.values() {
        for &(x1, y1) in antenna_positions {
            for &(x2, y2) in antenna_positions {
                // Check if the two antenna positions are different
                if (x1, y1) != (x2, y2) {
                    // Calculate distance between two antenna positions
                    let (dx, dy) = (x1 - x2, y1 - y2);

                    // Get the current antinode position
                    let (mut nx, mut ny) = (x1, y1);

                    // Check if the antinode is within the bounds of the map
                    while nx >= 0 && ny >= 0 && nx < width && ny < height {
                        antinodes.insert((nx, ny));
                        // Calculate the next antinode position in line with the two antenna positions
                        (nx, ny) = (nx + dx, ny + dy);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part_2(input: &str) -> usize {
    let antenna_map = parse_input(input);

    let start_time = std::time::Instant::now();
    let map = group_by_same_frequency(&antenna_map);
    let antinodes =
        count_antinodes_any_position(&map, antenna_map.len() as i32, antenna_map[0].len() as i32);
    println!("Part 2 time: {:?}", start_time.elapsed());

    antinodes
}

fn main() {
    println!("Part 1: {}", part_1(EXAMPLE));
    println!("Part 1: {}", part_1(INPUT));

    println!("Part 2: {}", part_2(EXAMPLE));
    println!("Part 2: {}", part_2(INPUT));
}
