const EXAMPLE: &str = include_str!("example.txt");

// Common parse function
fn parse_input(input: &str) -> Vec<u32> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).expect("Should be a digit between 0 and 9"))
        .collect::<Vec<_>>()
}

// Part 1 functions
fn unpack_disk_map(disk_map: &[u32]) -> Vec<Option<u32>> {
    let mut unpacked_information: Vec<Option<u32>> = Vec::new();
    let mut file_id: u32 = 0;
    let mut is_file = true;

    for &size in disk_map {
        let item: Option<u32> = is_file.then_some(file_id);
        unpacked_information.extend(std::iter::repeat_n(item, size as usize));
        file_id += is_file as u32;
        is_file = !is_file;
    }

    unpacked_information
}

fn fragment_unpacked_information(unpacked_information: &mut [Option<u32>]) {
    let mut free_space_index: usize = 0;
    let mut occupied_space_index: usize = unpacked_information.len() - 1;

    loop {
        while unpacked_information[free_space_index].is_some() {
            free_space_index += 1;
        }

        while unpacked_information[occupied_space_index].is_none() {
            occupied_space_index -= 1;
        }

        if free_space_index > occupied_space_index {
            break;
        }

        unpacked_information.swap(free_space_index, occupied_space_index);
    }
}

fn calculate_checksum(fragmented_information: &[Option<u32>]) -> u64 {
    fragmented_information
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 * value.unwrap_or(0)) as u64)
        .sum()
}

fn part_1(input: &str) -> u64 {
    let disk_map = parse_input(input);

    let start_time = std::time::Instant::now();
    let mut unpacked_disk_map = unpack_disk_map(&disk_map);
    fragment_unpacked_information(&mut unpacked_disk_map);
    let checksum = calculate_checksum(&unpacked_disk_map);

    println!("Part 1 time: {:?}", start_time.elapsed());

    checksum
}

// Part 2 functions
fn unpack_information(disk_map: &[u32]) -> [Vec<(u32, u32)>; 2] {
    let mut files: Vec<(u32, u32)> = Vec::new();
    let mut free_space: Vec<(u32, u32)> = Vec::new();

    let mut next_index: u32 = 0;
    let mut is_file = true;

    for &size in disk_map {
        if is_file {
            files.push((next_index, size));
        } else if size != 0 {
            free_space.push((next_index, size));
        }

        next_index += size;
        is_file = !is_file;
    }

    [files, free_space]
}

fn move_whole_files(files: &mut [(u32, u32)], free_space: &mut [(u32, u32)]) {
    for (file_start, file_length) in files.iter_mut().rev() {
        for (free_space_start, free_space_length) in free_space.iter_mut() {
            if *file_start < *free_space_start {
                break;
            }

            if *file_length > *free_space_length {
                continue;
            }

            *file_start = *free_space_start;

            if *file_length < *free_space_length {
                *free_space_start += *file_length;
                *free_space_length -= *file_length;
            } else {
                *free_space_length = 0;
            }
        }
    }
}

fn calculate_checksum_moved(files: &[(u32, u32)]) -> u64 {
    files
        .iter()
        .enumerate()
        .map(|(index, &(file_start, file_length))| {
            (file_start..(file_start + file_length)).sum::<u32>() as u64 * index as u64
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let disk_map = parse_input(input);

    let start_time = std::time::Instant::now();
    let [mut files, mut free_space] = unpack_information(&disk_map);
    move_whole_files(&mut files, &mut free_space);
    let checksum = calculate_checksum_moved(&files);

    println!("Part 2 time: {:?}", start_time.elapsed());

    checksum
}

fn main() {
    println!("Part 1: {}", part_1(EXAMPLE));

    println!("Part 2: {}", part_2(EXAMPLE));
}
