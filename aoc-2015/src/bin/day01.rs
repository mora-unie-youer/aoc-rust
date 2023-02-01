#![feature(control_flow_enum)]

use std::ops::ControlFlow;

use aoc_2015::*;

const DAY: i32 = 1;
type Solution = isize;

fn main() {
    let input = get_input_text(DAY);

    let steps = input.chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });

    let solution1: Solution = steps.clone().sum();
    let solution2: Solution = steps
        .clone()
        .enumerate()
        .try_fold(0, |level, (i, step)| {
            let new_level = level + step;
            if new_level < 0 {
                ControlFlow::Break(i + 1)
            } else {
                ControlFlow::Continue(new_level)
            }
        })
        .break_value()
        .unwrap() as Solution;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
