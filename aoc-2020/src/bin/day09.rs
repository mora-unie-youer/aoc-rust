use std::cmp::Ordering;

use aoc_2020::*;
use itertools::Itertools;

const DAY: i32 = 9;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = {
        numbers
            .windows(26)
            .find(|window| {
                window[..25]
                    .iter()
                    .tuple_combinations()
                    .all(|(a, b)| a + b != window[25])
            })
            .unwrap()[25]
    };

    let solution2: Solution = {
        let (mut i, mut j) = (0, 0);

        while j < numbers.len() {
            let sum: usize = numbers[i..=j].iter().sum();
            match sum.cmp(&solution1) {
                Ordering::Greater => i += 1,
                Ordering::Less => j += 1,
                Ordering::Equal => break,
            }
        }

        let slice = &numbers[i..=j];

        let (min, max) = slice.iter().minmax().into_option().unwrap();
        min + max
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
