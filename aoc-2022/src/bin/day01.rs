use std::cmp::Reverse;

use aoc_2022::*;

const DAY: i32 = 1;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let mut elfs: Vec<usize> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect();
    elfs.sort_by_key(|&elf| Reverse(elf));

    let solution1: Solution = *elfs.first().unwrap();
    let solution2: Solution = elfs.into_iter().take(3).sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
