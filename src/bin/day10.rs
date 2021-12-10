use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file = File::open("input/day10.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut corrupted_score = 0;
    let mut incomplete_scores = vec![];

    for line in lines {
        let line = line.unwrap();
        match parse_line(line.as_str()) {
            ParseError::Corrupted(c) => {
                corrupted_score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(),
                };
            }
            ParseError::Incomplete(chars) => {
                let mut score = 0;
                for c in chars {
                    score *= 5;
                    score += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!(),
                    };
                }
                incomplete_scores.push(score);
            }
        }
    }

    incomplete_scores.sort();

    println!("Part 1: {}", corrupted_score);
    println!("Part 2: {}", incomplete_scores[incomplete_scores.len() / 2]);
}

#[derive(Debug)]
enum ParseError {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn parse_line(input: &str) -> ParseError {
    let mut input = VecDeque::from_iter(input.chars());
    let mut stack = vec![];

    while let Some(first_char) = input.pop_front() {
        match first_char {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => {
                let c = stack.pop();
                if c.is_none() || c.unwrap() != first_char {
                    return ParseError::Corrupted(first_char)
                }
            }
        }
    }

    stack.reverse();
    ParseError::Incomplete(stack)
}
