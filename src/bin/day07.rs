use std::fs::File;
use std::io::Read;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

fn main() {
    let mut file = File::open("input/day07.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, positions) = parse_positions(input).unwrap();
    let &min_pos = positions.iter().min().unwrap();
    let &max_pos = positions.iter().max().unwrap();

    println!("Part 1: {}", find_min_fuel(min_pos, max_pos, &positions, |s: usize, g: usize| {
        if s < g {
            g - s
        } else {
            s - g
        }
    }));

    println!("Part 2: {}", find_min_fuel(min_pos, max_pos, &positions, |s: usize, g: usize| {
        let dist = if s < g {
            g - s
        } else {
            s - g
        };
        (dist * (dist + 1)) / 2
    }));
}

fn find_min_fuel<F>(min_pos: usize, max_pos: usize, positions: &Vec<usize>, calc_fuel: F) -> usize
    where F: Fn(usize, usize) -> usize
{
    (min_pos..=max_pos).into_iter()
        .map(|goal_pos| {
            positions.iter()
                .map(|&start_pos| {
                    calc_fuel(start_pos, goal_pos)
                })
                .sum::<usize>()
        })
        .min().unwrap()
}

fn parse_positions(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(
        tag(","),
        map(digit1, |s: &str| s.parse::<usize>().unwrap()),
    )(input)
}
