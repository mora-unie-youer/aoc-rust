use aoc_2015::*;

const DAY: i32 = 20;
type Solution = usize;

fn solve(presents: usize, multiplier: usize, limit: Option<usize>) -> Solution {
    let elves = presents / multiplier;
    let elves_max = elves / 3; // Solution is a lot smaller than input
    let mut houses = vec![0; elves_max];

    for first_elf in 1..elves_max {
        let iter = (first_elf..elves_max).step_by(first_elf);
        let add_elf = |elf| houses[elf - 1] += first_elf;
        match limit {
            Some(v) => iter.take(v).for_each(add_elf),
            None => iter.for_each(add_elf),
        }
    }

    let index = houses
        .into_iter()
        .enumerate()
        .find(|&(_, count)| count >= elves)
        .unwrap()
        .0;
    index + 1
}

fn main() {
    let input = get_input_text(DAY);
    let presents: Solution = input.trim().parse().unwrap();

    let solution1: Solution = solve(presents, 10, None);
    let solution2: Solution = solve(presents, 11, Some(50));

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
