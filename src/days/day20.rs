use std::sync::{Arc, mpsc};

use itertools::{Itertools, zip};

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

/// Get grid index
macro_rules! g {
    ( $w:expr, $x:expr, $y:expr ) => { $x + $w * $y };
}

fn process(
    ys: &Vec<usize>,
    w: usize,
    h: usize,
    grid: &Vec<bool>,
    edge: bool,
    alg: &Vec<bool>,
) -> Vec<bool> {
    let mut result = Vec::with_capacity(w * ys.len());

    let mut ys_range = 0..ys.len();

    /// Get the value "returned" by the "algorithm" for the given index
    macro_rules! alg_value {
        ( $x:expr ) => { alg[$x] };
    }

    // Special case for the first row of the grid
    if *ys.first().unwrap() == 0 {
        ys_range.start += 1;

        // First row
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            edge, edge, grid[g!(w, 0, 0)]
        ]));
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            edge, grid[g!(w, 0, 0)], grid[g!(w, 1, 0)]
        ]));
        for x in 1..(w - 1) {
            result.push(alg_value!(u_bits![
                edge, edge, edge,
                edge, edge, edge,
                grid[g!(w, x - 1, 0)], grid[g!(w, x, 0)], grid[g!(w, x + 1, 0)]
            ]));
        }
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            grid[g!(w, w - 2, 0)], grid[g!(w, w - 1, 0)], edge
        ]));
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            grid[g!(w, w - 1, 0)], edge, edge
        ]));

        // Second row
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, edge, grid[g!(w, 0, 0)],
            edge, edge, grid[g!(w, 0, 1)]
        ]));
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            edge, grid[g!(w, 0, 0)], grid[g!(w, 1, 0)],
            edge, grid[g!(w, 0, 1)], grid[g!(w, 1, 1)]
        ]));
        for x in 1..(w - 1) {
            result.push(alg_value!(u_bits![
                edge, edge, edge,
                grid[g!(w, x - 1, 0)], grid[g!(w, x, 0)], grid[g!(w, x + 1, 0)],
                grid[g!(w, x - 1, 1)], grid[g!(w, x, 1)], grid[g!(w, x + 1, 1)]
            ]));
        }
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            grid[g!(w, w - 2, 0)], grid[g!(w, w - 1, 0)], edge,
            grid[g!(w, w - 2, 1)], grid[g!(w, w - 1, 1)], edge
        ]));
        result.push(alg_value!(u_bits![
            edge, edge, edge,
            grid[g!(w, w - 1, 0)], edge, edge,
            grid[g!(w, w - 1, 1)], edge, edge
        ]));
    }

    // Skip last row of the grid, so we can handle special case after
    if *ys.last().unwrap() == h - 1 {
        ys_range.end -= 1;
    }

    // Middle rows (hot path)
    for y in ys_range.map(|i| ys[i]) {
        result.push(alg_value!(u_bits![
            edge, edge, grid[g!(w, 0, y - 1)],
            edge, edge, grid[g!(w, 0, y)],
            edge, edge, grid[g!(w, 0, y + 1)]
        ]));
        result.push(alg_value!(u_bits![
            edge, grid[g!(w, 0, y - 1)], grid[g!(w, 1, y - 1)],
            edge, grid[g!(w, 0, y)], grid[g!(w, 1, y)],
            edge, grid[g!(w, 0, y + 1)], grid[g!(w, 1, y + 1)]
        ]));
        for x in 1..(w - 1) {
            result.push(alg_value!(u_bits![
                grid[g!(w, x - 1, y - 1)], grid[g!(w, x, y - 1)], grid[g!(w, x + 1, y - 1)],
                grid[g!(w, x - 1, y)], grid[g!(w, x, y)], grid[g!(w, x + 1, y)],
                grid[g!(w, x - 1, y + 1)], grid[g!(w, x, y + 1)], grid[g!(w, x + 1, y + 1)]
            ]));
        }
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 2, y - 1)], grid[g!(w, w - 1, y - 1)], edge,
            grid[g!(w, w - 2, y)], grid[g!(w, w - 1, y)], edge,
            grid[g!(w, w - 2, y + 1)], grid[g!(w, w - 1, y + 1)], edge
        ]));
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 1, y - 1)], edge, edge,
            grid[g!(w, w - 1, y)], edge, edge,
            grid[g!(w, w - 1, y + 1)], edge, edge
        ]));
    }

    // Special case for the last row of the grid
    if *ys.last().unwrap() == h - 1 {
        // Second-to-last row
        result.push(alg_value!(u_bits![
            edge, edge, grid[g!(w, 0, h - 2)],
            edge, edge, grid[g!(w, 0, h - 1)],
            edge, edge, edge
        ]));
        result.push(alg_value!(u_bits![
            edge, grid[g!(w, 0, h - 2)], grid[g!(w, 1, h - 2)],
            edge, grid[g!(w, 0, h - 1)], grid[g!(w, 1, h - 1)],
            edge, edge, edge
        ]));
        for x in 1..(w - 1) {
            result.push(alg_value!(u_bits![
                grid[g!(w, x - 1, h - 2)], grid[g!(w, x, h - 2)], grid[g!(w, x + 1, h - 2)],
                grid[g!(w, x - 1, h - 1)], grid[g!(w, x, h - 1)], grid[g!(w, x + 1, h - 1)],
                edge, edge, edge
            ]));
        }
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 2, h - 2)], grid[g!(w, w - 1, h - 2)], edge,
            grid[g!(w, w - 2, h - 1)], grid[g!(w, w - 1, h - 1)], edge,
            edge, edge, edge
        ]));
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 1, h - 2)], edge, edge,
            grid[g!(w, w - 1, h - 1)], edge, edge,
            edge, edge, edge
        ]));

        // Last row
        result.push(alg_value!(u_bits![
            edge, edge, grid[g!(w, 0, h - 1)],
            edge, edge, edge,
            edge, edge, edge
        ]));
        result.push(alg_value!(u_bits![
            edge, grid[g!(w, 0, h - 1)], grid[g!(w, 1, h - 1)],
            edge, edge, edge,
            edge, edge, edge
        ]));
        for x in 1..(w - 1) {
            result.push(alg_value!(u_bits![
                grid[g!(w, x - 1, h - 1)], grid[g!(w, x, h - 1)], grid[g!(w, x + 1, h - 1)],
                edge, edge, edge,
                edge, edge, edge
            ]));
        }
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 2, h - 1)], grid[g!(w, w - 1, h - 1)], edge,
            edge, edge, edge,
            edge, edge, edge
        ]));
        result.push(alg_value!(u_bits![
            grid[g!(w, w - 1, h - 1)], edge, edge,
            edge, edge, edge,
            edge, edge, edge
        ]));
    }

    result
}

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize, num_threads: usize) -> usize {
    let (alg, grid) = input;

    let mut w = grid[0].len();
    let mut h = grid.len();
    let h_max = grid.len() + steps * 2; // eventual height

    // Construct the grid
    let mut grid = Arc::new({
        Vec::from_iter(
            grid.iter()
                .enumerate()
                .flat_map(|(_y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(_x, lit)| *lit)
                })
        )
    });
    let alg = Arc::new(alg.clone());

    // The state of all cells in the infinite grid outside our grid
    let mut edge = false;

    // Spawn threads, each thread processes a chunk of the rows in the grid
    let mut thread_pool = vec![];
    for _ in 0..num_threads {
        let alg = alg.clone();

        type ProcessData = (Vec<usize>, Arc<Vec<bool>>, usize, usize, bool);

        let (tx1, rx1) = mpsc::channel::<ProcessData>();
        let (tx2, rx2) = mpsc::channel::<Vec<bool>>();

        std::thread::spawn(move || {
            for (y_range, grid, w, h, edge) in rx1 {
                tx2.send(process(&y_range, w, h, &grid, edge, &alg)).unwrap();
            }
        });

        thread_pool.push((tx1, rx2));
    }

    let chunk_size = (h_max / num_threads).max(1);

    for _step in 0..steps {
        // Send out chunks to be processed
        let mut rxs = vec![];
        for (ys_chunk, (tx, rx)) in zip(&(0..h).chunks(chunk_size), &thread_pool) {
            let ys_chunk = ys_chunk.collect::<Vec<_>>();
            tx.send((ys_chunk, grid.clone(), w, h, edge)).unwrap();
            rxs.push(rx);
        }

        // Receive and collect the processed chunks
        let mut next_grid = Vec::with_capacity(grid.len());
        for rx in rxs {
            next_grid.extend_from_slice(rx.recv().unwrap().as_slice());
        }

        // Update width and height to account for the new cells added around the grid
        w += 2;
        h += 2;

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
    solve(input, 50, 8)
}
