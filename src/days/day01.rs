#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.parse::<_>().unwrap()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut increase_count = 0;
    for index in 0..input.len() {
        if index >= 1 && input[index - 1] < input[index] {
            increase_count += 1;
        }
    }
    increase_count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut increase_count = 0;
    for index in 0..input.len() {
        if index >= 3 && input[index - 3] < input[index] {
            increase_count += 1;
        }
    }
    increase_count
}
