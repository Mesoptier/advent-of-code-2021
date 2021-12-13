use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, separated_pair};

fn main() {
    let mut file = File::open("input/day13.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, (points, folds)) = parse_input(input).unwrap();

    let mut grid = HashSet::new();
    for (x, y) in points {
        grid.insert((x, y));
    }

    let mut is_first = true;

    for fold in folds {
        let mut next_grid = HashSet::new();
        for (x, y) in grid {
            let (nx, ny) = match fold {
                FoldInstruction::Up(fy) if y > fy => (x, 2 * fy - y),
                FoldInstruction::Left(fx) if x > fx => (2 * fx - x, y),
                _ => (x, y),
            };
            next_grid.insert((nx, ny));
        }

        if is_first {
            is_first = false;
            println!("Part 1: {}", next_grid.len());
        }

        grid = next_grid;
    }

    println!("Part 2:");
    print_grid(&grid);
}

fn print_grid(grid: &HashSet<(usize, usize)>) {
    let mut x_max = 0;
    let mut y_max = 0;

    for &(x, y) in grid {
        x_max = usize::max(x, x_max);
        y_max = usize::max(y, y_max);
    }

    for y in 0..=y_max {
        for x in 0..=x_max {
            if grid.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[derive(Debug)]
enum FoldInstruction {
    Up(usize),
    Left(usize),
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<FoldInstruction>)> {
    separated_pair(
        parse_points,
        count(newline, 2),
        parse_folds,
    )(input)
}

fn parse_points(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(
        newline,
        separated_pair(parse_number, tag(","), parse_number)
    )(input)
}

fn parse_folds(input: &str) -> IResult<&str, Vec<FoldInstruction>> {
    separated_list1(
        newline,
        preceded(tag("fold along "), alt((
            map(preceded(tag("x="), parse_number), |x| FoldInstruction::Left(x)),
            map(preceded(tag("y="), parse_number), |y| FoldInstruction::Up(y)),
        )))
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}
