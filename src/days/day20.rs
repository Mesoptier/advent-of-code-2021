use std::iter;
use std::sync::{Arc, mpsc};

use itertools::Itertools;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    fn parse_line(line: &str) -> Vec<bool> {
        line.chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
    }

    let mut lines = input.lines();
    let alg = parse_line(lines.next().unwrap());
    lines.next();
    let grid = lines.map(parse_line).collect::<Vec<_>>();
    (alg, grid)
}

/// Create usize from bits
macro_rules! u_bits {
    ( $( $x:expr ),* ) => {
        {
            let mut n = 0usize;
            $(
                n = (n << 1) + ($x as usize);
            )*
            n
        }
    };
}

macro_rules! g {
    ( $w:expr, $x:expr, $y:expr ) => { $x + $w * $y };
}

fn process(
    y_range: &Vec<usize>,
    w: usize,
    h: usize,
    grid: Arc<Vec<bool>>,
    edge: bool,
    alg: &Arc<Vec<bool>>,
) -> Vec<bool> {
    let mut result = Vec::with_capacity(w * y_range.len());

    let mut i_min = 0;
    let mut i_max = y_range.len() - 1;

    // Special case for the first row of the grid
    if *y_range.first().unwrap() == 0 {
        i_min += 1;
        let it = iter::empty()
            .chain(iter::once(u_bits![
                edge, edge, edge,
                edge, grid[g!(w, 0, 0)], grid[g!(w, 1, 0)],
                edge, grid[g!(w, 0, 1)], grid[g!(w, 1, 1)]
            ]))
            .chain((1..(w - 1)).map({
                |x| u_bits![
                    edge, edge, edge,
                    grid[g!(w, x - 1, 0)], grid[g!(w, x, 0)], grid[g!(w, x + 1, 0)],
                    grid[g!(w, x - 1, 1)], grid[g!(w, x, 1)], grid[g!(w, x + 1, 1)]
                ]
            }))
            .chain(iter::once(u_bits![
                edge, edge, edge,
                grid[g!(w, w - 2, 0)], grid[g!(w, w - 1, 0)], edge,
                grid[g!(w, w - 2, 1)], grid[g!(w, w - 1, 1)], edge
            ]))
            .map(|alg_idx| alg[alg_idx]);
        result.extend(it);
    }

    // Skip last row of the grid, so we can handle special case after
    if *y_range.last().unwrap() == h - 1 {
        i_max -= 1;
    }

    // Middle rows (hot path)
    for y in y_range[i_min]..=y_range[i_max] {
        let it = iter::empty()
            .chain(iter::once(u_bits![
                edge, grid[g!(w, 0, y - 1)], grid[g!(w, 1, y - 1)],
                edge, grid[g!(w, 0, y)], grid[g!(w, 1, y)],
                edge, grid[g!(w, 0, y + 1)], grid[g!(w, 1, y + 1)]
            ]))
            .chain((1..(w - 1)).map({
                |x| u_bits![
                    grid[g!(w, x - 1, y - 1)], grid[g!(w, x, y - 1)], grid[g!(w, x + 1, y - 1)],
                    grid[g!(w, x - 1, y)], grid[g!(w, x, y)], grid[g!(w, x + 1, y)],
                    grid[g!(w, x - 1, y + 1)], grid[g!(w, x, y + 1)], grid[g!(w, x + 1, y + 1)]
                ]
            }))
            .chain(iter::once(u_bits![
                grid[g!(w, w - 2, y - 1)], grid[g!(w, w - 1, y - 1)], edge,
                grid[g!(w, w - 2, y)], grid[g!(w, w - 1, y)], edge,
                grid[g!(w, w - 2, y + 1)], grid[g!(w, w - 1, y + 1)], edge
            ]))
            .map(|alg_idx| alg[alg_idx]);
        result.extend(it);
    }

    // Special case for the last row of the grid
    if *y_range.last().unwrap() == h - 1 {
        let it = iter::empty()
            .chain(iter::once(u_bits![
                edge, grid[g!(w, 0, h - 2)], grid[g!(w, 1, h - 2)],
                edge, grid[g!(w, 0, h - 1)], grid[g!(w, 1, h - 1)],
                edge, edge, edge
            ]))
            .chain((1..(w - 1)).map({
                |x| u_bits![
                    grid[g!(w, x - 1, h - 2)], grid[g!(w, x, h - 2)], grid[g!(w, x + 1, h - 2)],
                    grid[g!(w, x - 1, h - 1)], grid[g!(w, x, h - 1)], grid[g!(w, x + 1, h - 1)],
                    edge, edge, edge
                ]
            }))
            .chain(iter::once(u_bits![
                grid[g!(w, w - 2, h - 2)], grid[g!(w, w - 1, h - 2)], edge,
                grid[g!(w, w - 2, h - 1)], grid[g!(w, w - 1, h - 1)], edge,
                edge, edge, edge
            ]))
            .map(|alg_idx| alg[alg_idx]);
        result.extend(it);
    }

    result
}

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize, num_threads: usize) -> usize {
    let (alg, grid) = input;

    // Construct the grid, with padding
    let w = grid[0].len() + steps * 2;
    let h = grid.len() + steps * 2;
    let mut grid = Arc::new({
        let mut padded_grid = vec![false; w * h];
        for (y, row) in grid.iter().enumerate() {
            for (x, lit) in row.iter().enumerate() {
                padded_grid[(x + steps) + w * (y + steps)] = *lit;
            }
        }
        padded_grid
    });
    let alg = Arc::new(alg.clone());
    let mut edge = false;

    let mut channels = vec![];

    for y_range in &(0..h).chunks((h / num_threads).max(1)) {
        let alg = alg.clone();

        let (tx1, rx1) = mpsc::channel::<(Arc<Vec<bool>>, bool)>();
        let (tx2, rx2) = mpsc::channel::<Vec<bool>>();

        let y_range = y_range.collect::<Vec<_>>();
        std::thread::spawn(move || {
            for (grid, edge) in rx1 {
                tx2.send(process(&y_range, w, h, grid, edge, &alg)).unwrap();
            }
        });

        channels.push((tx1, rx2));
    }

    for _step in 0..steps {
        for (tx, _) in &channels {
            tx.send((grid.clone(), edge)).unwrap();
        }

        let mut next_grid = Vec::with_capacity(grid.len());
        for (_, rx) in &channels {
            next_grid.extend_from_slice(rx.recv().unwrap().as_slice());
        }

        grid = Arc::from(next_grid);
        edge = if edge {
            alg[511]
        } else {
            alg[0]
        };
    }

    grid.iter().filter(|b| **b).count()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 2, 1)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 50, 12)
}
