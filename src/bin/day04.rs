use std::fs::File;
use std::io::Read;

use nom::bytes::complete::{tag};
use nom::character::complete::{digit1, multispace0, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, separated_pair};

fn main() {
    let mut file = File::open("input/day04.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let (_, (draw_order, boards)) = parse_input(input.as_str()).unwrap();
    let mut boards_marked = vec![vec![false; 25]; boards.len()];

    let mut win_count = 0;
    let mut completed_boards = vec![false; boards.len()];

    for number in draw_order {
        for (board_index, board) in boards.iter().enumerate() {
            if completed_boards[board_index] {
                continue;
            }

            for (i, &n) in board.iter().enumerate() {
                if number == n {
                    boards_marked[board_index][i] = true;

                    if is_board_complete(&boards_marked[board_index]) {
                        win_count += 1;
                        completed_boards[board_index] = true;

                        // First board to win
                        if win_count == 1 {
                            println!("Part 1: {}", number * count_unmarked(board, &boards_marked[board_index]));
                        }

                        // Last board to win
                        if win_count == boards.len() {
                            println!("Part 2: {}", number * count_unmarked(board, &boards_marked[board_index]));
                        }
                    }
                }
            }
        }
    }
}

fn count_unmarked(board: &Vec<usize>, marked: &Vec<bool>) -> usize {
    let mut sum_unmarked = 0;
    for i in 0..25 {
        if !marked[i] {
            sum_unmarked += board[i];
        }
    }
    sum_unmarked
}

fn is_board_complete(marked: &Vec<bool>) -> bool {
    let mut col_count = [0; 5];
    let mut row_count = [0; 5];

    for row in 0..5 {
        for col in 0..5 {
            if marked[row * 5 + col] {
                col_count[col] += 1;
                row_count[row] += 1;

                if col_count[col] == 5 || row_count[row] == 5 {
                    return true;
                }
            }
        }
    }

    false
}

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<Vec<usize>>)> {
    separated_pair(
        parse_draw_order,
        count(newline, 2),
        parse_boards,
    )(input)
}

fn parse_draw_order(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(
        tag(","),
        map(
            digit1,
            |s: &str| s.parse::<usize>().unwrap(),
        ),
    )(input)
}

fn parse_boards(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        count(newline, 2),
        parse_board,
    )(input)
}

fn parse_board(input: &str) -> IResult<&str, Vec<usize>> {
    count(
        preceded(
            multispace0,
            map(
                digit1,
                |s: &str| s.parse::<usize>().unwrap(),
            ),
        ),
        25,
    )(input)
}
