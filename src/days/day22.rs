use std::ops::RangeInclusive;
use std::str::FromStr;

use hashbrown::HashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<(bool, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>)> {
    fn parse_step(input: &str) -> IResult<&str, (bool, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>)> {
        tuple((
            alt((
                map(tag("on"), |_| true),
                map(tag("off"), |_| false)
            )),
            preceded(tag(" x="), parse_range),
            preceded(tag(",y="), parse_range),
            preceded(tag(",z="), parse_range),
        ))(input)
    }

    fn parse_range(input: &str) -> IResult<&str, RangeInclusive<i32>> {
        map(
            separated_pair(
                parse_signed_int,
                tag(".."),
                parse_signed_int,
            ),
            |(start, end)| start..=end,
        )(input)
    }

    fn parse_signed_int<T: FromStr>(input: &str) -> IResult<&str, T> {
        map_res(
            recognize(tuple((opt(tag("-")), digit1))),
            FromStr::from_str,
        )(input)
    }

    separated_list1(
        newline,
        parse_step,
    )(input).unwrap().1
}

fn range_intersect(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> RangeInclusive<i32> {
    (*r1.start().max(r2.start()))..=(*r1.end().min(r2.end()))
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<(bool, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for (state, x_range, y_range, z_range) in input {
        for x in range_intersect(x_range.clone(), -50..=50) {
            for y in range_intersect(y_range.clone(), -50..=50) {
                for z in range_intersect(z_range.clone(), -50..=50) {
                    if *state {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len()
}
