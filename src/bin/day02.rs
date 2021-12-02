use std::fs::File;
use std::io::Read;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline};
use nom::combinator::{map_opt, map_res};
use nom::multi::separated_list0;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() {
    let mut file = File::open("input/day02.txt").unwrap();
    let mut input = "".to_string();
    file.read_to_string(&mut input).unwrap();

    let (_, commands) = parse_commands(&input).unwrap();

    // Part 1
    let mut hor_pos = 0;
    let mut depth = 0;
    for command in &commands {
        match command {
            Command::Forward(dist) => hor_pos += dist,
            Command::Down(dist) => depth += dist,
            Command::Up(dist) => depth -= dist,
        }
    }
    println!("Part 1: {}", hor_pos * depth);

    // Part 2
    let mut aim = 0;
    let mut hor_pos = 0;
    let mut depth = 0;
    for command in &commands {
        match command {
            Command::Forward(dist) => {
                hor_pos += dist;
                depth += aim * dist;
            }
            Command::Down(dist) => aim += dist,
            Command::Up(dist) => aim -= dist,
        }
    }
    println!("Part 2: {}", hor_pos * depth);
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(newline, parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd_type) = map_opt(
        alpha1,
        |s: &str| -> Option<fn(usize) -> Command> {
            match s {
                "forward" => Some(Command::Forward),
                "down" => Some(Command::Down),
                "up" => Some(Command::Up),
                _ => None
            }
        },
    )(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, cmd_dist) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;

    Ok((input, cmd_type(cmd_dist)))
}
