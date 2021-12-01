use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let depths: Vec<u64> = lines.map(|line| line.unwrap().parse::<u64>().unwrap()).collect();

    let mut increase_count_part1 = 0;
    let mut increase_count_part2 = 0;

    for (index, &depth) in depths.iter().enumerate() {
        if index >= 1 && depths[index - 1] < depth {
            increase_count_part1 += 1;
        }
        if index >= 3 && depths[index - 3] < depth {
            increase_count_part2 += 1;
        }
    }

    println!("Part 1: {}", increase_count_part1);
    println!("Part 2: {}", increase_count_part2);
}
