use std::collections::BTreeSet;
use std::str::FromStr;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::{count, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

static ROTATION_MATRICES: [[[i32; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
];

fn multiply_vec3_mat3(v: &[i32; 3], m: &[[i32; 3]; 3]) -> [i32; 3] {
    [
        v[0] * m[0][0] + v[1] * m[1][0] + v[2] * m[2][0],
        v[0] * m[0][1] + v[1] * m[1][1] + v[2] * m[2][1],
        v[0] * m[0][2] + v[1] * m[1][2] + v[2] * m[2][2],
    ]
}

fn orientations(points: &Vec<[i32; 3]>) -> impl Iterator<Item=Vec<[i32; 3]>> + '_ {
    ROTATION_MATRICES
        .iter()
        .map(|m| {
            points
                .iter()
                .map(|p| multiply_vec3_mat3(p, m))
                .collect()
        })
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Vec<[i32; 3]>> {
    fn parse_input(input: &str) -> IResult<&str, Vec<Vec<[i32; 3]>>> {
        separated_list1(
            count(newline, 2),
            parse_report,
        )(input)
    }

    fn parse_report(input: &str) -> IResult<&str, Vec<[i32; 3]>> {
        // Parse header
        let (input, _) = terminated(
            delimited(tag("--- scanner "), digit1, tag(" ---")),
            newline,
        )(input)?;
        // Parse coordinates list
        separated_list1(newline, parse_coords)(input)
    }

    fn parse_coords(input: &str) -> IResult<&str, [i32; 3]> {
        map(
            tuple((
                terminated(parse_signed_int, tag(",")),
                terminated(parse_signed_int, tag(",")),
                parse_signed_int,
            )),
            |(x, y, z)| [x, y, z],
        )(input)
    }

    fn parse_signed_int<T: FromStr>(input: &str) -> IResult<&str, T> {
        map_res(
            recognize(tuple((opt(tag("-")), digit1))),
            FromStr::from_str,
        )(input)
    }

    parse_input(input).unwrap().1
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Vec<Vec<[i32; 3]>>) -> usize {
    let mut beacons = BTreeSet::<[i32; 3]>::new();
    beacons.extend(input[0].iter());

    let mut remaining_reports = Vec::from_iter(input[1..].iter());

    while !remaining_reports.is_empty() {
        for (index, &report) in remaining_reports.iter().enumerate() {
            if let Some((_, transformed_report)) = find_match(&beacons, report) {
                beacons.extend(transformed_report.into_iter());
                remaining_reports.swap_remove(index);
                break;
            }
        }
    }

    beacons.len()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Vec<Vec<[i32; 3]>>) -> i32 {
    let mut beacons = BTreeSet::<[i32; 3]>::new();
    beacons.extend(input[0].iter());

    let mut remaining_reports = Vec::from_iter(input[1..].iter());
    let mut scanners = vec![[0, 0, 0]];

    while !remaining_reports.is_empty() {
        for (index, &report) in remaining_reports.iter().enumerate() {
            if let Some((scanner, transformed_report)) = find_match(&beacons, report) {
                scanners.push(scanner);
                beacons.extend(transformed_report.into_iter());
                remaining_reports.swap_remove(index);
                break;
            }
        }
    }

    scanners.into_iter()
        .tuple_combinations::<(_, _)>()
        .map(|(s1, s2)| {
            (s1[0] - s2[0]).abs()
                + (s1[1] - s2[1]).abs()
                + (s1[2] - s2[2]).abs()
        })
        .max()
        .unwrap()
}

fn find_match(beacons: &BTreeSet<[i32; 3]>, report: &Vec<[i32; 3]>) -> Option<([i32; 3], Vec<[i32; 3]>)> {
    for rotated_report in orientations(report) {
        for pinned1 in beacons {
            for pinned2 in &rotated_report {
                let dx = pinned1[0] - pinned2[0];
                let dy = pinned1[1] - pinned2[1];
                let dz = pinned1[2] - pinned2[2];

                let transformed_report = rotated_report
                    .iter()
                    .map(|p| [
                        p[0] + dx,
                        p[1] + dy,
                        p[2] + dz,
                    ])
                    .collect::<Vec<_>>();

                let mut num_matches = 0;
                for transformed_beacon in &transformed_report {
                    if beacons.contains(transformed_beacon) {
                        num_matches += 1;
                    }

                    if num_matches >= 12 {
                        return Some(([dx, dy, dz], transformed_report));
                    }
                }
            }
        }
    }
    None
}
