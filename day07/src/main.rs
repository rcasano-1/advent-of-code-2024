const EXAMPLE: &str = include_str!("example.txt");

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.replace(":", "")
                .split(" ")
                .map(|num| num.parse().expect("Should be a valid u32 number"))
                .collect()
        })
        .collect()
}

// Part 1 functions
fn is_valid_equation(equation: &[u64], current_value: u64, index: usize) -> bool {
    let expected_test_value = equation[0];

    match equation.get(index + 1) {
        None => {
            // If the current value is equal to the expected result, equation is valid.
            if current_value == expected_test_value {
                return true;
            }

            false
        }
        Some(next_value) => {
            // If the current value is greater than the expected result, we can stop.
            if current_value > expected_test_value {
                return false;
            }

            // Recursion! -- "Iterate" over the equation, trying all combinations of operations and values.
            is_valid_equation(equation, current_value + next_value, index + 1)
                || is_valid_equation(equation, current_value * next_value, index + 1)
        }
    }
}

fn sum_valid_operations(equations: &[Vec<u64>]) -> u64 {
    equations
        .iter()
        .filter(|equation| is_valid_equation(equation, equation[1], 1))
        .map(|equation| equation[0])
        .sum()
}

fn part_1(input: &str) -> u64 {
    let equations = parse_input(input);

    let start_time = std::time::Instant::now();
    let sum = sum_valid_operations(&equations);
    println!("Part 1 time: {:?}", start_time.elapsed());

    sum
}

// Part 2 functions
fn concat(a: u64, b: u64) -> u64 {
    let mut multiplier = 10;

    while multiplier <= b {
        multiplier *= 10;
    }

    a * multiplier + b
}

fn is_valid_equation_with_concat(equation: &[u64], current_value: u64, index: usize) -> bool {
    let expected_result = equation[0];

    match equation.get(index + 1) {
        None => {
            if current_value == expected_result {
                return true;
            }

            false
        }
        Some(next_value) => {
            if current_value > expected_result {
                return false;
            }

            is_valid_equation_with_concat(equation, current_value + next_value, index + 1)
                || is_valid_equation_with_concat(equation, current_value * next_value, index + 1)
                || is_valid_equation_with_concat(
                    equation,
                    concat(current_value, *next_value),
                    index + 1,
                )
        }
    }
}

fn sum_valid_operations_with_concat(equations: &[Vec<u64>]) -> u64 {
    equations
        .iter()
        .filter(|equation| is_valid_equation_with_concat(equation, equation[1], 1))
        .map(|equation| equation[0])
        .sum()
}

fn part_2(input: &str) -> u64 {
    let equations = parse_input(input);

    let start_time = std::time::Instant::now();
    let sum = sum_valid_operations_with_concat(&equations);
    println!("Part 2 time: {:?}", start_time.elapsed());

    sum
}

fn main() {
    println!("Part 1: {}", part_1(EXAMPLE));

    println!("Part 2: {}", part_2(EXAMPLE));
}
