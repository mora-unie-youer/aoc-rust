use std::collections::VecDeque;

use aoc_2017::*;

const DAY: i32 = 16;
type Solution = String;

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            's' => Self::Spin(value[1..].parse().unwrap()),
            'x' => {
                let (first, second) = value[1..].split_once('/').unwrap();
                Self::Exchange(first.parse().unwrap(), second.parse().unwrap())
            }
            'p' => {
                let (first, second) = value[1..].split_once('/').unwrap();
                Self::Partner(
                    first.chars().next().unwrap(),
                    second.chars().next().unwrap(),
                )
            }
            _ => unreachable!(),
        }
    }
}

fn dance(programs: &mut VecDeque<char>, instructions: &[Instruction]) {
    for instruction in instructions {
        match *instruction {
            Instruction::Spin(v) => programs.rotate_right(v),
            Instruction::Exchange(i, j) => {
                programs.swap(i, j);
            }
            Instruction::Partner(a, b) => {
                let i = programs.iter().position(|&ch| ch == a).unwrap();
                let j = programs.iter().position(|&ch| ch == b).unwrap();
                programs.swap(i, j);
            }
        }
    }
}

fn main() {
    let start = "abcdefghijklmnop";
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.trim().split(',').map(Instruction::from).collect();

    let solution1: Solution = {
        let mut programs: VecDeque<char> = start.chars().collect();
        dance(&mut programs, &instructions);
        programs.iter().collect()
    };

    let solution2: Solution = {
        let mut programs: VecDeque<char> = start.chars().collect();
        let mut i = 0;

        while i < 1_000_000_000 {
            dance(&mut programs, &instructions);
            i += 1;

            // Solve the cycle
            if programs.iter().collect::<String>() == start {
                let cycles = 1_000_000_000 / i;
                i *= cycles;
            }
        }

        programs.iter().collect()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
