use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::{preceded, separated_pair, tuple};

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (usize, usize) {
    fn parse_player(input: &str) -> IResult<&str, usize> {
        preceded(
            tuple((tag("Player "), digit1, tag(" starting position: "))),
            map_res(digit1, FromStr::from_str),
        )(input)
    }

    let (_, result) = separated_pair(
        parse_player,
        newline,
        parse_player,
    )(input).unwrap();

    result
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &(usize, usize)) -> usize {
    let mut positions = [input.0 - 1, input.1 - 1];
    let mut scores = [0, 0];

    let mut die_rolls = 0;

    let mut current_player = 0;
    let loser_score = loop {
        let current_pos = &mut positions[current_player];
        let current_score = &mut scores[current_player];

        let mut die_total = 0;
        for _ in 0..3 {
            die_rolls += 1;
            die_total += die_rolls % 100;
        }

        *current_pos = (*current_pos + die_total) % 10;
        *current_score += *current_pos + 1;

        if *current_score >= 1000 {
            let other_player = (current_player + 1) % 2;
            break scores[other_player];
        }

        current_player = (current_player + 1) % 2;
    };

    die_rolls * loser_score
}
