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
    POS(usize),
    NEG(usize),
}

impl From<&str> for Offset {
    fn from(value: &str) -> Self {
        let offset = value[1..].parse().unwrap();
        match value.chars().next().unwrap() {
            '+' => Self::POS(offset),
            '-' => Self::NEG(offset),
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(Offset),
    JIE(Register, Offset),
    JIO(Register, Offset),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "hlf" => Self::HLF(splits[1].into()),
            "tpl" => Self::TPL(splits[1].into()),
            "inc" => Self::INC(splits[1].into()),
            "jmp" => Self::JMP(splits[1].into()),
            "jie" => Self::JIE(splits[1].into(), splits[3].into()),
            "jio" => Self::JIO(splits[1].into(), splits[3].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct CPU {
    // Instruction pointer
    ip: usize,
    // Registers
    a: usize,
    b: usize,
}

impl CPU {
    fn run(&mut self, instructions: &[Instruction]) {
        // While we are in program
        while self.ip < instructions.len() {
            self.execute_instruction(&instructions[self.ip]);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::JMP(offset) => return self.jump(offset),
            Instruction::JIE(reg, offset) if self.reg(reg) % 2 == 0 => return self.jump(offset),
            Instruction::JIO(reg, offset) if self.reg(reg) == 1 => return self.jump(offset),
            _ => self.ip += 1,
        }

        match instruction {
            Instruction::HLF(reg) => *self.reg_mut(reg) /= 2,
            Instruction::TPL(reg) => *self.reg_mut(reg) *= 3,
            Instruction::INC(reg) => *self.reg_mut(reg) += 1,
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
            Offset::POS(v) => self.ip += v,
            Offset::NEG(v) => self.ip -= v,
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(|line| line.into()).collect();

    let solution1: Solution = {
        let mut cpu = CPU::default();
        cpu.run(&instructions);
        cpu.b
    };

    let solution2: Solution = {
        let mut cpu = CPU::default();
        cpu.a = 1;
        cpu.run(&instructions);
        cpu.b
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::CPU;

    #[test]
    fn test_cpu() {
        let input = "inc a
jio a, +2
tpl a
inc a";
        let instructions: Vec<_> = input.lines().map(|line| line.into()).collect();

        let mut cpu = CPU::default();
        cpu.run(&instructions);

        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.b, 0);
        assert_eq!(cpu.ip, 4);
    }
}
