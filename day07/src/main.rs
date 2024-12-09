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

fn main() {
    let input = parse_input(EXAMPLE);
    println!("{:?}", input);
}
