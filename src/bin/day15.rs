use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

use nom::character::complete::{digit1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// Sort state in order to create a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut file = File::open("input/day15.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.as_str();

    let (_, grid) = parse_input(input).unwrap();
    let width = grid.len();
    let height = grid[0].len();

    println!("Part 1: {}", shortest_path_cost(&grid));

    let mut large_grid = vec![vec![0; height * 5]; width * 5];
    for px in 0..5 {
        for py in 0..5 {
            let dx = px * width;
            let dy = py * height;
            let dc = (px + py) as u32;
            for x in 0..width {
                for y in 0..height {
                    large_grid[x + dx][y + dy] = (grid[x][y] + dc - 1) % 9 + 1;
                }
            }
        }
    }

    println!("Part 2: {}", shortest_path_cost(&large_grid));
}

fn shortest_path_cost(grid: &Vec<Vec<u32>>) -> usize {
    let width = grid.len();
    let height = grid[0].len();

    let valid_neighbors = |x: usize, y: usize| -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x + 1 < width {
            neighbors.push((x + 1, y));
        }
        if y + 1 < height {
            neighbors.push((x, y + 1));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        neighbors
    };

    let mut dist = vec![vec![usize::MAX; height]; width];
    let mut heap = BinaryHeap::new();

    dist[0][0] = 0;
    heap.push(State { cost: 0, position: (0, 0) });

    while let Some(State { cost, position: (x, y) }) = heap.pop() {
        if x == width - 1 && y == height - 1 {
            return cost;
        }

        if cost > dist[x][y] {
            continue;
        }

        for (nx, ny) in valid_neighbors(x, y) {
            let next = State { cost: cost + grid[nx][ny] as usize, position: (nx, ny) };

            if next.cost < dist[nx][ny] {
                heap.push(next);
                dist[nx][ny] = next.cost;
            }
        }
    }

    panic!();
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
