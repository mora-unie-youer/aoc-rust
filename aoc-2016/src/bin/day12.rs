use aoc_2016::*;

const DAY: i32 = 12;
type Solution = isize;

#[derive(Debug)]
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
}

enum Instruction<'input> {
    Inc(Operand<'input>),
    Dec(Operand<'input>),
    Cpy(Operand<'input>, Operand<'input>),
    Jnz(Operand<'input>, Operand<'input>),
}

impl<'input> From<&'input str> for Instruction<'input> {
    fn from(input: &'input str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "inc" => Self::Inc(splits[1].into()),
            "dec" => Self::Dec(splits[1].into()),
            "cpy" => Self::Cpy(splits[1].into(), splits[2].into()),
            "jnz" => Self::Jnz(splits[1].into(), splits[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Cpu {
    // Instruction pointer
    ip: usize,
    // Registers (a, b, c, d)
    registers: [isize; 4],
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
            // These instructions must work with registers
            Instruction::Dec(op1) => *self.reg_mut(op1.register().unwrap()) -= 1,
            Instruction::Inc(op1) => *self.reg_mut(op1.register().unwrap()) += 1,
            Instruction::Cpy(op1, op2) => {
                let value = match op1 {
                    Operand::Integer(v) => *v,
                    Operand::Register(v) => self.reg(v),
                };
                // Second operand must be a register
                *self.reg_mut(op2.register().unwrap()) = value;
            }
            Instruction::Jnz(op1, op2) => {
                let cmp = match op1 {
                    Operand::Integer(v) => *v,
                    Operand::Register(v) => self.reg(v),
                };

                let offset = match op2 {
                    Operand::Integer(v) => *v,
                    Operand::Register(v) => self.reg(v),
                };

                if cmp != 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                    return;
                }
            }
        }

        // If it wasn't a jump - increasing IP
        self.ip += 1;
    }

    fn reg_id(&self, register: &str) -> usize {
        match register {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => unreachable!(),
        }
    }

    fn reg(&self, register: &str) -> isize {
        let i = self.reg_id(register);
        self.registers[i]
    }

    fn reg_mut(&mut self, register: &str) -> &mut isize {
        let i = self.reg_id(register);
        &mut self.registers[i]
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut cpu = Cpu::default();
        cpu.run(&instructions);
        cpu.registers[0]
    };

    let solution2: Solution = {
        let mut cpu = Cpu::default();
        cpu.registers[2] = 1;
        cpu.run(&instructions);
        cpu.registers[0]
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Cpu, Instruction};

    #[test]
    fn test_cpu() {
        let input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
        let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

        let mut cpu = Cpu::default();
        cpu.run(&instructions);

        assert_eq!(cpu.ip, 6);
        assert_eq!(cpu.registers[0], 42);
        assert_eq!(cpu.registers[1], 0);
        assert_eq!(cpu.registers[2], 0);
        assert_eq!(cpu.registers[3], 0);
    }
}
