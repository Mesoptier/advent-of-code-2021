use bitvec::prelude::*;

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

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize) -> usize {
    let (alg, grid) = input;

    let w = grid[0].len() + 2 * steps;
    let h = grid.len() + 2 * steps;
    let mut pixels = bitvec![Msb0, usize; 0; w * h];
    for (y, row) in grid.iter().enumerate() {
        for (x, lit) in row.iter().enumerate() {
            pixels.set((x + steps) + w * (y + steps), *lit);
        }
    }

    let mut fill_pixel = false;

    for _step in 0..steps {
        let mut next_pixels = bitvec![Msb0, usize; 0; w * h];

        for y in 0..h {
            for x in 0..w {
                let mut bits = bitvec![Msb0, usize;];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let prev_pixel = if {
                            (x == 0 && dx == -1)
                                || (x == w - 1 && dx == 1)
                                || (y == 0 && dy == -1)
                                || (y == h - 1 && dy == 1)
                        } {
                            fill_pixel
                        } else {
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            pixels[nx + w * ny]
                        };

                        bits.push(prev_pixel);
                    }
                }

                let idx = bits.load::<usize>();
                next_pixels.set(x + w * y, alg[idx]);
            }
        }

        fill_pixel = if fill_pixel {
            alg[512 - 1]
        } else {
            alg[0]
        };

        pixels = next_pixels;
    }

    pixels.count_ones()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 50)
}
