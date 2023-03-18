use aoc_2016::*;

// Took base code from day 12
const DAY: i32 = 23;
type Solution = isize;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Instruction<'input> {
    Inc(Operand<'input>),
    Dec(Operand<'input>),
    Cpy(Operand<'input>, Operand<'input>),
    Jnz(Operand<'input>, Operand<'input>),
    Tgl(Operand<'input>),
}

impl<'input> From<&'input str> for Instruction<'input> {
    fn from(input: &'input str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "inc" => Self::Inc(splits[1].into()),
            "dec" => Self::Dec(splits[1].into()),
            "cpy" => Self::Cpy(splits[1].into(), splits[2].into()),
            "jnz" => Self::Jnz(splits[1].into(), splits[2].into()),
            "tgl" => Self::Tgl(splits[1].into()),
            _ => unreachable!(),
        }
    }
}

impl Instruction<'_> {
    fn toggle(&mut self) {
        let new_instruction = match *self {
            Self::Inc(op1) => Self::Dec(op1),
            Self::Dec(op1) | Self::Tgl(op1) => Self::Inc(op1),
            Self::Cpy(op1, op2) => Self::Jnz(op1, op2),
            Self::Jnz(op1, op2) => Self::Cpy(op1, op2),
        };

        *self = new_instruction;
    }
}

struct Cpu<'input> {
    // Instruction pointer
    instructions: Vec<Instruction<'input>>,
    ip: usize,
    // Registers (a, b, c, d)
    registers: [isize; 4],
}

impl<'input> Cpu<'input> {
    fn new(instructions: Vec<Instruction<'input>>) -> Self {
        Self {
            instructions,
            ip: 0,
            registers: [0, 0, 0, 0],
        }
    }
}

impl Cpu<'_> {
    fn run(&mut self) {
        // While we are in program
        while self.ip < self.instructions.len() {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        let current_instruction = self.instructions[self.ip];
        match current_instruction {
            // These instructions must work with registers
            Instruction::Dec(op1) => *self.reg_mut(op1.register().unwrap()) -= 1,
            Instruction::Inc(op1) => *self.reg_mut(op1.register().unwrap()) += 1,
            Instruction::Cpy(op1, op2) => {
                let value = match op1 {
                    Operand::Integer(v) => v,
                    Operand::Register(v) => self.reg(v),
                };
                // Second operand must be a register
                *self.reg_mut(op2.register().unwrap()) = value;
            }
            Instruction::Jnz(op1, op2) => {
                let cmp = match op1 {
                    Operand::Integer(v) => v,
                    Operand::Register(v) => self.reg(v),
                };

                let offset = match op2 {
                    Operand::Integer(v) => v,
                    Operand::Register(v) => self.reg(v),
                };

                if cmp != 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                    return;
                }
            }
            Instruction::Tgl(op1) => {
                let offset = match op1 {
                    Operand::Integer(v) => v,
                    Operand::Register(v) => self.reg(v),
                };

                let ip = (self.ip as isize + offset) as usize;
                if ip < self.instructions.len() {
                    self.instructions[ip].toggle();
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
        let mut cpu = Cpu::new(instructions.clone());
        cpu.registers[0] = 7;
        cpu.run();
        cpu.registers[0]
    };

    let solution2: Solution = {
        let mut cpu = Cpu::new(instructions.clone());
        cpu.registers[0] = 12;
        cpu.run();
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
        let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
        let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

        let mut cpu = Cpu::new(instructions);
        cpu.run();

        assert_eq!(cpu.ip, 7);
        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[1], 0);
        assert_eq!(cpu.registers[2], 0);
        assert_eq!(cpu.registers[3], 0);
    }
}
