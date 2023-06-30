#![feature(array_windows)]

use aoc_2021::*;

const DAY: i32 = 1;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let depths: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = depths
        .array_windows()
        .map(|[a, b]| b.cmp(a))
        .filter(|ord| ord.is_gt())
        .count();

    let solution2: Solution = depths
        .windows(3)
        .map(|values| values.iter().sum())
        .collect::<Vec<usize>>()
        .array_windows()
        .map(|[a, b]| b.cmp(a))
        .filter(|ord| ord.is_gt())
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
