use std::str::FromStr;

use hashbrown::{HashMap, HashSet};
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

fn extend_fingerprints_from_report(
    fingerprints: &mut HashMap<Fingerprint, Vec<[Vector3<i32>; 2]>>,
    report: &Vec<Vector3<i32>>,
) {
    for (p1, p2) in report.iter().tuple_combinations::<(_, _)>() {
        let f = fingerprint(p1, p2);
        fingerprints.entry(f).or_default().push([*p1, *p2]);
    }
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
    let (first_report, remaining_reports) = input.split_first().unwrap();

    let mut known_beacons = HashSet::<Vector3<i32>>::new();
    known_beacons.extend(first_report.iter());

    // NOTE: This does not contain fingerprints for all point-pairs (across report boundaries), only
    // for point-pairs within reports.
    let mut known_fingerprints = HashMap::new();
    extend_fingerprints_from_report(&mut known_fingerprints, first_report);

    // Pre-compute fingerprints for each remaining report
    let mut reports_with_fingerprints = remaining_reports.iter()
        .map(|report| {
            let fingerprints = report.iter()
                .tuple_combinations::<(_, _)>()
                .map(|(p1, p2)| {
                    (fingerprint(p1, p2), [p1, p2])
                })
                .collect::<Vec<_>>();
            (report, fingerprints)
        })
        .collect::<Vec<_>>();

    let mut scanners = vec![[0, 0, 0].into()];

    while !reports_with_fingerprints.is_empty() {
        for (index, (report, fingerprints)) in reports_with_fingerprints.iter().enumerate() {
            if let Some((scanner, transformed_report)) = {
                find_match(&known_beacons, &known_fingerprints, report, fingerprints)
            } {
                extend_fingerprints_from_report(&mut known_fingerprints, &transformed_report);
                scanners.push(scanner);
                known_beacons.extend(transformed_report.into_iter());
                reports_with_fingerprints.swap_remove(index);
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
    known_beacons: &HashSet<Vector3<i32>>,
    known_fingerprints: &HashMap<Fingerprint, Vec<[Vector3<i32>; 2]>>,
    report: &Vec<Vector3<i32>>,
    report_fingerprints: &Vec<(Fingerprint, [&Vector3<i32>; 2])>,
) -> Option<(Vector3<i32>, Vec<Vector3<i32>>)> {
    let matching_fingerprints = report_fingerprints
        .iter()
        .filter(|(f, _)| {
            known_fingerprints.contains_key(f)
        })
        .collect::<Vec<_>>();

    // At least (12 choose 2) fingerprints should match
    if matching_fingerprints.len() < 66 {
        return None;
    }

    for (f, report_pair) in matching_fingerprints {
        for known_pair in known_fingerprints.get(&f).unwrap() {
            // ASSUMPTION: Order of beacons in input is invariant between reports. So if beacon A
            // appears before beacon B in report 1, so should it in other reports. Without this
            // assumption we'd need to iterator through the cartesian product of report_pair with
            // known_pair for our pinned points.
            let [kp1, kp2] = known_pair;
            let [rp1, rp2] = *report_pair;

            let supported_rotations = ROTATION_MATRICES.iter()
                // This always returns 0 or 1 items, so it could be replaced with a find().
                // But it's not really performance critical, so I'll leave it as is.
                .filter(|&m| {
                    kp1 - m * rp1 == kp2 - m * rp2
                });

            for m in supported_rotations {
                let translation = kp1 - m * rp1;

                let transformed_report = report
                    .iter()
                    .map(|p| m * p + translation)
                    .collect::<Vec<_>>();

                let mut num_matches = 0;
                for transformed_beacon in &transformed_report {
                    if known_beacons.contains(transformed_beacon) {
                        num_matches += 1;
                    }

                    // ASSUMPTION: Only 3 matches are needed to determine overlap.
                    if num_matches >= 3 {
                        return Some((translation, transformed_report));
                    }
                }
            }
        }
    }
    None
}
