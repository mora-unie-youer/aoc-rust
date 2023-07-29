use aoc_2022::*;

const DAY: i32 = 5;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut field = vec![vec![]; 9];
    for level in stacks.lines().rev().skip(1) {
        for (i, ch) in level
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|&(_, ch)| ch != ' ')
        {
            field[i].push(ch);
        }
    }

    let solution1: Solution = {
        let mut field = field.clone();
        for instruction in instructions.lines() {
            let mut parts = instruction
                .split_ascii_whitespace()
                .filter_map(|v| v.parse().ok());

            let (amount, from, to) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );

            for _ in 0..amount {
                let element = field[from - 1].pop().unwrap();
                field[to - 1].push(element);
            }
        }

        field
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .collect()
    };

    let solution2: Solution = {
        let mut field = field;
        for instruction in instructions.lines() {
            let mut parts = instruction
                .split_ascii_whitespace()
                .filter_map(|v| v.parse().ok());

            let (amount, from, to) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );

            let mut temp = Vec::with_capacity(amount);
            for _ in 0..amount {
                let element = field[from - 1].pop().unwrap();
                temp.push(element);
            }

            while let Some(element) = temp.pop() {
                field[to - 1].push(element);
            }
        }

        field
            .into_iter()
            .map(|stack| *stack.last().unwrap())
            .collect()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
