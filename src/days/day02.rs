use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline};
use nom::combinator::{map_opt, map_res};
use nom::multi::separated_list0;

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
        separated_list0(newline, parse_command)(input)
    }

    fn parse_command(input: &str) -> IResult<&str, Command> {
        let (input, cmd_type) = map_opt(
            alpha1,
            |s: &str| -> Option<fn(u32) -> Command> {
                match s {
                    "forward" => Some(Command::Forward),
                    "down" => Some(Command::Down),
                    "up" => Some(Command::Up),
                    _ => None
                }
            },
        )(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, cmd_dist) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;

        Ok((input, cmd_type(cmd_dist)))
    }

    parse_commands(input).unwrap().1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> u32 {
    let mut hor_pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(dist) => hor_pos += dist,
            Command::Down(dist) => depth += dist,
            Command::Up(dist) => depth -= dist,
        }
    }
    hor_pos * depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> u32 {
    let mut aim = 0;
    let mut hor_pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(dist) => {
                hor_pos += dist;
                depth += aim * dist;
            }
            Command::Down(dist) => aim += dist,
            Command::Up(dist) => aim -= dist,
        }
    }
    hor_pos * depth
}
