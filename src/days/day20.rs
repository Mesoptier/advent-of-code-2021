use bitvec::prelude::*;
use hashbrown::HashSet;

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

fn print_grid(pixels: &HashSet<(i32, i32)>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", match pixels.contains(&(x, y)) {
                true => '#',
                false => '.',
            });
        }
        println!();
    }
}

fn solve(input: &(Vec<bool>, Vec<Vec<bool>>), steps: usize) -> usize {
    let (alg, grid) = input;

    let mut pixels = HashSet::new();
    let mut fill_pixel = false;
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (y, row) in grid.iter().enumerate() {
        for (x, lit) in row.iter().enumerate() {
            if *lit {
                pixels.insert((x as i32, y as i32));
                min_x = min_x.min(x as i32);
                max_x = max_x.max(x as i32);
                min_y = min_y.min(y as i32);
                max_y = max_y.max(y as i32);
            }
        }
    }

    for _step in 0..steps {
        let mut next_pixels = HashSet::new();
        let mut next_min_x = i32::MAX;
        let mut next_max_x = i32::MIN;
        let mut next_min_y = i32::MAX;
        let mut next_max_y = i32::MIN;

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let mut bits = bitvec![Msb0, usize;];
                for ny in (y - 1)..=(y + 1) {
                    for nx in (x - 1)..=(x + 1) {
                        let prev_pixel = if nx < min_x || max_x < nx || ny < min_y || max_y < ny {
                            fill_pixel
                        } else {
                            pixels.contains(&(nx, ny))
                        };

                        bits.push(prev_pixel);
                    }
                }

                let idx = bits.load::<usize>();
                if alg[idx] {
                    next_pixels.insert((x, y));
                    next_min_x = next_min_x.min(x);
                    next_max_x = next_max_x.max(x);
                    next_min_y = next_min_y.min(y);
                    next_max_y = next_max_y.max(y);
                }
            }
        }

        fill_pixel = if fill_pixel {
            alg[512 - 1]
        } else {
            alg[0]
        };

        // Swap state
        pixels = next_pixels;
        min_x = next_min_x;
        max_x = next_max_x;
        min_y = next_min_y;
        max_y = next_max_y;
    }

    pixels.len()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(Vec<bool>, Vec<Vec<bool>>)) -> usize {
    solve(input, 50)
}
