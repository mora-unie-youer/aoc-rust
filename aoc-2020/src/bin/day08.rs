use std::collections::HashSet;

use aoc_2020::*;

const DAY: i32 = 8;
type Solution = isize;

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (instruction, value) = input.split_once(' ').unwrap();
        let value = value.parse().unwrap();

        match instruction {
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            "nop" => Self::Nop(value),
            _ => unreachable!(),
        }
    }
}

fn run(instructions: &[Instruction]) -> (bool, isize) {
    let mut ip = 0;
    let mut visited = HashSet::new();

    let mut acc = 0;
    while ip < instructions.len() as isize && visited.insert(ip) {
        match instructions[ip as usize] {
            Instruction::Acc(v) => acc += v,
            Instruction::Jmp(v) => ip += v - 1,
            _ => (),
        }

        ip += 1;
    }

    (ip >= instructions.len() as isize, acc)
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = run(&instructions).1;
    let solution2: Solution = instructions
        .iter()
        .enumerate()
        .find_map(|(i, instruction)| {
            if matches!(instruction, Instruction::Jmp(_) | Instruction::Nop(_)) {
                let mut new_instructions = instructions.clone();

                match instruction {
                    Instruction::Jmp(v) => new_instructions[i] = Instruction::Nop(*v),
                    Instruction::Nop(v) => new_instructions[i] = Instruction::Jmp(*v),
                    _ => (),
                }

                let (done, acc) = run(&new_instructions);
                if done {
                    Some(acc)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
