use std::collections::HashMap;

use aoc_2017::*;

const DAY: i32 = 23;
type Solution = usize;

enum Operand<'input> {
    Register(&'input str),
    Integer(isize),
}

impl<'input> From<&'input str> for Operand<'input> {
    fn from(value: &'input str) -> Self {
        if let Ok(int) = value.parse() {
            Self::Integer(int)
        } else {
            Self::Register(value)
        }
    }
}

impl<'input> Operand<'input> {
    fn register(&self) -> Option<&str> {
        match self {
            Self::Register(v) => Some(v),
            _ => None,
        }
    }

    fn value(&self, registers: &HashMap<&'input str, isize>) -> isize {
        match *self {
            Self::Integer(v) => v,
            Self::Register(v) => *registers.get(v).unwrap_or(&0),
        }
    }
}

enum Instruction<'input> {
    Set(Operand<'input>, Operand<'input>),
    Jnz(Operand<'input>, Operand<'input>),
    Mul(Operand<'input>, Operand<'input>),
    Sub(Operand<'input>, Operand<'input>),
}

impl<'input> From<&'input str> for Instruction<'input> {
    fn from(input: &'input str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "sub" => Self::Sub(splits[1].into(), splits[2].into()),
            "jnz" => Self::Jnz(splits[1].into(), splits[2].into()),
            "mul" => Self::Mul(splits[1].into(), splits[2].into()),
            "set" => Self::Set(splits[1].into(), splits[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Cpu<'input> {
    ip: usize,
    registers: HashMap<&'input str, isize>,

    mul_count: usize,
}

impl<'input> Cpu<'input> {
    fn run(&mut self, instructions: &'input [Instruction]) {
        while self.ip < instructions.len() {
            self.execute_instruction(&instructions[self.ip]);
        }
    }

    fn execute_instruction(&mut self, instruction: &'input Instruction) {
        match instruction {
            Instruction::Set(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register = value;
            }
            Instruction::Sub(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register -= value;
            }
            Instruction::Mul(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register *= value;
                self.mul_count += 1;
            }
            Instruction::Jnz(op1, op2) => {
                let offset = op2.value(&self.registers);
                if op1.value(&self.registers) != 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                    return;
                }
            }
        }

        self.ip += 1;
    }
}

fn solve_part2() -> Solution {
    // let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (1, 0, 0, 0, 0, 0, 0, 0);
    //
    // b = 65;
    // // if a != 0 -> is true for part2
    // b = b * 100 + 100000;
    // c = b + 17000;
    //
    // loop {
    //     f = 1;
    //     d = 2;
    //
    //     // There's prime checker
    //     // e = 2; -> this is useless for our algorithm
    //     while d * d <= b {
    //         if b % d == 0 {
    //             f = 0;
    //             break;
    //         }
    //
    //         d += 1;
    //     }
    //
    //     if f == 0 {
    //         h += 1;
    //     }
    //
    //     g = b - c;
    //     b += 17;
    //     if g == 0 {
    //         break h;
    //     }
    // }

    // So all this code shrinks into the code of checking for non-prime numbers in range
    // [106500; 123500] step by 17. Result must be count of non-prime numbers
    (106500..=123500)
        .step_by(17)
        .filter(|&num| {
            let mut i = 2;

            while i * i < num {
                if num % i == 0 {
                    return true;
                }

                i += 1;
            }

            false
        })
        .count()
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut cpu = Cpu::default();
        cpu.run(&instructions);
        cpu.mul_count
    };

    let solution2: Solution = solve_part2();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
