use std::iter;

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

fn windows(w: usize, h: usize, grid: &Vec<bool>, edge: bool) -> impl Iterator<Item=[bool; 9]> + '_ {
    let g = move |x: usize, y: usize| { grid[x + w * y] };

    let first_row = iter::once([
        edge, edge, edge,
        edge, g(0, 0), g(1, 0),
        edge, g(0, 1), g(1, 1),
    ]).chain((1..(w - 1)).map(move |x| [
        edge, edge, edge,
        g(x - 1, 0), g(x, 0), g(x + 1, 0),
        g(x - 1, 1), g(x, 1), g(x + 1, 1),
    ])).chain(iter::once([
        edge, edge, edge,
        g(w - 2, 0), g(w - 1, 0), edge,
        g(w - 2, 1), g(w - 1, 1), edge,
    ]));

    let middle_rows = (1..(h - 1)).flat_map(move |y| {
        iter::once([
            edge, g(0, y - 1), g(1, y - 1),
            edge, g(0, y), g(1, y),
            edge, g(0, y + 1), g(1, y + 1),
        ]).chain((1..(w - 1)).map(move |x| [
            g(x - 1, y - 1), g(x, y - 1), g(x + 1, y - 1),
            g(x - 1, y), g(x, y), g(x + 1, y),
            g(x - 1, y + 1), g(x, y + 1), g(x + 1, y + 1),
        ])).chain(iter::once([
            g(w - 2, y - 1), g(w - 1, y - 1), edge,
            g(w - 2, y), g(w - 1, y), edge,
            g(w - 2, y + 1), g(w - 1, y + 1), edge,
        ]))
    });

    let last_row = iter::once([
        edge, g(0, h - 2), g(1, h - 2),
        edge, g(0, h - 1), g(1, h - 1),
        edge, edge, edge,
    ]).chain((1..(w - 1)).map(move |x| [
        g(x - 1, h - 2), g(x, h - 2), g(x + 1, h - 2),
        g(x - 1, h - 1), g(x, h - 1), g(x + 1, h - 1),
        edge, edge, edge,
    ])).chain(iter::once([
        g(w - 2, h - 2), g(w - 1, h - 2), edge,
        g(w - 2, h - 1), g(w - 1, h - 1), edge,
        edge, edge, edge,
    ]));

    first_row.chain(middle_rows).chain(last_row)
}

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize) -> usize {
    let (alg, grid) = input;

    let w = grid[0].len() + 2 * steps;
    let h = grid.len() + 2 * steps;
    let mut pixels = vec![false; w * h];
    for (y, row) in grid.iter().enumerate() {
        for (x, lit) in row.iter().enumerate() {
            pixels[(x + steps) + w * (y + steps)] = *lit;
        }
    }

    let mut fill_pixel = false;

    for _step in 0..steps {
        let mut next_pixels = Vec::with_capacity(pixels.len());

        for window in windows(w, h, &pixels, fill_pixel) {
            let mut idx = 0;
            for b in window {
                idx = idx << 1;
                idx = idx + b as usize;
            }
            next_pixels.push(alg[idx]);
        }

        fill_pixel = if fill_pixel {
            alg[511]
        } else {
            alg[0]
        };

        pixels = next_pixels;
    }

    pixels.iter().filter(|b| **b).count()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 50)
}
