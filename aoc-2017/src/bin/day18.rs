use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use aoc_2017::*;

const DAY: i32 = 18;
type Solution = isize;

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
    Snd(Operand<'input>),
    Rcv(Operand<'input>),
    Add(Operand<'input>, Operand<'input>),
    Jgz(Operand<'input>, Operand<'input>),
    Mul(Operand<'input>, Operand<'input>),
    Mod(Operand<'input>, Operand<'input>),
    Set(Operand<'input>, Operand<'input>),
}

impl<'input> From<&'input str> for Instruction<'input> {
    fn from(input: &'input str) -> Self {
        let splits: Vec<_> = input.split([' ', ',']).collect();
        match splits[0] {
            "snd" => Self::Snd(splits[1].into()),
            "rcv" => Self::Rcv(splits[1].into()),
            "add" => Self::Add(splits[1].into(), splits[2].into()),
            "jgz" => Self::Jgz(splits[1].into(), splits[2].into()),
            "mul" => Self::Mul(splits[1].into(), splits[2].into()),
            "mod" => Self::Mod(splits[1].into(), splits[2].into()),
            "set" => Self::Set(splits[1].into(), splits[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Cpu1<'input> {
    ip: usize,
    registers: HashMap<&'input str, isize>,
    sound: isize,
}

impl<'input> Cpu1<'input> {
    fn run(&mut self, instructions: &'input [Instruction]) -> isize {
        while self.ip < instructions.len() {
            if let Some(sound) = self.execute_instruction(&instructions[self.ip]) {
                return sound;
            }
        }

        unreachable!()
    }

    fn execute_instruction(&mut self, instruction: &'input Instruction) -> Option<isize> {
        match instruction {
            Instruction::Snd(op1) => self.sound = op1.value(&self.registers),
            Instruction::Rcv(op1) => {
                if op1.value(&self.registers) != 0 {
                    return Some(self.sound);
                }
            }
            Instruction::Set(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register = value;
            }
            Instruction::Add(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register += value;
            }
            Instruction::Mul(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register *= value;
            }
            Instruction::Mod(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register %= value;
            }
            Instruction::Jgz(op1, op2) => {
                let offset = op2.value(&self.registers);
                if op1.value(&self.registers) > 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                    return None;
                }
            }
        }

        self.ip += 1;
        None
    }
}

struct Cpu2<'input> {
    ip: usize,
    registers: HashMap<&'input str, isize>,

    tx: Sender<isize>,
    rx: Receiver<isize>,

    sent_count: usize,
    waiting: bool,
}

impl<'input> Cpu2<'input> {
    fn new(id: isize, tx: Sender<isize>, rx: Receiver<isize>) -> Self {
        let mut registers = HashMap::new();
        registers.insert("p", id);

        Self {
            ip: 0,
            registers,

            tx,
            rx,

            sent_count: 0,
            waiting: false,
        }
    }

    fn execute_once(&mut self, instructions: &'input [Instruction]) {
        let instruction = &instructions[self.ip];

        if self.ip >= instructions.len() {
            self.waiting = true;
            return;
        }

        match instruction {
            Instruction::Snd(op1) => self.send(op1.value(&self.registers)),
            Instruction::Rcv(op1) => {
                if let Some(v) = self.recv() {
                    let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                    *register = v;
                    self.waiting = false;
                } else {
                    self.waiting = true;
                    return;
                }
            }
            Instruction::Set(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register = value;
            }
            Instruction::Add(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register += value;
            }
            Instruction::Mul(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register *= value;
            }
            Instruction::Mod(op1, op2) => {
                let value = op2.value(&self.registers);
                let register = self.registers.entry(op1.register().unwrap()).or_insert(0);
                *register %= value;
            }
            Instruction::Jgz(op1, op2) => {
                let offset = op2.value(&self.registers);
                if op1.value(&self.registers) > 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                    return;
                }
            }
        }

        self.ip += 1;
    }

    fn send(&mut self, value: isize) {
        self.sent_count += 1;
        self.tx.send(value).unwrap();
    }

    fn recv(&mut self) -> Option<isize> {
        self.rx.try_recv().ok()
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut cpu = Cpu1::default();
        cpu.run(&instructions)
    };

    let solution2: Solution = {
        let (tx1, rx1) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();
        let mut cpu1 = Cpu2::new(0, tx2, rx1);
        let mut cpu2 = Cpu2::new(1, tx1, rx2);

        loop {
            cpu1.execute_once(&instructions);
            cpu2.execute_once(&instructions);
            if cpu1.waiting && cpu2.waiting {
                break;
            }
        }

        cpu2.sent_count as isize
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Cpu2};

    #[test]
    fn test_cpu2() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        let instructions: Vec<_> = input.lines().map(Instruction::from).collect();
        let (tx1, rx1) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();
        let mut cpu1 = Cpu2::new(0, tx2, rx1);
        let mut cpu2 = Cpu2::new(1, tx1, rx2);
        assert_eq!(cpu1.registers["p"], 0);
        assert_eq!(cpu2.registers["p"], 1);

        loop {
            cpu1.execute_once(&instructions);
            cpu2.execute_once(&instructions);
            if cpu1.waiting && cpu2.waiting {
                break;
            }
        }

        assert_eq!(cpu1.registers["a"], 1);
        assert_eq!(cpu2.registers["a"], 1);
        assert_eq!(cpu1.registers["b"], 2);
        assert_eq!(cpu2.registers["b"], 2);
        assert_eq!(cpu1.registers["c"], 1);
        assert_eq!(cpu2.registers["c"], 0);
    }
}
