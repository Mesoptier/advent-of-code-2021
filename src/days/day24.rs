use std::str::FromStr;
use itertools::Itertools;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<(i32, i32, i32)> {
    let mut chunks = vec![];

    for line in input.lines() {
        match line {
            "inp w" => chunks.push(vec![]),
            instruction => chunks.last_mut().unwrap().push(instruction),
        }
    }

    chunks.into_iter()
        .map(|chunk| {
            let mut it = chunk.into_iter();
            (
                i32::from_str(it.nth(3).unwrap().trim_start_matches("div z ")).unwrap(),
                i32::from_str(it.nth(0).unwrap().trim_start_matches("add x ")).unwrap(),
                i32::from_str(it.nth(9).unwrap().trim_start_matches("add y ")).unwrap(),
            )
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vec<(i32, i32, i32)>) -> usize {
    solve_both_parts(input, false)
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Vec<(i32, i32, i32)>) -> usize {
    solve_both_parts(input, true)
}

pub fn solve_both_parts(input: &Vec<(i32, i32, i32)>, minimize: bool) -> usize {
    /*
    Working out:
    ------------------------------
    inp w
    mul x 0      -> x = 0        |
    add x z      -> x += z       |
    mod x 26     -> x %= 26      | -> x = z % 26
    add x B      -> x += B       |
    eql x w      -> x = (x == w) |
    eql x 0      -> x = (x == 0) | -> x = ((x + B) != w)     // x matches 0 | 1
    mul y 0      -> y = 0        |
    add y 25     -> y += 25      |
    mul y x      -> y *= x       |
    add y 1      -> y += 1       | -> y = (25 * x) + 1       // y matches 1 | 26
    div z A      -> z /= A       // A matches 1 | 26
    mul z y      -> z *= y
    mul y 0      -> y = 0        |
    add y w      -> y += w       |
    add y 8      -> y += C       |
    mul y x      -> y *= x       | -> y = (w + C) * x        // y matches 0 | (w + C)
    add z y      -> z += y
    ------------------------------

    Simplifies to:
    x = ((z % 26 + B) != w)
    z = z / A
    if x {
        z = z * 26 + w + C
    }
    z

    Facts about the constants:
    for all (a, b, c):
        a ∈ {1, 26}
        a == 1  <=>  b > 10
        b > 10   =>  z % 26 + b != w

    Which further simplifies the code:
    if a == 1:
        z = z * 26 + w + c
        z
    else:
        x = (z % 26 + B) != w
        z = z / 26
        if x:
            z = z * 26 + w + c
        z

    There are 7 cases where a == 1, and 7 cases where a == 26. In the cases where a == 1, we "push"
    a base 26 digit (w + c) to the end of z. So for z to equal 0 at the end, in the other cases
    where a == 26 we must "pop" those digits again, and so (z % 26 + B) must equal w.

    */

    /// Evaluate the state. Returns `None` if we should have "popped", but didn't. Otherwise returns
    /// the `Some(z)` with the `z` for the next step.
    fn eval_state(z: i32, w: i32, consts: (i32, i32, i32)) -> Option<i32> {
        let (a, b, c) = consts;

        if a == 26 {
            if z % 26 + b != w {
                None
            } else {
                Some(z / 26)
            }
        } else {
            Some(z * 26 + w + c)
        }
    }

    fn optimize(z: i32, i: usize, input: &Vec<(i32, i32, i32)>, minimize: bool) -> Option<usize> {
        // Base case
        if i == 14 {
            return if z == 0 {
                Some(0)
            } else {
                None
            };
        }

        let consts = input[i];

        // Search through 1..=9 range with smallest/largest first depending on whether we're
        // minimizing or maximizing.
        let mut ws = (1..=9).collect_vec();
        if !minimize {
            ws.reverse();
        }

        for w in ws {
            // Check if state is valid and if so, what the next value of z is.
            if let Some(z) = eval_state(z, w, consts) {
                // Try to optimize the remaining digits, if possible.
                if let Some(n) = optimize(z, i + 1, input, minimize) {
                    return Some(n + w as usize * 10usize.pow(13 - i as u32));
                }
            }
        }

        None
    }

    optimize(0, 0, input, minimize).unwrap()
}
