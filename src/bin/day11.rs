use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

fn main() {
    let mut file = File::open("input/day11.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, mut grid) = parse_input(input).unwrap();
    let width = grid.len();
    let height = grid[0].len();

    let valid_neighbors = |x: usize, y: usize| {
        let mut neighbors = vec![];
        for ix in 0..3 {
            if (ix == 0 && x == 0) || (ix == 2 && x == width - 1) {
                continue;
            }
            for iy in 0..3 {
                if (iy == 0 && y == 0) || (iy == 2 && y == height - 1) || (iy == 1 && ix == 1) {
                    continue;
                }
                neighbors.push((x + ix - 1, y + iy - 1));
            }
        }
        neighbors
    };

    let mut step = 0;
    let mut flash_count = 0;

    loop {
        let mut next_grid = grid.clone();
        let mut flashed = HashSet::new();

        for x in 0..width {
            for y in 0..height {
                let mut q = VecDeque::new();
                q.push_back((x, y));

                while let Some((x, y)) = q.pop_front() {
                    next_grid[x][y] += 1;

                    if next_grid[x][y] > 9 && !flashed.contains(&(x, y)) {
                        flashed.insert((x, y));

                        for (nx, ny) in valid_neighbors(x, y) {
                            q.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        for &(x, y) in &flashed {
            next_grid[x][y] = 0;
            flash_count += 1;
        }

        grid = next_grid;
        step += 1;

        if step == 100 {
            println!("Part 1: {}", flash_count);
        }

        if flashed.len() == 100 {
            println!("Part 2: {}", step);
            break;
        }
    }

}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        newline,
        map(
            digit1,
            |s: &str| {
                s.chars().map(|c: char| {
                    c.to_digit(10).unwrap()
                }).collect()
            },
        ),
    )(input)
}
