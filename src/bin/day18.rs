use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};

fn main() {
    let mut file = File::open("input/day18.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, numbers) = parse_input(input).unwrap();

    let result_part1 = numbers.iter()
        .cloned()
        .reduce(|n1, n2| reduce(add(n1, n2)))
        .unwrap();

    println!("Part 1: {}", magnitude(result_part1));

    let result_part2 = numbers.iter()
        .cloned()
        .tuple_combinations::<(_, _)>()
        .map(|(n1, n2)| {
            magnitude(reduce(add(n1, n2)))
        })
        .max()
        .unwrap();

    println!("Part 2: {}", result_part2);
}

fn magnitude(n: Number) -> usize {
    match n {
        Number::Regular(n) => n,
        Number::Pair(n1, n2) => 3 * magnitude(*n1) + 2 * magnitude(*n2),
    }
}

fn add(n1: Number, n2: Number) -> Number {
    Number::Pair(Box::new(n1), Box::new(n2))
}

fn reduce(mut n: Number) -> Number {
    loop {
        if let Some(_n) = reduce_explode(&n) {
            n = _n;
            continue;
        }
        if let Some(_n) = reduce_split(&n) {
            n = _n;
            continue;
        }

        break n;
    }
}

fn reduce_explode(n: &Number) -> Option<Number> {
    fn add_left(n: Number, nl: usize) -> Number {
        match n {
            Number::Regular(n) => Number::Regular(n + nl),
            Number::Pair(n1, n2) => {
                return add(add_left(*n1.clone(), nl), *n2.clone());
            }
        }
    }
    fn add_right(n: Number, nr: usize) -> Number {
        match n {
            Number::Regular(n) => Number::Regular(n + nr),
            Number::Pair(n1, n2) => {
                return add(*n1.clone(), add_right(*n2.clone(), nr));
            }
        }
    }

    fn explode(n: &Number, depth: usize) -> Option<(usize, Number, usize)> {
        match n {
            Number::Pair(n1, n2) if depth == 4 => {
                match (n1.as_ref(), n2.as_ref()) {
                    (Number::Regular(nl), Number::Regular(nr)) => Some((*nl, Number::Regular(0), *nr)),
                    // At depth 4, only regular numbers should remain
                    _ => unreachable!(),
                }
            }
            Number::Pair(n1, n2) => {
                // Explode left pair
                if let Some((nl, n1, nr)) = explode(n1.as_ref(), depth + 1) {
                    return Some((nl, add(n1, add_left(*n2.clone(), nr)), 0));
                }
                // Explode right pair
                if let Some((nl, n2, nr)) = explode(n2.as_ref(), depth + 1) {
                    return Some((0, add(add_right(*n1.clone(), nl), n2), nr));
                }
                None
            }
            _ => None,
        }
    }

    explode(n, 0).map(|(_, n, _)| n)
}

fn reduce_split(n: &Number) -> Option<Number> {
    match n {
        Number::Regular(n) => {
            if *n >= 10 {
                let n1 = *n / 2;
                let n2 = *n - (*n / 2);
                return Some(add(Number::Regular(n1), Number::Regular(n2)));
            }

            None
        }
        Number::Pair(n1, n2) => {
            if let Some(n1) = reduce_split(n1.as_ref()) {
                return Some(Number::Pair(Box::new(n1), n2.clone()));
            }
            if let Some(n2) = reduce_split(n2.as_ref()) {
                return Some(Number::Pair(n1.clone(), Box::new(n2)));
            }
            None
        }
    }
}

#[derive(Debug, Clone)]
enum Number {
    Regular(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Regular(n) => write!(f, "{}", n),
            Number::Pair(n1, n2) => write!(f, "[{},{}]", n1, n2)
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Number>> {
    separated_list1(
        newline,
        parse_snailfish_number,
    )(input)
}

fn parse_snailfish_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(
            delimited(
                tag("["),
                separated_pair(
                    parse_snailfish_number,
                    tag(","),
                    parse_snailfish_number,
                ),
                tag("]"),
            ),
            |(n1, n2)| add(n1, n2),
        ),
        map(parse_number, |n| Number::Regular(n))
    ))(input)
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

