use std::iter::successors;

use aoc_2019::*;

const DAY: i32 = 1;
type Solution = isize;

fn main() {
    let input = get_input_text(DAY);
    let masses: Vec<Solution> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = masses.iter().map(|mass| mass / 3 - 2).sum();
    let solution2: Solution = masses
        .iter()
        .map(|&mass| {
            successors(Some(mass), |&v| Some(v / 3 - 2))
                .skip(1)
                .take_while(|&v| v > 0)
                .sum::<Solution>()
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
