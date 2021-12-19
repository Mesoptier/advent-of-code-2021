use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::{count, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

static ROTATION_MATRICES: [Matrix3<i32>; 24] = [
    Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
    Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
    Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
    Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
    Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
    Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
    Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
    Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
    Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
    Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
    Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
    Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
    Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
    Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
    Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
    Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
    Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
];

fn norm_l1(v: &Vector3<i32>) -> i32 {
    v[0].abs() + v[1].abs() + v[2].abs()
}

fn norm_lmax(v: &Vector3<i32>) -> i32 {
    v[0].abs()
        .max(v[1].abs())
        .max(v[2].abs())
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Vec<Vector3<i32>>> {
    fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Vector3<i32>>>> {
        separated_list1(
            count(newline, 2),
            parse_report,
        )(input)
    }

    fn parse_report(input: &str) -> IResult<&str, Vec<Vector3<i32>>> {
        // Parse header
        let (input, _) = terminated(
            delimited(tag("--- scanner "), digit1, tag(" ---")),
            newline,
        )(input)?;
        // Parse coordinates list
        separated_list1(newline, parse_coords)(input)
    }

    fn parse_coords(input: &str) -> IResult<&str, Vector3<i32>> {
        map(
            tuple((
                terminated(parse_signed_int, tag(",")),
                terminated(parse_signed_int, tag(",")),
                parse_signed_int,
            )),
            |(x, y, z)| [x, y, z].into(),
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

type Fingerprint = (i32, i32);

fn fingerprint(p1: &Vector3<i32>, p2: &Vector3<i32>) -> Fingerprint {
    let d = p1 - p2;
    (norm_l1(&d), norm_lmax(&d))
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Vec<Vec<Vector3<i32>>>) -> usize {
    solve_both(input).0
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Vec<Vec<Vector3<i32>>>) -> i32 {
    solve_both(input).1
}

fn solve_both(input: &Vec<Vec<Vector3<i32>>>) -> (usize, i32) {
    let mut known_beacons = HashSet::<Vector3<i32>>::new();
    known_beacons.extend(input[0].iter());

    // NOTE: This does not contain fingerprints for all point-pairs (across report boundaries), only
    // for point-pairs within reports.
    // TODO: Might contain duplicate pairs with swapped points?
    let mut known_fingerprints = HashMap::<Fingerprint, HashSet<[Vector3<i32>; 2]>>::new();
    for (p1, p2) in input[0].iter().tuple_combinations::<(_, _)>() {
        let f = fingerprint(p1, p2);
        known_fingerprints.entry(f).or_default().insert([*p1, *p2]);
    }

    let mut scanners = vec![[0, 0, 0].into()];

    // consider fingerprints of point pairs
    // known_fingerprints: map from fingerprint to list of point pairs with that fingerprint
    // find_match(report):
    //      compute map of fingerprints to lists of point pairs in the report
    //      if fewer than 12 of these fingerprints are in known_fingerprints -> skip report
    //      use the two maps of fingerprints to point pairs as candidate alignments for brute-force match checking
    //      IDEA: a pair of these point pairs could be used to reduce possible rotations to 2

    let mut remaining_reports = Vec::from_iter(input[1..].iter());

    while !remaining_reports.is_empty() {
        for (index, &report) in remaining_reports.iter().enumerate() {
            if let Some((scanner, transformed_report)) = find_match(&known_beacons, &known_fingerprints, report) {
                // Extend known_fingerprints with transformed points
                for (p1, p2) in transformed_report.iter().tuple_combinations::<(_, _)>() {
                    let f = fingerprint(p1, p2);
                    known_fingerprints.entry(f).or_default().insert([*p1, *p2]);
                }

                scanners.push(scanner);
                known_beacons.extend(transformed_report.into_iter());
                remaining_reports.swap_remove(index);
                break;
            }
        }
    }

    let scanner_range = scanners.into_iter()
        .tuple_combinations::<(_, _)>()
        .map(|(s1, s2)| norm_l1(&(s1 - s2)))
        .max()
        .unwrap();

    (
        // Part 1
        known_beacons.len(),
        // Part 2
        scanner_range
    )
}

fn find_match(
    beacons: &HashSet<Vector3<i32>>,
    known_fingerprints: &HashMap<Fingerprint, HashSet<[Vector3<i32>; 2]>>,
    report: &Vec<Vector3<i32>>,
) -> Option<(Vector3<i32>, Vec<Vector3<i32>>)> {
    // TODO: Compute report fingerprints in advance, instead of repeating this every iteration
    let matching_fingerprints = report
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| {
            (fingerprint(p1, p2), [*p1, *p2])
        })
        .filter(|(f, _)| {
            known_fingerprints.contains_key(f)
        })
        .collect::<Vec<_>>();

    // TODO: Check that 12+ fingerprints support the same rotation
    if matching_fingerprints.len() < 12 {
        return None;
    }

    for (f, report_pair) in matching_fingerprints {
        for known_pair in known_fingerprints.get(&f).unwrap() {
            let supported_rotations = ROTATION_MATRICES.iter()
                .filter(|&m| {
                    known_pair[0] - m * report_pair[0] == known_pair[1] - m * report_pair[1]
                        || known_pair[0] - m * report_pair[1] == known_pair[1] - m * report_pair[0]
                })
                .collect::<Vec<_>>();

            for (report_pinned, known_pinned) in report_pair.iter().cartesian_product(known_pair) {
                for &m in &supported_rotations {
                    let report_pinned = m * report_pinned;

                    let translation = known_pinned - report_pinned;

                    let transformed_report = report
                        .iter()
                        .map(|p| m * p + translation)
                        .collect::<Vec<_>>();

                    let mut num_matches = 0;
                    for transformed_beacon in &transformed_report {
                        if beacons.contains(transformed_beacon) {
                            num_matches += 1;
                        }

                        if num_matches >= 12 {
                            return Some((translation, transformed_report));
                        }
                    }
                }
            }
        }
    }
    None
}
