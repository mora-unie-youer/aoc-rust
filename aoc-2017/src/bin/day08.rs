use std::collections::HashMap;

use aoc_2017::*;

const DAY: i32 = 8;
type Solution = isize;

enum Instruction<'input> {
    Inc(&'input str, isize, &'input str, &'input str, isize),
    Dec(&'input str, isize, &'input str, &'input str, isize),
}

impl<'input> From<&'input str> for Instruction<'input> {
    fn from(input: &'input str) -> Self {
        let parts: Vec<_> = input.split(' ').collect();
        let reg1 = parts[0];
        let reg2 = parts[4];
        let op1 = parts[2].parse().unwrap();
        let op2 = parts[6].parse().unwrap();
        let cmp = parts[5];

        match parts[1] {
            "dec" => Self::Dec(reg1, op1, reg2, cmp, op2),
            "inc" => Self::Inc(reg1, op1, reg2, cmp, op2),
            _ => unreachable!(),
        }
    }
}

impl<'input> Instruction<'input> {
    fn execute(&self, registers: &mut HashMap<&'input str, isize>) -> isize {
        let (reg1, op1, reg2, cmp, op2) = match *self {
            Self::Dec(r1, o1, r2, c, o2) => (r1, o1, r2, c, o2),
            Self::Inc(r1, o1, r2, c, o2) => (r1, o1, r2, c, o2),
        };

        let reg2 = *registers.entry(reg2).or_insert(0);
        let reg1 = registers.entry(reg1).or_insert(0);
        let condition = match cmp {
            "==" => reg2 == op2,
            "!=" => reg2 != op2,
            ">" => reg2 > op2,
            ">=" => reg2 >= op2,
            "<" => reg2 < op2,
            "<=" => reg2 <= op2,
            _ => unreachable!(),
        };

        if condition {
            match self {
                Self::Dec(..) => *reg1 -= op1,
                Self::Inc(..) => *reg1 += op1,
            }
        }

        // Return new value
        *reg1
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut registers = HashMap::new();
        instructions.iter().for_each(|instruction| {
            instruction.execute(&mut registers);
        });
        *registers.values().max().unwrap()
    };

    let solution2: Solution = {
        let mut registers = HashMap::new();
        instructions.iter().fold(std::isize::MIN, |acc, instruction| {
            acc.max(instruction.execute(&mut registers))
        })
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
