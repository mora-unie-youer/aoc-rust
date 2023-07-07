use std::collections::HashSet;

use aoc_2021::*;

const DAY: i32 = 8;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .map(|digits| {
            digits
                .split_ascii_whitespace()
                .map(|digit| digit.len())
                .filter(|&len| len == 2 || len == 3 || len == 4 || len == 7)
                .count()
        })
        .sum();

    let solution2: Solution = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(samples, output)| {
            let mut samples: Vec<_> = samples.split_ascii_whitespace().collect();
            samples.sort_by_key(|sample| sample.len());

            let one: HashSet<char> = samples[0].chars().collect();
            let _seven: HashSet<char> = samples[1].chars().collect();
            let four: HashSet<char> = samples[2].chars().collect();
            let _eight: HashSet<char> = samples.last().unwrap().chars().collect();

            let mut number = 0;
            for digit in output.split_ascii_whitespace() {
                let digit: HashSet<char> = digit.chars().collect();
                let common_with_one = one.intersection(&digit).count();
                let common_with_four = four.intersection(&digit).count();

                let digit = match (digit.len(), common_with_one, common_with_four) {
                    (2, _, _) => 1,
                    (3, _, _) => 7,
                    (4, _, _) => 4,
                    (7, _, _) => 8,

                    (5, 2, _) => 3,
                    (5, _, 2) => 2,
                    (5, _, _) => 5,

                    (6, 1, 3) => 6,
                    (6, 2, 4) => 9,
                    (6, _, _) => 0,
                    _ => unreachable!(),
                };

                number = number * 10 + digit;
            }

            number
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
