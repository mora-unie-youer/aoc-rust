use std::collections::HashMap;

use itertools::Itertools;

use aoc_2024::*;

const DAY: i32 = 1;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unzip::<Solution, Solution, _, _>();
    a.sort();
    b.sort();

    let solution1: Solution = a.iter().zip(&b).map(|(&a, &b)| a.abs_diff(b)).sum();
    let solution2: Solution = {
        let counts: HashMap<usize, usize> = b
            .into_iter()
            .dedup_with_count()
            .map(|(a, b)| (b, a))
            .collect();

        a.into_iter()
            .map(|x| x * counts.get(&x).copied().unwrap_or(0))
            .sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
