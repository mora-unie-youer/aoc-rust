#![feature(array_windows)]

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 2;
type Solution = usize;

fn solution(xs: &[usize]) -> bool {
    let (ords, diffs): (Vec<_>, Vec<_>) = xs
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a.cmp(b), a.abs_diff(*b)))
        .unzip();
    ords.into_iter().all_equal() && diffs.into_iter().all(|diff| (1..=3).contains(&diff))
}

fn main() {
    let input = get_input_text(DAY);
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect_vec())
        .collect_vec();

    let solution1: Solution = reports.iter().filter(|xs| solution(xs)).count();
    let solution2: Solution = reports
        .iter()
        .filter(|xs| {
            solution(xs) || (0..xs.len()).any(|i| solution(&[&xs[..i], &xs[i + 1..]].concat()))
        })
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
