use bitvec::prelude::*;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<BitVec<Msb0>> {
    input
        .lines()
        .map(|s| BitVec::from_iter(s.chars().map(|c| c == '1')))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[BitVec<Msb0>]) -> u32 {
    let num_bits = input[0].len();

    let total_count = input.len();

    // Count '1' frequencies
    let mut one_frequencies = vec![0; num_bits];
    for n in input {
        for i in n.iter_ones() {
            one_frequencies[i] += 1;
        }
    }

    let gamma: BitVec<Msb0> = BitVec::from_iter(
        one_frequencies.into_iter()
            .map(|one_frequency| one_frequency >= total_count / 2)
    );
    let epsilon = !gamma.clone();

    let gamma = gamma.load::<u32>();
    let epsilon = epsilon.load::<u32>();

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[BitVec<Msb0>]) -> u32 {
    let oxygen_generator_rating = search_part2_number(Vec::from(input), true);
    let co2_scrubber_rating = search_part2_number(Vec::from(input), false);
    oxygen_generator_rating * co2_scrubber_rating
}

fn search_part2_number(mut numbers: Vec<BitVec<Msb0>>, keep_most_common_bit: bool) -> u32 {
    let mut bit_pos = 0;
    while numbers.len() != 1 {
        let one_frequency = numbers.iter()
            .filter(|n| n[bit_pos])
            .count();
        let zero_frequency = numbers.len() - one_frequency;
        let filter_bit = (one_frequency >= zero_frequency) == keep_most_common_bit;

        numbers = numbers.into_iter()
            .filter(|n| n[bit_pos] == filter_bit)
            .collect();

        bit_pos += 1;
    }

    numbers[0].load::<u32>()
}
