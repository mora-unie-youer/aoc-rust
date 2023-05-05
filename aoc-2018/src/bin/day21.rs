#![feature(let_chains)]

use std::collections::HashSet;

use aoc_2018::*;

const DAY: i32 = 21;
type Solution = usize;

fn extract_number<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    nth_line: usize,
    nth_word: usize,
) -> Option<usize> {
    lines
        .nth(nth_line)?
        .split_ascii_whitespace()
        .nth(nth_word)?
        .parse()
        .ok()
}

fn main() {
    let input = get_input_text(DAY);
    let mut lines = input.lines();

    let c0 = extract_number(&mut lines, 7, 2).unwrap(); // bori reg[3] operand
    let c1 = extract_number(&mut lines, 0, 1).unwrap(); // seti reg[3] operand
    let c2 = extract_number(&mut lines, 0, 2).unwrap(); // bani reg[4] operand
    let c3 = extract_number(&mut lines, 1, 2).unwrap(); // bani reg[3] operand
    let c4 = extract_number(&mut lines, 0, 2).unwrap(); // muli reg[3] operand
    let c5 = extract_number(&mut lines, 0, 2).unwrap(); // bani reg[3] operand

    let mut seen = HashSet::new();
    let mut first = 0;
    let mut last = 0;

    let mut r1 = 0;
    loop {
        let mut r2 = r1 | c0; // bori reg[4] operation
        r1 = c1;              // seti reg[3] operation

        while r2 > 0 {
            // 7-14 lines of assembly
            r1 = (((r1 + (r2 & c2)) & c3) * c4) & c5;
            // Easy to use "comparison" of "gtir 256 4 5"
            r2 >>= 8;
        }

        // If it is first value -> storing first (first solution)
        if seen.is_empty() {
            first = r1;
        }

        // If there's a duplicate -> breaking the loop
        if !seen.insert(r1) {
            break;
        }

        // We need to save last non-dup value (second solution)
        last = r1;
    }

    let solution1: Solution = first;
    let solution2: Solution = last;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
