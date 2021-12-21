use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

use hashbrown::HashMap;
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    current_player: Player,
    positions: (usize, usize),
    scores: (usize, usize),
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_score().cmp(&other.min_score()).reverse()
            .then(self.max_score().cmp(&other.max_score()).reverse())
    }
}

impl State {
    fn other_player(&self) -> Player {
        match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    fn is_game_over(&self) -> bool {
        let max_score = 21;
        self.scores.0 == max_score || self.scores.1 == max_score
    }

    fn min_score(&self) -> usize {
        self.scores.0.min(self.scores.1)
    }

    fn max_score(&self) -> usize {
        self.scores.0.max(self.scores.1)
    }

    fn take_turn(&self, die_total: usize) -> State {
        let num_spaces = 10;
        let max_score = 21;

        let new_positions = match self.current_player {
            Player::Player1 => ((self.positions.0 + die_total) % num_spaces, self.positions.1),
            Player::Player2 => (self.positions.0, (self.positions.1 + die_total) % num_spaces),
        };
        let new_scores = match self.current_player {
            Player::Player1 => ((self.scores.0 + new_positions.0 + 1).min(max_score), self.scores.1),
            Player::Player2 => (self.scores.0, (self.scores.1 + new_positions.1 + 1).min(max_score)),
        };

        State {
            current_player: self.other_player(),
            positions: new_positions,
            scores: new_scores,
        }
    }
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

    // Maps state to the number of states it can be reached from
    let mut states_count: HashMap<State, usize> = HashMap::new();

    let mut q = BinaryHeap::new();

    let initial_state = State {
        current_player: Player::Player1,
        positions: (input.0 - 1, input.1 - 1),
        scores: (0, 0),
    };
    q.push(initial_state);
    states_count.insert(initial_state, 1);

    while let Some(state) = q.pop() {
        let parent_count = states_count[&state];

        for (die_total, universe_count) in dice_rolls {
            let new_state = state.take_turn(die_total);

            let foo = states_count.contains_key(&new_state);

            *states_count.entry(new_state).or_insert(0) += universe_count * parent_count;

            if new_state.is_game_over() {
                continue;
            }

            if !foo {
                q.push(new_state);
            }
        }
    }

    println!("{}", states_count.len());

    let mut p1_count = 0;
    let mut p2_count = 0;

    for (state, count) in states_count {
        if state.is_game_over() {
            match state.other_player() {
                Player::Player1 => p1_count += count,
                Player::Player2 => p2_count += count,
            }
        }
    }

    p1_count.max(p2_count)
}
