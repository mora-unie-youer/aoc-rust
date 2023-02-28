use std::collections::HashMap;

use aoc_2015::*;

const DAY: i32 = 7;
type Solution = u16;
type Wires<'name> = HashMap<&'name str, Solution>;

#[derive(Clone, Copy)]
enum Gate {
    Not,

    Lshift,
    Rshift,

    And,
    Or,

    Assign,
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "NOT" => Self::Not,
            "LSHIFT" => Self::Lshift,
            "RSHIFT" => Self::Rshift,
            "AND" => Self::And,
            "OR" => Self::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Wire<'name> {
    Set(Solution),
    Unset(&'name str),
}

impl<'name> From<&'name str> for Wire<'name> {
    fn from(wire: &'name str) -> Self {
        match wire.parse::<Solution>() {
            Ok(v) => Self::Set(v),
            Err(_) => Self::Unset(wire),
        }
    }
}

trait ApplyWire {
    fn apply(self, wires: &Wires) -> Self;
}

impl ApplyWire for Wire<'_> {
    fn apply(self, wires: &Wires) -> Self {
        match self {
            Self::Unset(name) if wires.contains_key(name) => Self::Set(wires[name]),
            _ => self,
        }
    }
}

impl ApplyWire for Option<Wire<'_>> {
    fn apply(self, wires: &Wires) -> Self {
        self.map(|wire| wire.apply(wires))
    }
}

trait GetWire {
    fn get_value(self) -> Solution;
}

impl GetWire for Wire<'_> {
    fn get_value(self) -> Solution {
        match self {
            Self::Set(v) => v,
            _ => panic!("Tried to get wire value when unset"),
        }
    }
}

impl GetWire for Option<Wire<'_>> {
    fn get_value(self) -> Solution {
        match self {
            Some(wire) => wire.get_value(),
            _ => panic!("Tried to get wire value when unused"),
        }
    }
}

trait IsSetWire {
    fn is_set(&self) -> bool;
}

impl IsSetWire for Wire<'_> {
    fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }
}

impl IsSetWire for Option<Wire<'_>> {
    fn is_set(&self) -> bool {
        match self {
            Some(wire) => wire.is_set(),
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction<'wire> {
    gate: Gate,
    op1: Wire<'wire>,
    op2: Option<Wire<'wire>>,
    out: Wire<'wire>,
}

impl Instruction<'_> {
    fn result(self, wires: &Wires) -> Self {
        if !self.op1.is_set() || !self.op2.is_set() {
            Instruction {
                gate: self.gate,
                op1: self.op1.apply(wires),
                op2: self.op2.apply(wires),
                out: self.out,
            }
        } else {
            let value = match self.gate {
                Gate::Not => !self.op1.get_value(),
                Gate::Lshift => self.op1.get_value() << self.op2.get_value(),
                Gate::Rshift => self.op1.get_value() >> self.op2.get_value(),
                Gate::And => self.op1.get_value() & self.op2.get_value(),
                Gate::Or => self.op1.get_value() | self.op2.get_value(),
                _ => unreachable!(),
            };

            Instruction {
                gate: Gate::Assign,
                op1: Wire::Set(value),
                op2: None,
                out: self.out,
            }
        }
    }
}

impl<'wire> From<&'wire str> for Instruction<'wire> {
    fn from(value: &'wire str) -> Self {
        let parts: Vec<_> = value.split_ascii_whitespace().collect();
        match parts.len() {
            // Assignment
            3 => Instruction {
                gate: Gate::Assign,
                op1: parts[0].into(),
                op2: None,
                out: parts[2].into(),
            },
            // Unary
            4 => Instruction {
                gate: parts[0].into(),
                op1: parts[1].into(),
                op2: None,
                out: parts[3].into(),
            },
            // Binary
            5 => Instruction {
                gate: parts[1].into(),
                op1: parts[0].into(),
                op2: Some(parts[2].into()),
                out: parts[4].into(),
            },
            _ => unreachable!(),
        }
    }
}

fn solve<'input>(mut instructions: Vec<Instruction<'input>>, wires: &mut Wires<'input>) {
    while !instructions.is_empty() {
        instructions.retain(|instr| match instr.gate {
            Gate::Assign => match instr.op1 {
                Wire::Set(v) => {
                    let wire_name = match instr.out {
                        Wire::Unset(name) => name,
                        _ => panic!("Tried to get name of set wire"),
                    };

                    if !wires.contains_key(wire_name) {
                        wires.insert(wire_name, v);
                    }

                    false
                }
                _ => true,
            },
            _ => true,
        });

        instructions = instructions
            .iter()
            .map(|instr| instr.result(wires))
            .collect();
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut wires: Wires = HashMap::new();
        solve(instructions.clone(), &mut wires);
        *wires.get("a").expect("Value of `a` was not recieved")
    };

    let solution2: Solution = {
        let mut wires: Wires = HashMap::new();
        wires.insert("b", solution1);
        solve(instructions.clone(), &mut wires);
        *wires.get("a").expect("Value of `a` was not recieved")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
