use std::collections::HashMap;

use aoc_2019::*;

const DAY: i32 = 3;
type Solution = usize;

fn parse_wire(input: &str) -> HashMap<(isize, isize), usize> {
    let (mut x, mut y) = (0, 0);
    let mut visited = HashMap::new();
    let mut total_steps = 0;

    for instruction in input.split(',') {
        let steps = instruction[1..].parse().unwrap();
        let (dx, dy) = match instruction.chars().next().unwrap() {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            x += dx;
            y += dy;
            total_steps += 1;
            visited.insert((x, y), total_steps);
        }
    }

    visited
}

fn main() {
    let input = get_input_text(DAY);
    let (first, second) = input.trim().split_once('\n').unwrap();

    let first = parse_wire(first);
    let second = parse_wire(second);

    let solution1: Solution = first
        .keys()
        .filter(|pos| second.contains_key(pos))
        .map(|(x, y)| x.unsigned_abs() + y.unsigned_abs())
        .min()
        .unwrap();
    let solution2: Solution = first
        .iter()
        .filter(|(k, _)| second.contains_key(k))
        .map(|(k, v)| v + second[k])
        .min()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
