#![feature(control_flow_enum)]

use std::{collections::HashSet, ops::ControlFlow};

use aoc_2018::*;

const DAY: i32 = 1;
type Solution = isize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum();

    let solution2: Solution = {
        let mut numbers = HashSet::new();
        numbers.insert(0);

        input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .cycle()
            .try_fold(0, |acc, value| {
                let acc = acc + value;

                if numbers.insert(acc) {
                    ControlFlow::Continue(acc)
                } else {
                    ControlFlow::Break(acc)
                }
            })
            .break_value()
            .unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
