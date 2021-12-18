#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (usize, Vec<u32>) {
    let mut lines = input.lines().peekable();
    let num_bits = lines.peek().unwrap().len();
    let input = lines
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    (num_bits, input)
}

/// Gets the `i`-th of `n`
fn get_bit(n: u32, i: usize) -> u32 {
    (n & (1 << i)) >> i
}

/// Sets the `i`-th bit of `n` to `b`
fn set_bit(n: u32, i: usize, b: bool) -> u32 {
    n | ((b as u32) << i)
}

/// Inverts the last `i` bits of `n`
fn invert_bits(n: u32, i: usize) -> u32 {
    !n & ((1 << i) - 1)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(usize, Vec<u32>)) -> u32 {
    let num_bits = input.0;
    let input = input.1.as_slice();

    let total_count = input.len() as u32;

    let one_frequencies = input
        .into_iter()
        .fold(vec![0; num_bits], |frequencies, bits| {
            frequencies
                .into_iter()
                .enumerate()
                .map(|(i, freq)| freq + get_bit(*bits, i))
                .collect()
        });

    let gamma = one_frequencies
        .into_iter()
        .enumerate()
        .fold(0, |n, (i, freq)| {
            set_bit(n, i, freq >= total_count / 2)
        });

    let epsilon = invert_bits(gamma, num_bits);
    gamma * epsilon
}

// #[aoc(day3, part2)]
// pub fn solve_part2(input: &[BitVec<Msb0>]) -> u32 {
//     let oxygen_generator_rating = search_part2_number(Vec::from(input), true);
//     let co2_scrubber_rating = search_part2_number(Vec::from(input), false);
//     oxygen_generator_rating * co2_scrubber_rating
// }
//
// fn search_part2_number(mut numbers: Vec<BitVec<Msb0>>, keep_most_common_bit: bool) -> u32 {
//     let mut bit_pos = 0;
//     while numbers.len() != 1 {
//         let one_frequency = numbers.iter()
//             .filter(|n| n[bit_pos])
//             .count();
//         let zero_frequency = numbers.len() - one_frequency;
//         let filter_bit = (one_frequency >= zero_frequency) == keep_most_common_bit;
//
//         numbers = numbers.into_iter()
//             .filter(|n| n[bit_pos] == filter_bit)
//             .collect();
//
//         bit_pos += 1;
//     }
//
//     numbers[0].load::<u32>()
// }

// #[aoc(day3, part2, bitmask)]
// pub fn solve_part2_bitmask(input: &[BitVec<Msb0>]) -> u32 {
//     fn search_number(input: &[BitVec<Msb0>], keep_common_bit: bool) -> u32 {
//         let num_bits = input[0].len();
//
//         let mut filter_input = vec![true; input.len()];
//         let mut filter: BitVec<Msb0> = bitvec![Msb0, u32;];
//
//         for bit_pos in 0..num_bits {
//             let mut total_freq = 0u32;
//             let mut one_freq = 0u32;
//
//             for i in 0..input.len() {
//                 if !filter_input[i] {
//                     continue;
//                 }
//
//                 let n = &input[i];
//                 if n[0..bit_pos] != filter {
//                     filter_input[i] = false;
//                     continue;
//                 }
//
//                 total_freq += 1;
//
//                 if n[bit_pos] {
//                     one_freq += 1;
//                 }
//             }
//
//             let zero_freq = total_freq - one_freq;
//
//             let filter_bit = if one_freq == 0 {
//                 false
//             } else if zero_freq == 0 {
//                 true
//             } else {
//                 (one_freq >= zero_freq) == keep_common_bit
//             };
//
//             filter.push(filter_bit);
//         }
//
//         filter.load::<u32>()
//     }
//
//     search_number(input, true) * search_number(input, false)
// }
