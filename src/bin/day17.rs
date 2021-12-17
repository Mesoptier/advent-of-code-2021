use std::cmp::max;
use std::fs::File;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt, recognize};
use nom::IResult;
use nom::sequence::{preceded, separated_pair, tuple};

fn main() {
    let mut file = File::open("input/day17.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, ((x_min, x_max), (y_min, y_max))) = parse_input(input).unwrap();

    let mut overall_highest_y = 0;
    let mut num_valid_initial_velocities = 0;

    for initial_vy in y_min..-y_min {
        for initial_vx in 0..=max(0, x_max) {
            let mut y = 0;
            let mut vy = initial_vy;

            let mut x = 0;
            let mut vx = initial_vx;

            let mut highest_y = 0;

            while x <= x_max && y >= y_min {
                x += vx;
                y += vy;
                vx = max(0, vx - 1);
                vy -= 1;

                highest_y = max(highest_y, y);

                if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                    num_valid_initial_velocities += 1;
                    overall_highest_y = max(overall_highest_y, highest_y);
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", overall_highest_y);
    println!("Part 2: {}", num_valid_initial_velocities);
}


fn parse_input(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    preceded(
        tag("target area: "),
        separated_pair(
            preceded(tag("x="), parse_range),
            tag(", "),
            preceded(tag("y="), parse_range),
        ),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_number, tag(".."), parse_number)(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map(
        recognize(tuple((
            opt(tag("-")),
            digit1
        ))),
        |s: &str| s.parse::<i32>().unwrap(),
    )(input)
}
