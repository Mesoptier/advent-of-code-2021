use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    let mut file = File::open("input/day08.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, notes) = parse_input(input).unwrap();

    let mut easy_count = 0;
    let mut output_sum = 0;

    for (patterns, outputs) in &notes {
        let mut digit_to_pattern: HashMap<usize, HashSet<char>> = HashMap::new();

        // Find 1, 4, 7, 8
        for &pattern in patterns {
            let pattern: HashSet<char> = HashSet::from_iter(pattern.chars());
            if let Some(digit) = match pattern.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None
            } {
                digit_to_pattern.insert(digit, pattern.clone());
            }
        }

        // Find 9, 0, 6
        for &pattern in patterns {
            let pattern: HashSet<char> = HashSet::from_iter(pattern.chars());
            if pattern.len() == 6 {
                if pattern.is_superset(&digit_to_pattern[&4]) {
                    digit_to_pattern.insert(9, pattern.clone());
                } else if pattern.is_superset(&digit_to_pattern[&7]) {
                    digit_to_pattern.insert(0, pattern.clone());
                } else {
                    digit_to_pattern.insert(6, pattern.clone());
                }
            }
        }

        // Find 3, 5, 2
        for &pattern in patterns {
            let pattern: HashSet<char> = HashSet::from_iter(pattern.chars());
            if pattern.len() == 5 {
                if pattern.is_superset(&digit_to_pattern[&1]) {
                    digit_to_pattern.insert(3, pattern.clone());
                } else if pattern.is_subset(&digit_to_pattern[&6]) {
                    digit_to_pattern.insert(5, pattern.clone());
                } else {
                    digit_to_pattern.insert(2, pattern.clone());
                }
            }
        }

        let mut output_value = 0;
        let mut pos = 1000;
        for &output in outputs.iter() {
            let output: HashSet<char> = HashSet::from_iter(output.chars());
            match output.len() {
                2 | 3 | 4 | 7 => {
                    easy_count += 1;
                }
                _ => {}
            }

            let digit = digit_to_pattern.iter().find_map(|(digit, pattern)| {
                if pattern.clone() == output {
                    Some(digit)
                } else {
                    None
                }
            }).unwrap();
            output_value += pos * digit;
            pos /= 10;
        }

        output_sum += output_value;
    }

    println!("Part 1: {}", easy_count);
    println!("Part 2: {}", output_sum);
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list1(
        newline,
        separated_pair(
            separated_list1(tag(" "), alpha1),
            tag(" | "),
            separated_list1(tag(" "), alpha1),
        ),
    )(input)
}
