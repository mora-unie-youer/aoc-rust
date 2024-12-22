use std::collections::HashMap;

use aoc_2024::*;

const DAY: i32 = 22;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| {
            let mut number: usize = line.parse().unwrap();

            for _ in 0..2000 {
                number = (number ^ (number * 64)) % 16777216;
                number = (number ^ (number / 32)) % 16777216;
                number = (number ^ (number * 2048)) % 16777216;
            }

            number
        })
        .sum();

    let solution2: Solution = {
        let mut sequences: HashMap<[isize; 4], usize> = HashMap::new();

        for line in input.lines() {
            let mut number: usize = line.parse().unwrap();
            let mut sequence = [0, 0, 0, 0];
            let mut best_sequences = HashMap::new();

            for i in 0..2000 {
                let mut new_number = number;
                new_number = (new_number ^ (new_number * 64)) % 16777216;
                new_number = (new_number ^ (new_number / 32)) % 16777216;
                new_number = (new_number ^ (new_number * 2048)) % 16777216;

                let prev_bananas = number % 10;
                let bananas = new_number % 10;
                let diff = bananas as isize - prev_bananas as isize;
                sequence.rotate_left(1);
                sequence[3] = diff;

                if i >= 3 {
                    best_sequences.entry(sequence).or_insert(bananas);
                }

                number = new_number;
            }

            for (seq, bananas) in best_sequences {
                *sequences.entry(seq).or_default() += bananas;
            }
        }

        *sequences.values().max().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
