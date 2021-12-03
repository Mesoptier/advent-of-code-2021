use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day03.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let numbers: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let number_size = numbers[0].len();

    let mut total_count = 0;
    let mut one_counts = vec![0; number_size];

    for number_string in numbers.clone() {
        total_count += 1;

        for (i, c) in number_string.chars().enumerate() {
            match c {
                '1' => { one_counts[i] += 1; }
                _ => {}
            }
        }
    }

    let mut gamma_str: String = "".to_string();
    let mut epsilon_str: String = "".to_string();
    for i in 0..number_size {
        if one_counts[i] >= total_count - one_counts[i] {
            gamma_str = format!("{}1", gamma_str);
            epsilon_str = format!("{}0", epsilon_str);
        } else {
            gamma_str = format!("{}0", gamma_str);
            epsilon_str = format!("{}1", epsilon_str);
        }
    }

    let gamma = usize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = usize::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    println!("Part 1: {}", gamma * epsilon);

    let oxygen_generator_rating = search_part2_number(numbers.clone(), true);
    let co2_scrubber_rating = search_part2_number(numbers.clone(), false);
    println!("Part 2: {}", oxygen_generator_rating * co2_scrubber_rating);

}

fn search_part2_number(mut numbers: Vec<String>, keep_most_common: bool) -> usize {
    let mut pos = 0;
    while numbers.len() != 1 {
        let filter_char = if most_common_bit_in_position(&numbers, pos) == keep_most_common {
            '1'
        } else {
            '0'
        };

        numbers = numbers.into_iter()
            .filter(|number| {
                let c = number.as_bytes()[pos] as char;
                c == filter_char
            })
            .collect();

        pos += 1;
    }

    usize::from_str_radix(numbers[0].as_str(), 2).unwrap()
}

fn most_common_bit_in_position(numbers: &Vec<String>, pos: usize) -> bool {
    let total_count = numbers.len();
    let mut one_count = 0;

    for number in numbers {
        let c = number.as_bytes()[pos] as char;
        if c == '1' {
            one_count += 1;
        }
    }

    one_count >= total_count - one_count
}
