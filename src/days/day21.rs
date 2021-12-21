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
            die_total += die_rolls;
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

#[aoc(day21, part2)]
pub fn solve_part2(input: &(usize, usize)) -> usize {
    let dice_rolls = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    let mut dp = [[[[[0; 2]; 10]; 10]; 21]; 21];

    dp[0][0][input.0 - 1][input.1 - 1][0] = 1;

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for p1_score in 0..21 {
        for p2_score in 0..21 {
            for p1_pos in 0..10 {
                for p2_pos in 0..10 {
                    for current_player in 0..2 {
                        let parent_count = dp[p1_score][p2_score][p1_pos][p2_pos][current_player];
                        if parent_count == 0 {
                            continue;
                        }

                        for (die_total, roll_count) in dice_rolls {
                            let (p1_pos, p2_pos) = match current_player {
                                0 => ((p1_pos + die_total) % 10, p2_pos),
                                1 => (p1_pos, (p2_pos + die_total) % 10),
                                _ => unreachable!(),
                            };
                            let (p1_score, p2_score) = match current_player {
                                0 => (p1_score + p1_pos + 1, p2_score),
                                1 => (p1_score, p2_score + p2_pos + 1),
                                _ => unreachable!(),
                            };
                            let current_player = (current_player + 1) % 2;

                            if p1_score >= 21 {
                                p1_wins += parent_count * roll_count;
                            } else if p2_score >= 21 {
                                p2_wins += parent_count * roll_count;
                            } else {
                                dp[p1_score][p2_score][p1_pos][p2_pos][current_player] += parent_count * roll_count;
                            }
                        }
                    }
                }
            }
        }
    }

    p1_wins.max(p2_wins)
}
