use aoc_2015::*;

const DAY: i32 = 23;
type Solution = usize;

enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" => Self::A,
            "b" => Self::B,
            _ => unreachable!(),
        }
    }
}

enum Offset {
    Pos(usize),
    Neg(usize),
}

impl From<&str> for Offset {
    fn from(value: &str) -> Self {
        let offset = value[1..].parse().unwrap();
        match value.chars().next().unwrap() {
            '+' => Self::Pos(offset),
            '-' => Self::Neg(offset),
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "hlf" => Self::Hlf(splits[1].into()),
            "tpl" => Self::Tpl(splits[1].into()),
            "inc" => Self::Inc(splits[1].into()),
            "jmp" => Self::Jmp(splits[1].into()),
            "jie" => Self::Jie(splits[1].into(), splits[3].into()),
            "jio" => Self::Jio(splits[1].into(), splits[3].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Cpu {
    // Instruction pointer
    ip: usize,
    // Registers
    a: usize,
    b: usize,
}

impl Cpu {
    fn run(&mut self, instructions: &[Instruction]) {
        // While we are in program
        while self.ip < instructions.len() {
            self.execute_instruction(&instructions[self.ip]);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Jmp(offset) => return self.jump(offset),
            Instruction::Jie(reg, offset) if self.reg(reg) % 2 == 0 => return self.jump(offset),
            Instruction::Jio(reg, offset) if self.reg(reg) == 1 => return self.jump(offset),
            _ => self.ip += 1,
        }

        match instruction {
            Instruction::Hlf(reg) => *self.reg_mut(reg) /= 2,
            Instruction::Tpl(reg) => *self.reg_mut(reg) *= 3,
            Instruction::Inc(reg) => *self.reg_mut(reg) += 1,
            _ => (),
        }
    }

    fn reg(&mut self, register: &Register) -> usize {
        match register {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn reg_mut(&mut self, register: &Register) -> &mut usize {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }

    fn jump(&mut self, offset: &Offset) {
        match *offset {
            Offset::Pos(v) => self.ip += v,
            Offset::Neg(v) => self.ip -= v,
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut cpu = Cpu::default();
        cpu.run(&instructions);
        cpu.b
    };

    let solution2: Solution = {
        let mut cpu = Cpu {
            a: 1,
            ..Default::default()
        };

        cpu.run(&instructions);
        cpu.b
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Cpu, Instruction};

    #[test]
    fn test_cpu() {
        let input = "inc a
jio a, +2
tpl a
inc a";
        let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

        let mut cpu = Cpu::default();
        cpu.run(&instructions);

        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.b, 0);
        assert_eq!(cpu.ip, 4);
    }
}
