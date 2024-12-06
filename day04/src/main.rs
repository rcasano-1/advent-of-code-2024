const PART_1_EXAMPLE_INPUT: &str = include_str!("test-input.txt");
// const PART_1_EXAMPLE_INPUT_DOTS: &str = include_str!("test-input-dots.txt");
// const PART_2_EXAMPLE_INPUT_DOTS: &str = include_str!("test-input-dots-part-2.txt");

fn part_1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let start_time = std::time::Instant::now();
    let mut xmas_count = 0;

    for line in &lines {
        // Need line index to check vertial bounds
        let line_index = lines.iter().position(|l| l == line).unwrap();
        for (i, c) in line.chars().enumerate() {
            if c == 'X' {
                // Left -> Right (Horizontal)
                if (i <= line.chars().count() - 3)  // Check if we have enough space to go right (overflow)
                    && line.chars().nth(i + 1) == Some('M')
                    && line.chars().nth(i + 2) == Some('A')
                    && line.chars().nth(i + 3) == Some('S')
                {
                    xmas_count += 1;
                }

                // Left <- Right (Horizontal)
                if (i >= 3)
                    && line.chars().nth(i - 1) == Some('M')
                    && line.chars().nth(i - 2) == Some('A')
                    && line.chars().nth(i - 3) == Some('S')
                {
                    xmas_count += 1;
                }

                // Up (Vertical)
                if (line_index >= 3)   // Check if we have enough space to go up (overflow)
                    && lines.get(line_index - 1).and_then(|l| l.chars().nth(i)) == Some('M')
                    && lines.get(line_index - 2).and_then(|l| l.chars().nth(i)) == Some('A')
                    && lines.get(line_index - 3).and_then(|l| l.chars().nth(i)) == Some('S')
                {
                    xmas_count += 1;
                }

                // Down (Vertical)
                if (line_index <= (lines.len() - 4))  // Check if we have enough space to go down (overflow)
                    && lines.get(line_index + 1).and_then(|l| l.chars().nth(i)) == Some('M')
                    && lines.get(line_index + 2).and_then(|l| l.chars().nth(i)) == Some('A')
                    && lines.get(line_index + 3).and_then(|l| l.chars().nth(i)) == Some('S')
                {
                    xmas_count += 1;
                }

                // Up-Left (Diagonal)
                if (line_index >= 3)   // Check if we have enough space to go up (overflow)
                    && (i >= 3)  // Check if we have enough space to go left (overflow)
                    && lines.get(line_index - 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                    && lines.get(line_index - 2).and_then(|l| l.chars().nth(i - 2)) == Some('A')
                    && lines.get(line_index - 3).and_then(|l| l.chars().nth(i - 3)) == Some('S')
                {
                    xmas_count += 1;
                }

                // Up-Right (Diagonal)
                if (line_index >= 3)  // Check if we have enough space to go up (overflow)
                    && (i <= line.chars().count() - 4)  // Check if we have enough space to go right (overflow)
                    && lines.get(line_index - 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                    && lines.get(line_index - 2).and_then(|l| l.chars().nth(i + 2)) == Some('A')
                    && lines.get(line_index - 3).and_then(|l| l.chars().nth(i + 3)) == Some('S')
                {
                    xmas_count += 1;
                }

                // Down-Left (Diagonal)
                if (line_index <= (lines.len() - 3))  // Check if we have enough space to go down
                    && i >= 3
                    && lines.get(line_index + 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                    && lines.get(line_index + 2).and_then(|l| l.chars().nth(i - 2)) == Some('A')
                    && lines.get(line_index + 3).and_then(|l| l.chars().nth(i - 3)) == Some('S')
                {
                    xmas_count += 1;
                }

                // Down-Right (Diagonal)
                if (line_index <= (lines.len() - 3)) // Check if we have enough space to go down
                    && (i <= (line.chars().count() - 3))
                    && lines.get(line_index + 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                    && lines.get(line_index + 2).and_then(|l| l.chars().nth(i + 2)) == Some('A')
                    && lines.get(line_index + 3).and_then(|l| l.chars().nth(i + 3)) == Some('S')
                {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("Part 1 time: {:?}", start_time.elapsed());

    xmas_count
}

fn part_2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let start_time = std::time::Instant::now();
    let mut x_mas_count = 0;

    for line in &lines {
        // Need line index to check vertial bounds
        let line_index = lines.iter().position(|l| l == line).unwrap();
        for (i, c) in line.chars().enumerate() {
            // 'A' is always the middle character of "MAS"
            // Check if we have enough "space" to go up, down, left, right
            if c == 'A'
                && (line_index >= 1)
                && (i >= 1)
                && (line_index <= (lines.len() - 1))
                && (i <= (line.chars().count() - 1))
            {
                // Top corners of X are both "M"
                // "MAS" diagonal from left to right
                if lines.get(line_index - 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i + 1)) == Some('S')
                // "MAS" diagonal from right to left
                && lines.get(line_index - 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i - 1)) == Some('S')
                {
                    x_mas_count += 1;
                }

                // Top left corner of X is "M", top right corner is "S"
                // "MAS" diagonal from left to right
                if lines.get(line_index - 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i + 1)) == Some('S')
                // "SAM" diagonal from right to left
                && lines.get(line_index - 1).and_then(|l| l.chars().nth(i + 1)) == Some('S')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                {
                    x_mas_count += 1;
                }

                // Top left corner of X is "S", top right corner is "M"
                // "SAM" diagonal from left to right
                if lines.get(line_index - 1).and_then(|l| l.chars().nth(i - 1)) == Some('S')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                // "MAS" diagonal from right to left
                && lines.get(line_index - 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i - 1)) == Some('S')
                {
                    x_mas_count += 1;
                }

                // Bottom corners of X are both "M"
                // "SAM" diagonal from left to right
                if lines.get(line_index - 1).and_then(|l| l.chars().nth(i - 1)) == Some('S')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i + 1)) == Some('M')
                // "SAM" diagonal from right to left
                && lines.get(line_index - 1).and_then(|l| l.chars().nth(i + 1)) == Some('S')
                && lines.get(line_index + 1).and_then(|l| l.chars().nth(i - 1)) == Some('M')
                {
                    x_mas_count += 1;
                }
            }
        }
    }

    println!("Part 2 time: {:?}", start_time.elapsed());

    x_mas_count
}

fn main() {
    println!("Part 1: {}", part_1(PART_1_EXAMPLE_INPUT));
    // println!("Part 1: {}", part_1(PART_1_EXAMPLE_INPUT_DOTS));

    println!("Part 2: {}", part_2(PART_1_EXAMPLE_INPUT));
    // println!("Part 2: {}", part_2(PART_2_EXAMPLE_INPUT_DOTS));
}
