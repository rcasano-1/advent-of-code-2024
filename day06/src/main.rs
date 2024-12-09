use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");

// Common functions
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn starting_position(map: &[Vec<char>]) -> Option<(i32, i32)> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            return Some((x as i32, y as i32));
        }
    }
    None
}

// Part 1 functions
fn predict_guard_route(map: &[Vec<char>], (x, y): (i32, i32)) -> usize {
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let (mut dx, mut dy) = directions
        .next()
        .expect("Should have at least one direction");

    let mut distinct_positions: HashSet<(i32, i32)> = HashSet::from([(x, y)]);

    // Guard starting position
    let (mut x, mut y): (i32, i32) = (x, y);

    loop {
        let (nx, ny) = (x + dx, y + dy);

        let sym = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some(&c) => c,
            None => return distinct_positions.len(),
        };

        match sym {
            '#' => {
                (dx, dy) = *directions
                    .next()
                    .expect("Should have at least one direction")
            }
            '.' | '^' => {
                distinct_positions.insert((nx, ny));
                (x, y) = (nx, ny);
            }
            c => panic!("Unexpected character: {}", c),
        }
    }
}

fn part_1(input: &str) -> Result<usize, &str> {
    let map: Vec<Vec<char>> = parse_input(input);

    let start_time = std::time::Instant::now();
    let start: (i32, i32) = starting_position(&map).ok_or("Starting position not found")?;
    let result: usize = predict_guard_route(&map, start);
    println!("Part 1 time: {:?}", start_time.elapsed());

    Ok(result)
}

// Part 2 functions
fn record_guard_route(map: &[Vec<char>], (x, y): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let (mut dx, mut dy): &(i32, i32) = directions
        .next()
        .expect("Should have at least one direction");

    // Similar to predict_guard_route, but we record the guard's path
    let mut distinct_positions: HashSet<(i32, i32)> = HashSet::from([(x, y)]);
    let (mut x, mut y): (i32, i32) = (x, y);

    loop {
        let (nx, ny) = (x + dx, y + dy);

        let sym = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some(&c) => c,
            None => return distinct_positions,
        };

        match sym {
            '#' => {
                (dx, dy) = *directions
                    .next()
                    .expect("Should have at least one direction")
            }
            '.' | '^' => {
                distinct_positions.insert((nx, ny));
                (x, y) = (nx, ny);
            }
            c => panic!("Invalid symbol on the map: '{}'", c),
        }
    }
}

fn insert_obstacle_loops(
    map: &[Vec<char>],
    (x, y): (i32, i32),
    guard_route: &HashSet<(i32, i32)>,
) -> u64 {
    let mut loop_counter: u64 = 0;

    for obstacle in guard_route {
        let mut stop_conditions: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
        let (mut dx, mut dy) = directions
            .next()
            .expect("Should have at least one direction");
        let (mut x, mut y): (i32, i32) = (x, y);

        loop {
            let (nx, ny) = (x + dx, y + dy);

            let mut sym = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                Some(&c) => c,
                None => break,
            };

            if (nx, ny) == *obstacle {
                sym = 'O';
            }

            match sym {
                '#' | 'O' => {
                    if stop_conditions.contains(&((nx, ny), (dx, dy))) {
                        loop_counter += 1;
                        break;
                    }

                    stop_conditions.push(((nx, ny), (dx, dy)));
                    (dx, dy) = *directions
                        .next()
                        .expect("Should have at least one direction");
                }
                '.' | '^' => (x, y) = (nx, ny),
                c => panic!("Invalid symbol on the map: '{}'", c),
            }
        }
    }

    loop_counter
}

fn part_2(input: &str) -> Result<u64, &str> {
    let map: Vec<Vec<char>> = parse_input(input);

    let start_time = std::time::Instant::now();
    let start: (i32, i32) = starting_position(&map).ok_or("Starting position not found")?;
    let guard_route: HashSet<(i32, i32)> = record_guard_route(&map, start);
    let result: u64 = insert_obstacle_loops(&map, start, &guard_route);
    println!("Part 2 time: {:?}", start_time.elapsed());

    Ok(result)
}

fn main() {
    match part_1(EXAMPLE) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match part_2(EXAMPLE) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
