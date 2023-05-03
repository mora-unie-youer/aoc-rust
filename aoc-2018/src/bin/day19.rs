#![feature(let_chains)]

use aoc_2018::*;

// Took code from day 16
const DAY: i32 = 19;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum OperationType {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value {
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "borr" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => unreachable!(),
        }
    }
}

struct Operation {
    ty: OperationType,
    in1: usize,
    in2: usize,
    out: usize,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        Self {
            ty: parts.next().unwrap().into(),
            in1: parts.next().unwrap().parse().unwrap(),
            in2: parts.next().unwrap().parse().unwrap(),
            out: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Operation {
    fn execute(&self, regs: &mut [usize; 6]) {
        match self.ty {
            OperationType::Addr => regs[self.out] = regs[self.in1] + regs[self.in2],
            OperationType::Addi => regs[self.out] = regs[self.in1] + self.in2,
            OperationType::Mulr => regs[self.out] = regs[self.in1] * regs[self.in2],
            OperationType::Muli => regs[self.out] = regs[self.in1] * self.in2,
            OperationType::Banr => regs[self.out] = regs[self.in1] & regs[self.in2],
            OperationType::Bani => regs[self.out] = regs[self.in1] & self.in2,
            OperationType::Borr => regs[self.out] = regs[self.in1] | regs[self.in2],
            OperationType::Bori => regs[self.out] = regs[self.in1] | self.in2,
            OperationType::Setr => regs[self.out] = regs[self.in1],
            OperationType::Seti => regs[self.out] = self.in1,
            OperationType::Gtir => regs[self.out] = (self.in1 > regs[self.in2]) as _,
            OperationType::Gtri => regs[self.out] = (regs[self.in1] > self.in2) as _,
            OperationType::Gtrr => regs[self.out] = (regs[self.in1] > regs[self.in2]) as _,
            OperationType::Eqir => regs[self.out] = (self.in1 == regs[self.in2]) as _,
            OperationType::Eqri => regs[self.out] = (regs[self.in1] == self.in2) as _,
            OperationType::Eqrr => regs[self.out] = (regs[self.in1] == regs[self.in2]) as _,
        }
    }
}

struct Cpu {
    ip_index: usize,
    registers: [usize; 6],
}

impl Cpu {
    fn new(ip_index: usize) -> Self {
        Self {
            ip_index,
            registers: Default::default(),
        }
    }

    fn run(&mut self, program: &[Operation]) {
        loop {
            let ip = self.registers[self.ip_index];
            if ip >= program.len() {
                break;
            }

            program[ip].execute(&mut self.registers);
            self.registers[self.ip_index] += 1;
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (ip_index, program) = {
        let mut lines = input.lines();
        let ip_index = lines.next().unwrap()[4..].parse().unwrap();
        let program: Vec<_> = lines.map(Operation::from).collect();
        (ip_index, program)
    };

    let solution1: Solution = {
        let mut cpu = Cpu::new(ip_index);
        cpu.run(&program);
        cpu.registers[0]
    };

    let solution2: Solution = {
        // Reversed assembler and understood, that this algo is sum of factors
        let a0 = 2 * 2 * 19 * 11 + 4 * 22 + 16; // when a == 0
        let a1 = (27 * 28 + 29) * 30 * 14 * 32; // added when a == 1
        let n = a0 + a1;
        (1..=n).filter(|v| n % v == 0).sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
