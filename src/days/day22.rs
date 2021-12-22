use std::str::FromStr;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};

#[derive(Copy, Clone, Debug)]
pub struct Cuboid {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);

        if x1 <= x2 && y1 <= y2 && z1 <= z2 {
            Some(Cuboid {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        } else {
            None
        }
    }

    fn sub(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut result = vec![];

        if let Some(intersection) = self.intersection(other) {
            // Split X
            let remainder = *self;
            if self.x1 < intersection.x1 {
                result.push({
                    let mut slice = remainder;
                    slice.x2 = intersection.x1 - 1;
                    slice
                });
            }
            if intersection.x2 < self.x2 {
                result.push({
                    let mut slice = remainder;
                    slice.x1 = intersection.x2 + 1;
                    slice
                });
            }

            // Split Y
            let remainder = {
                let mut slice = remainder;
                slice.x1 = intersection.x1;
                slice.x2 = intersection.x2;
                slice
            };
            if self.y1 < intersection.y1 {
                result.push({
                    let mut slice = remainder;
                    slice.y2 = intersection.y1 - 1;
                    slice
                });
            }
            if intersection.y2 < self.y2 {
                result.push({
                    let mut slice = remainder;
                    slice.y1 = intersection.y2 + 1;
                    slice
                });
            }

            // Split Z
            let remainder = {
                let mut slice = remainder;
                slice.y1 = intersection.y1;
                slice.y2 = intersection.y2;
                slice
            };
            if self.z1 < intersection.z1 {
                result.push({
                    let mut slice = remainder;
                    slice.z2 = intersection.z1 - 1;
                    slice
                });
            }
            if intersection.z2 < self.z2 {
                result.push({
                    let mut slice = remainder;
                    slice.z1 = intersection.z2 + 1;
                    slice
                });
            }
        } else {
            result.push(*self);
        }

        result
    }

    fn size(&self) -> usize {
        (1 + self.x2 - self.x1) as usize
            * (1 + self.y2 - self.y1) as usize
            * (1 + self.z2 - self.z1) as usize
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<(bool, Cuboid)> {
    fn parse_step(input: &str) -> IResult<&str, (bool, Cuboid)> {
        tuple((
            alt((
                map(tag("on"), |_| true),
                map(tag("off"), |_| false)
            )),
            map(tuple((
                preceded(tag(" x="), parse_range),
                preceded(tag(",y="), parse_range),
                preceded(tag(",z="), parse_range),
            )), |((x1, x2), (y1, y2), (z1, z2))| Cuboid {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        ))(input)
    }

    fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(
            parse_signed_int,
            tag(".."),
            parse_signed_int,
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

fn solve_both_parts(input: &Vec<(bool, Cuboid)>, part1: bool) -> usize {
    let mut on_cuboids = vec![];

    for (state, mut cuboid) in input {
        // Part 1: intersect cuboid with intersection area, skip if intersection is empty
        if part1 {
            if let Some(intersection) = cuboid.intersection(&Cuboid {
                x1: -50,
                x2: 50,
                y1: -50,
                y2: 50,
                z1: -50,
                z2: 50,
            }) {
                cuboid = intersection;
            } else {
                continue;
            }
        }

        if *state {
            // State: on
            let mut new_cuboids = vec![cuboid];

            for on_cuboid in &on_cuboids {
                new_cuboids = new_cuboids
                    .into_iter()
                    .flat_map(|new_cuboid| {
                        new_cuboid.sub(on_cuboid)
                    })
                    .collect_vec()
            }

            on_cuboids.extend(new_cuboids.into_iter());
        } else {
            // State: off
            on_cuboids = on_cuboids
                .into_iter()
                .flat_map(|on_cuboid| {
                    on_cuboid.sub(&cuboid)
                })
                .collect_vec()
        }
    }

    on_cuboids.into_iter().map(|c| c.size()).sum::<usize>()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<(bool, Cuboid)>) -> usize {
    solve_both_parts(input, true)
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<(bool, Cuboid)>) -> usize {
    solve_both_parts(input, false)
}
