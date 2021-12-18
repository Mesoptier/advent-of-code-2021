#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    let number_size = input[0].len();

    let mut total_count = 0;
    let mut one_counts = vec![0; number_size];

    for number_string in input {
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

    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    let oxygen_generator_rating = search_part2_number(Vec::from(input), true);
    let co2_scrubber_rating = search_part2_number(Vec::from(input), false);
    oxygen_generator_rating * co2_scrubber_rating
}

fn search_part2_number(mut numbers: Vec<String>, keep_most_common: bool) -> u32 {
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

    u32::from_str_radix(numbers[0].as_str(), 2).unwrap()
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
