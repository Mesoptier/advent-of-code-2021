use std::str::FromStr;
use itertools::Itertools;
use z3::{ast, Config, Context, Optimize};
use z3::ast::Ast;

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
pub fn solve_part1(input: &Vec<(i32, i32, i32)>) -> i64 {
    solve_both_parts(input, false)
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Vec<(i32, i32, i32)>) -> i64 {
    solve_both_parts(input, true)
}

pub fn solve_both_parts(input: &Vec<(i32, i32, i32)>, minimize: bool) -> i64 {
    // Working out:
    // ------------------------------
    // inp w
    // mul x 0      -> x = 0        |
    // add x z      -> x += z       |
    // mod x 26     -> x %= 26      | -> x = z % 26
    // add x B      -> x += B       |
    // eql x w      -> x = (x == w) |
    // eql x 0      -> x = (x == 0) | -> x = ((x + B) != w)     // x matches 0 | 1
    // mul y 0      -> y = 0        |
    // add y 25     -> y += 25      |
    // mul y x      -> y *= x       |
    // add y 1      -> y += 1       | -> y = (25 * x) + 1       // y matches 1 | 26
    // div z A      -> z /= A       // A matches 1 | 26
    // mul z y      -> z *= y
    // mul y 0      -> y = 0        |
    // add y w      -> y += w       |
    // add y 8      -> y += C       |
    // mul y x      -> y *= x       | -> y = (w + C) * x        // y matches 0 | (w + C)
    // add z y      -> z += y
    // ------------------------------
    // x = ((z % 26 + B) != w)
    // z = z / A
    // if x == 1 {
    //     z = z * 26 + w + C
    // }
    // z

    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let z = (0..=14)
        .map(|i| {
            ast::Int::new_const(&ctx, format!("z{}", i))
        })
        .collect_vec();

    let w = (0..14)
        .map(|i| {
            ast::Int::new_const(&ctx, format!("w{}", i))
        })
        .collect_vec();

    let solver = Optimize::new(&ctx);

    solver.assert(&z[0]._eq(&ast::Int::from_i64(&ctx, 0)));
    solver.assert(&z[14]._eq(&ast::Int::from_i64(&ctx, 0)));

    let mut model_number = ast::Int::from_i64(&ctx, 0);

    for i in 0..14 {
        // 1 <= w <= 9
        solver.assert(&w[i].ge(&ast::Int::from_i64(&ctx, 1)));
        solver.assert(&w[i].le(&ast::Int::from_i64(&ctx, 9)));

        // Combine digits into the model_number
        let p = 13 - i;
        model_number += &w[i] * 10i64.pow(p as u32);
    }

    if minimize {
        solver.minimize(&model_number);
    } else {
        solver.maximize(&model_number);
    }

    for (i, &(a, b, c)) in input.iter().enumerate() {
        let z_in = &z[i];
        let z_out = &z[i + 1];
        let w = &w[i];

        // x = ((z % 26 + B) != w)
        let x = ((z_in % 26 as i64) + b as i64)._eq(w).not();

        // x => z_out == (z_in / 26) * 26 + w + c
        solver.assert(&x.implies(&z_out._eq(&((z_in / a as i64) * 26 as i64 + w + c as i64))));

        // !x => z_out == (z_in / 26)
        solver.assert(&x.not().implies(&z_out._eq(&(z_in / a as i64))));
    }

    solver.check(&[]);
    let model = solver.get_model().unwrap();
    let result = model.eval(&model_number, true).unwrap().as_i64().unwrap();
    result
}
