use std::fs::File;
use std::io::Read;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

fn main() {
    let mut file = File::open("input/day06.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, timers) = parse_timers(input).unwrap();
    let mut timer_counts = [0usize; 9];
    for timer in timers {
        timer_counts[timer] += 1;
    }

    for day in 0..256 {
        if day == 80 {
            println!("Part 1: {}", timer_counts.iter().sum::<usize>());
        }

        timer_counts.rotate_left(1);
        timer_counts[6] += timer_counts[8];
    }

    println!("Part 2: {}", timer_counts.iter().sum::<usize>());
}

fn parse_timers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(
        tag(","),
        map(digit1, |s: &str| s.parse::<usize>().unwrap())
    )(input)
}
