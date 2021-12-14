use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use itertools::{Itertools, MinMaxResult};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, newline};
use nom::IResult;
use nom::multi::{count, separated_list1};
use nom::sequence::{pair, separated_pair};

type Rules = HashMap<(char, char), char>;
type Counts = HashMap<char, usize>;
type CountsCache = HashMap<(char, char, usize), Counts>;

fn main() {
    let mut file = File::open("input/day14.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, (template, rules_vec)) = parse_input(input).unwrap();
    let rules: Rules = Rules::from_iter(rules_vec.into_iter());

    let mut cache: CountsCache = CountsCache::new();

    println!("Part 1: {}", solve(&rules, &mut cache, template, 10));
    println!("Part 2: {}", solve(&rules, &mut cache, template, 40));
}

fn merge_counts(counts: &mut Counts, other: Counts) {
    for (c, n) in other {
        *counts.entry(c).or_default() += n;
    }
}

fn process_pair(
    rules: &Rules,
    cache: &mut CountsCache,
    c1: char,
    c2: char,
    steps: usize
) -> Counts {
    if let Some(cached_result) = cache.get(&(c1, c2, steps)) {
        return cached_result.clone();
    }

    let mut counts: Counts = Counts::new();

    if steps == 0 {
        *counts.entry(c1).or_default() += 1;
    } else if let Some(&c3) = rules.get(&(c1, c2)) {
        merge_counts(&mut counts, process_pair(rules, cache, c1, c3, steps - 1));
        merge_counts(&mut counts, process_pair(rules, cache, c3, c2, steps - 1));
    }

    cache.insert((c1, c2, steps), counts.clone());

    counts
}

fn solve(
    rules: &Rules,
    cache: &mut CountsCache,
    template: &str,
    steps: usize
) -> usize {
    let mut counts: Counts = Counts::new();
    for (c1, c2) in template.chars().tuple_windows() {
        merge_counts(&mut counts, process_pair(rules, cache, c1, c2, steps));
    }
    let last_char = template.chars().last().unwrap();
    *counts.entry(last_char).or_default() += 1;

    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => {
            return max - min;
        }
        _ => panic!()
    }
}

fn parse_input(input: &str) -> IResult<&str, (&str, Vec<((char, char), char)>)> {
    separated_pair(
        alpha1,
        count(newline, 2),
        parse_rules,
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<((char, char), char)>> {
    separated_list1(
        newline,
        separated_pair(
            pair(anychar, anychar),
            tag(" -> "),
            anychar,
        ),
    )(input)
}

