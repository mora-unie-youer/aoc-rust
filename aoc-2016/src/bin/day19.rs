use std::collections::VecDeque;

use aoc_2016::*;

const DAY: i32 = 19;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let elves = input.trim().parse().unwrap();

    let solution1: Solution = {
        let mut elves: Vec<Option<usize>> = (1..=elves).map(Option::Some).collect();

        while elves.len() != 1 {
            for elf in elves.iter_mut().skip(1).step_by(2) {
                *elf = None;
            }

            if elves.len() % 2 == 1 {
                elves[0] = None;
            }

            // Retaining array, so elves without presents leave the circle
            elves.retain(|elf| elf.is_some());
        }

        elves[0].unwrap()
    };

    let solution2: Solution = {
        let mut left: VecDeque<usize> = (1..elves / 2 + 1).collect();
        let mut right: VecDeque<usize> = (elves / 2 + 1..=elves).collect();

        // "left" always contains more elements
        while !right.is_empty() {
            // Remove "opposite" element
            if left.len() > right.len() {
                left.pop_back();
            } else {
                right.pop_front();
            }

            // Rotate the circle
            right.push_back(left.pop_front().unwrap());
            left.push_back(right.pop_front().unwrap());
        }

        left.pop_front().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
