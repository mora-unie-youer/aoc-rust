use aoc_2021::*;
use itertools::Itertools;

const DAY: i32 = 7;
type Solution = isize;

fn sum_n(n: isize) -> isize {
    n * (n + 1) / 2
}

fn main() {
    let input = get_input_text(DAY);

    let positions: Vec<isize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();

    let solution1: Solution = (min..=max)
        .map(|pos| positions.iter().map(|p| (p - pos).abs()).sum())
        .min()
        .unwrap();

    let solution2: Solution = (min..=max)
        .map(|pos| positions.iter().map(|p| sum_n((p - pos).abs())).sum())
        .min()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
