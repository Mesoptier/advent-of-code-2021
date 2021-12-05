use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    let mut file = File::open("input/day05.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let (_, lines) = parse_input(input.as_str()).unwrap();

    let mut counts_part1: HashMap<(i32, i32), usize> = HashMap::new();
    let mut counts_part2: HashMap<(i32, i32), usize> = HashMap::new();

    for &((x1, y1), (x2, y2)) in &lines {
        let dx = match x1.cmp(&x2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let dy = match y1.cmp(&y2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        let len = (x1 - x2).abs().max((y1 - y2).abs());

        for i in 0..=len {
            let x = x1 + i * dx;
            let y = y1 + i * dy;

            // Only consider horizontal and vertical lines for part 1
            if x1 == x2 || y1 == y2 {
                counts_part1.insert((x, y), match counts_part1.get(&(x, y)) {
                    None => 1,
                    Some(n) => n + 1
                });
            }

            counts_part2.insert((x, y), match counts_part2.get(&(x, y)) {
                None => 1,
                Some(n) => n + 1
            });
        }
    }

    let mut part1 = 0;
    for (_, count) in counts_part1 {
        if count >= 2 {
            part1 += 1;
        }
    }

    let mut part2 = 0;
    for (_, count) in counts_part2 {
        if count >= 2 {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    separated_list1(
        newline,
        parse_line,
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    separated_pair(
        parse_coords,
        tag(" -> "),
        parse_coords,
    )(input)
}

fn parse_coords(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        parse_number,
        tag(","),
        parse_number,
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)
}
