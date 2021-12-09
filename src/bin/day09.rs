use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

fn main() {
    let mut file = File::open("input/day09.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, grid) = parse_input(input).unwrap();

    let width = grid.len();
    let height = grid[0].len();

    let valid_neighbors = |x: usize, y: usize| -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < width {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < height {
            neighbors.push((x, y + 1));
        }
        neighbors
    };

    let mut sum_risk_level = 0;
    let mut basin_sizes: Vec<usize> = vec![];

    for x in 0..width {
        for y in 0..height {
            let mut is_low_point = true;

            for (nx, ny) in valid_neighbors(x, y) {
                if grid[x][y] >= grid[nx][ny] {
                    is_low_point = false;
                    break;
                }
            }

            if is_low_point {
                sum_risk_level += grid[x][y] + 1;

                // Find basin size
                let mut basin_size = 0;
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                let mut closed_set: HashSet<(usize, usize)> = HashSet::new();
                q.push_back((x, y));

                while let Some((x, y)) = q.pop_front() {
                    if grid[x][y] == 9 {
                        continue;
                    }
                    if closed_set.contains(&(x, y)) {
                        continue;
                    }

                    for (nx, ny) in valid_neighbors(x, y) {
                        q.push_back((nx, ny));
                    }

                    basin_size += 1;
                    closed_set.insert((x, y));
                }

                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();
    let part2 = basin_sizes[basin_sizes.len() - 1] * basin_sizes[basin_sizes.len() - 2] * basin_sizes[basin_sizes.len() - 3];

    println!("Part 1: {}", sum_risk_level);
    println!("Part 2: {}", part2);
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
