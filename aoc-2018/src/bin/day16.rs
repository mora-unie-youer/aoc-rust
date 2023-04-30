#![feature(let_chains)]

use std::collections::HashSet;

use aoc_2018::*;

const DAY: i32 = 16;
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

    Unknown(usize),
}

impl From<usize> for OperationType {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Addr,
            1 => Self::Addi,
            2 => Self::Mulr,
            3 => Self::Muli,
            4 => Self::Banr,
            5 => Self::Bani,
            6 => Self::Borr,
            7 => Self::Bori,
            8 => Self::Setr,
            9 => Self::Seti,
            10 => Self::Gtir,
            11 => Self::Gtri,
            12 => Self::Gtrr,
            13 => Self::Eqir,
            14 => Self::Eqri,
            15 => Self::Eqrr,
            _ => unreachable!(),
        }
    }
}

impl OperationType {
    fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    fn unknown_opcode(&self) -> Option<usize> {
        match self {
            Self::Unknown(v) => Some(*v),
            _ => None,
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
            ty: OperationType::Unknown(parts.next().unwrap().parse().unwrap()),
            in1: parts.next().unwrap().parse().unwrap(),
            in2: parts.next().unwrap().parse().unwrap(),
            out: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Operation {
    fn execute(&self, regs: &mut [usize; 4]) {
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
            _ => unreachable!(),
        }
    }
}

struct Sample {
    before: [usize; 4],
    operation: Operation,
    after: [usize; 4],
}

impl From<&str> for Sample {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let before = lines.next().unwrap();
        let operation = lines.next().unwrap().into();
        let after = lines.next().unwrap();

        let before = before[9..before.len() - 1]
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let after = after[9..after.len() - 1]
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            before,
            operation,
            after,
        }
    }
}

impl Sample {
    fn similar_to(&self) -> Vec<bool> {
        let in1 = self.operation.in1;
        let in2 = self.operation.in2;
        let out = self.operation.out;

        let reg_in1 = self.reg_before(in1);
        let reg_in2 = self.reg_before(in2);
        // This must be always true
        let reg_out = self.reg_after(out);

        // Testing cases
        vec![
            // Addition
            reg_in1 + reg_in2 == reg_out,
            reg_in1 + in2 == reg_out,
            // Multiplication
            reg_in1 * reg_in2 == reg_out,
            reg_in1 * in2 == reg_out,
            // Bitwise AND
            reg_in1 & reg_in2 == reg_out,
            reg_in1 & in2 == reg_out,
            // Bitwise OR
            reg_in1 | reg_in2 == reg_out,
            reg_in1 | in2 == reg_out,
            // Assignment
            reg_in1 == reg_out,
            in1 == reg_out,
            // Greater-than
            (in1 > reg_in2) as usize == reg_out,
            (reg_in1 > in2) as usize == reg_out,
            (reg_in1 > reg_in2) as usize == reg_out,
            // Equality
            (in1 == reg_in2) as usize == reg_out,
            (reg_in1 == in2) as usize == reg_out,
            (reg_in1 == reg_in2) as usize == reg_out,
        ]
    }

    fn reg_before(&self, id: usize) -> usize {
        self.before[id]
    }

    fn reg_after(&self, id: usize) -> usize {
        self.after[id]
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (samples, program) = input.split_once("\n\n\n\n").unwrap();
    let samples: Vec<_> = samples.split("\n\n").map(Sample::from).collect();
    let program: Vec<_> = program.lines().map(Operation::from).collect();

    let solution1: Solution = samples
        .iter()
        .map(|sample| sample.similar_to().iter().filter(|&&case| case).count())
        .filter(|&count| count >= 3)
        .count();

    let solution2: Solution = {
        let mut samples = samples;
        let mut program = program;

        let mut mapped = HashSet::new();
        let mut mappings: [OperationType; 16] = [OperationType::Unknown(std::usize::MAX); 16];

        // While we not mapped all the instructions
        while mappings.contains(&OperationType::Unknown(std::usize::MAX)) {
            let (mut ty, mut opcode) = (OperationType::Unknown(0), 0);

            for sample in samples
                .iter()
                .filter(|sample| sample.operation.ty.is_unknown())
            {
                let cases = sample.similar_to();
                let mut unknown_cases = cases
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| !mapped.contains(&OperationType::from(i)))
                    .filter(|(_, &case)| case);
                if unknown_cases.clone().count() != 1 {
                    continue;
                }

                let (i, _) = unknown_cases.next().unwrap();
                ty = OperationType::from(i);
                opcode = sample.operation.ty.unknown_opcode().unwrap();
                break;
            }

            mappings[opcode] = ty;
            mapped.insert(ty);

            for sample in &mut samples {
                if let Some(v) = sample.operation.ty.unknown_opcode() && v == opcode {
                    sample.operation.ty = ty;
                }
            }
        }

        // Map new opcodes and execute them
        let mut regs = [0usize; 4];
        for operation in &mut program {
            operation.ty = mappings[operation.ty.unknown_opcode().unwrap()];
            operation.execute(&mut regs);
        }

        regs[0]
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::Sample;

    #[test]
    fn test_similar_to() {
        let input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let sample = Sample::from(input);
        assert_eq!(sample.similar_to().iter().filter(|&&case| case).count(), 3);
    }
}
