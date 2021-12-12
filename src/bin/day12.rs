use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    let mut file = File::open("input/day12.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let mut adjacency_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (c1, c2) in parse_input(input).unwrap().1 {
        adjacency_map.entry(c1).or_insert(HashSet::new()).insert(c2);
        adjacency_map.entry(c2).or_insert(HashSet::new()).insert(c1);
    }

    println!("Part 1: {}", count_paths(&adjacency_map, "start", HashSet::new(), true));
    println!("Part 2: {}", count_paths(&adjacency_map, "start", HashSet::new(), false));
}

fn count_paths<'a>(
    adjacency_map: &HashMap<&'a str, HashSet<&'a str>>,
    cave: &'a str,
    mut closed: HashSet<&'a str>,
    is_joker_used: bool,
) -> usize {
    if cave == "end" {
        return 1;
    }

    if !is_big(cave) {
        closed.insert(cave);
    }

    let mut sum = 0;
    for &adj_cave in &adjacency_map[cave] {
        if !closed.contains(adj_cave) {
            sum += count_paths(&adjacency_map, adj_cave, closed.clone(), is_joker_used);
        } else if !is_joker_used && adj_cave != "start" && adj_cave != "end" {
            sum += count_paths(&adjacency_map, adj_cave, closed.clone(), true);
        }
    }
    sum
}

fn is_big(cave: &str) -> bool {
    cave.chars().next().unwrap().is_ascii_uppercase()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        newline,
        separated_pair(parse_cave, tag("-"), parse_cave),
    )(input)
}

fn parse_cave(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
