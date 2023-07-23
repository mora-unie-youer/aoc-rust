use std::collections::HashMap;

use aoc_2021::*;

const DAY: i32 = 24;
type Solution = String;

enum Operand {
    Register(usize),
    Value(isize),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Ok(value) = value.parse() {
            Self::Value(value)
        } else {
            let ch = value.chars().next().unwrap();
            Self::Register((ch as u8 - b'w') as usize)
        }
    }
}

impl Operand {
    fn register(&self) -> Option<usize> {
        match self {
            Self::Register(reg) => Some(*reg),
            _ => None,
        }
    }

    fn extract(&self, regs: &[isize; 4]) -> isize {
        match self {
            Self::Register(reg) => regs[*reg],
            Self::Value(value) => *value,
        }
    }
}

enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        match parts.next().unwrap() {
            "inp" => Self::Inp(parts.next().unwrap().into()),
            "add" => Self::Add(parts.next().unwrap().into(), parts.next().unwrap().into()),
            "mul" => Self::Mul(parts.next().unwrap().into(), parts.next().unwrap().into()),
            "div" => Self::Div(parts.next().unwrap().into(), parts.next().unwrap().into()),
            "mod" => Self::Mod(parts.next().unwrap().into(), parts.next().unwrap().into()),
            "eql" => Self::Eql(parts.next().unwrap().into(), parts.next().unwrap().into()),
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    fn output_reg(&self) -> usize {
        match self {
            Self::Inp(op) => op.register().unwrap(),
            Self::Add(op1, _) => op1.register().unwrap(),
            Self::Mul(op1, _) => op1.register().unwrap(),
            Self::Div(op1, _) => op1.register().unwrap(),
            Self::Mod(op1, _) => op1.register().unwrap(),
            Self::Eql(op1, _) => op1.register().unwrap(),
        }
    }
}

fn execute(registers: &mut [isize; 4], instruction: &Instruction, input: Option<isize>) {
    let output_reg = instruction.output_reg();

    let mut output = registers[output_reg];
    match instruction {
        Instruction::Inp(_) => output = input.unwrap(),
        Instruction::Add(_, op) => output += op.extract(registers),
        Instruction::Mul(_, op) => output *= op.extract(registers),
        Instruction::Div(_, op) => output /= op.extract(registers),
        Instruction::Mod(_, op) => output %= op.extract(registers),
        Instruction::Eql(_, op) => output = (output == op.extract(registers)) as isize,
    }

    registers[output_reg] = output;
}

type Registers = [isize; 4];
type State = (usize, Registers);
fn solve(
    instructions: &[Instruction],
    state: State,
    visited: &mut HashMap<State, Option<isize>>,
    part2: bool,
) -> Option<isize> {
    // Must start with input instruction, as we're doing recursion here
    assert!(matches!(instructions[state.0], Instruction::Inp(_)));

    if visited.contains_key(&state) {
        return visited[&state];
    }

    const BIGGEST: [isize; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    const SMALLEST: [isize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let digits = if part2 { SMALLEST } else { BIGGEST };
    'digit: for digit in digits {
        let (mut ip, mut registers) = state;
        execute(&mut registers, &instructions[ip], Some(digit));
        ip += 1;

        while let Some(instruction) = instructions.get(ip) {
            if matches!(instruction, Instruction::Inp(_)) {
                if let Some(max) = solve(instructions, (ip, registers), visited, part2) {
                    let result = Some(max * 10 + digit);
                    visited.insert((ip, registers), result);
                    return result;
                } else {
                    continue 'digit;
                }
            } else {
                execute(&mut registers, instruction, None);
                ip += 1;
            }
        }

        const Z: usize = 3;
        if registers[Z] == 0 {
            let result = Some(digit);
            visited.insert((ip, registers), result);
            return result;
        }
    }

    visited.insert(state, None);
    None
}

fn main() {
    let input = get_input_text(DAY);

    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();
    let initial_state = (0, [0; 4]);

    let solution1: Solution = solve(&instructions, initial_state, &mut HashMap::new(), false)
        .unwrap()
        .to_string()
        .chars()
        .rev()
        .collect();

    let solution2: Solution = solve(&instructions, initial_state, &mut HashMap::new(), true)
        .unwrap()
        .to_string()
        .chars()
        .rev()
        .collect();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
