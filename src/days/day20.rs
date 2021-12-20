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

fn windows(w: usize, h: usize, grid: &Vec<bool>, edge: bool) -> impl Iterator<Item=usize> + '_ {
    let g = move |x: usize, y: usize| { grid[x + w * y] };

    let first_extend_row = iter::empty()
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            edge, edge, g(0, 0)
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            edge, g(0, 0), g(1, 0)
        ]))
        .chain((1..(w - 1)).map(move |x| u_bits![
            edge, edge, edge,
            edge, edge, edge,
            g(x - 1, 0), g(x, 0), g(x + 1, 0)
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            g(w - 2, 0), g(w - 1, 0), edge
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, edge, edge,
            g(w - 1, 0), edge, edge
        ]));

    let first_real_row = iter::empty()
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, edge, g(0, 0),
            edge, edge, g(0, 1)
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            edge, g(0, 0), g(1, 0),
            edge, g(0, 1), g(1, 1)
        ]))
        .chain((1..(w - 1)).map(move |x| u_bits![
            edge, edge, edge,
            g(x - 1, 0), g(x, 0), g(x + 1, 0),
            g(x - 1, 1), g(x, 1), g(x + 1, 1)
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            g(w - 2, 0), g(w - 1, 0), edge,
            g(w - 2, 1), g(w - 1, 1), edge
        ]))
        .chain(iter::once(u_bits![
            edge, edge, edge,
            g(w - 1, 0), edge, edge,
            g(w - 1, 1), edge, edge
        ]));

    let middle_rows = (1..(h - 1)).flat_map(move |y| {
        iter::empty()
            .chain(
            iter::once(u_bits![
                edge, edge, g(0, y - 1),
                edge, edge, g(0, y),
                edge, edge, g(0, y + 1)
            ]))
            .chain(
            iter::once(u_bits![
                edge, g(0, y - 1), g(1, y - 1),
                edge, g(0, y), g(1, y),
                edge, g(0, y + 1), g(1, y + 1)
            ]))
            .chain((1..(w - 1)).map(move |x| u_bits![
                g(x - 1, y - 1), g(x, y - 1), g(x + 1, y - 1),
                g(x - 1, y), g(x, y), g(x + 1, y),
                g(x - 1, y + 1), g(x, y + 1), g(x + 1, y + 1)
            ]))
            .chain(iter::once(u_bits![
                g(w - 2, y - 1), g(w - 1, y - 1), edge,
                g(w - 2, y), g(w - 1, y), edge,
                g(w - 2, y + 1), g(w - 1, y + 1), edge
            ]))
            .chain(iter::once(u_bits![
                g(w - 1, y - 1), edge, edge,
                g(w - 1, y), edge, edge,
                g(w - 1, y + 1), edge, edge
            ]))
    });

    let last_real_row = iter::empty()
        .chain(iter::once(u_bits![
            edge, edge, g(0, h - 2),
            edge, edge, g(0, h - 1),
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            edge, g(0, h - 2), g(1, h - 2),
            edge, g(0, h - 1), g(1, h - 1),
            edge, edge, edge
        ]))
        .chain((1..(w - 1)).map(move |x| u_bits![
            g(x - 1, h - 2), g(x, h - 2), g(x + 1, h - 2),
            g(x - 1, h - 1), g(x, h - 1), g(x + 1, h - 1),
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            g(w - 2, h - 2), g(w - 1, h - 2), edge,
            g(w - 2, h - 1), g(w - 1, h - 1), edge,
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            g(w - 1, h - 2), edge, edge,
            g(w - 1, h - 1), edge, edge,
            edge, edge, edge
        ]));

    let last_extend_row = iter::empty()
        .chain(iter::once(u_bits![
            edge, edge, g(0, h - 1),
            edge, edge, edge,
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            edge, g(0, h - 1), g(1, h - 1),
            edge, edge, edge,
            edge, edge, edge
        ]))
        .chain((1..(w - 1)).map(move |x| u_bits![
            g(x - 1, h - 1), g(x, h - 1), g(x + 1, h - 1),
            edge, edge, edge,
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            g(w - 2, h - 1), g(w - 1, h - 1), edge,
            edge, edge, edge,
            edge, edge, edge
        ]))
        .chain(iter::once(u_bits![
            g(w - 1, h - 1), edge, edge,
            edge, edge, edge,
            edge, edge, edge
        ]));

    iter::empty()
        .chain(first_extend_row)
        .chain(first_real_row)
        .chain(middle_rows)
        .chain(last_real_row)
        .chain(last_extend_row)
}

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize) -> usize {
    let (alg, grid) = input;

    let mut w = grid[0].len();
    let mut h = grid.len();
    let mut pixels = Vec::from_iter(
        grid.iter()
            .enumerate()
            .flat_map(|(_y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(_x, lit)| {
                        *lit
                    })
            })
    );

    let mut fill_pixel = false;

    for _step in 0..steps {
        pixels = windows(w, h, &pixels, fill_pixel)
            .map(|idx| {
                alg[idx]
            })
            .collect();

        w += 2;
        h += 2;

        fill_pixel = if fill_pixel {
            alg[511]
        } else {
            alg[0]
        };
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
