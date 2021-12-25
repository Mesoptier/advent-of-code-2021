use itertools::Itertools;

#[aoc_generator(day25)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .into_iter()
        .map(|line| {
            line.chars().collect_vec()
        })
        .collect_vec()
}

#[aoc(day25, part1)]
fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let mut step = 0;

    let width = input[0].len();
    let height = input.len();

    let mut grid = input.clone();

    loop {
        let mut next_grid = vec![vec!['.'; width]; height];
        let mut has_moved = false;

        for y in 0..height {
            for x in 0..width {
                let nx = (x + 1) % width;
                if grid[y][x] == '>' {
                    if grid[y][nx] == '.' {
                        next_grid[y][nx] = '>';
                        has_moved = true;
                    } else {
                        next_grid[y][x] = '>';
                    }
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let ny = (y + 1) % height;
                if grid[y][x] == 'v' {
                    if grid[ny][x] != 'v' && next_grid[ny][x] == '.' {
                        next_grid[ny][x] = 'v';
                        has_moved = true;
                    } else {
                        next_grid[y][x] = 'v';
                    }
                }
            }
        }

        step += 1;

        if !has_moved {
            return step;
        }

        std::mem::swap(&mut grid, &mut next_grid);
    }
}
