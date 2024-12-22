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
        let mut sequences = HashMap::new();

        for line in input.lines() {
            let mut number: usize = line.parse().unwrap();

            let mut sequence = 0;
            let mut used = vec![false; 2usize.pow(20)];

            for i in 0..2000 {
                let mut new_number = number;
                new_number = (new_number ^ (new_number * 64)) % 16777216;
                new_number = (new_number ^ (new_number / 32)) % 16777216;
                new_number = (new_number ^ (new_number * 2048)) % 16777216;

                let prev_bananas = number % 10;
                let bananas = new_number % 10;
                let diff = bananas as isize - prev_bananas as isize;

                let value = diff as usize & 0b11111;
                sequence = ((sequence << 5) | value) & ((1 << 20) - 1);

                if i >= 3 && !used[sequence] {
                    *sequences.entry(sequence).or_default() += bananas;
                    used[sequence] = true;
                }

                number = new_number;
            }
        }

        *sequences.values().max().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
