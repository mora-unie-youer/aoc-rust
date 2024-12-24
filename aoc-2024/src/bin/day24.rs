use std::collections::HashMap;

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 24;
type Solution = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Gate<'input> {
    And(&'input str, &'input str),
    Or(&'input str, &'input str),
    Xor(&'input str, &'input str),
}

impl Gate<'_> {
    fn in1(&self) -> &str {
        match self {
            Self::And(in1, _) => in1,
            Self::Or(in1, _) => in1,
            Self::Xor(in1, _) => in1,
        }
    }

    fn in2(&self) -> &str {
        match self {
            Self::And(_, in2) => in2,
            Self::Or(_, in2) => in2,
            Self::Xor(_, in2) => in2,
        }
    }

    fn eval(&self, vars: &HashMap<&str, bool>) -> Option<bool> {
        let op1 = vars.get(self.in1());
        let op2 = vars.get(self.in2());
        if op1.is_none() || op2.is_none() {
            return None;
        }

        let v1 = op1.unwrap();
        let v2 = op2.unwrap();
        match self {
            Self::And(_, _) => Some(v1 & v2),
            Self::Or(_, _) => Some(v1 | v2),
            Self::Xor(_, _) => Some(v1 ^ v2),
        }
    }
}

impl<'input> From<&'input str> for Gate<'input> {
    fn from(value: &'input str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let in1 = parts.next().unwrap();
        let gate = parts.next().unwrap();
        let in2 = parts.next().unwrap();

        match gate {
            "AND" => Self::And(in1, in2),
            "OR" => Self::Or(in1, in2),
            "XOR" => Self::Xor(in1, in2),
            _ => unreachable!(),
        }
    }
}

fn simulate<'input>(vars: &mut HashMap<&'input str, bool>, mut wires: HashMap<&'input str, Gate>) {
    while !wires.is_empty() {
        let mut new_wires = wires.clone();

        for (output, gate) in &wires {
            if let Some(value) = gate.eval(vars) {
                vars.insert(output, value);
                new_wires.remove(output);
            }
        }

        wires = new_wires;
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (vars, wires) = input.split_once("\n\n").unwrap();

    let vars: HashMap<&str, bool> = vars
        .lines()
        .flat_map(|line| line.split_once(": "))
        .map(|(k, v)| (k, v.parse::<usize>().unwrap() != 0))
        .collect();

    let wires: HashMap<&str, Gate> = wires
        .lines()
        .flat_map(|line| line.split_once(" -> "))
        .map(|(gate, result)| (result, Gate::from(gate)))
        .collect();

    let solution1: Solution = {
        let mut vars = vars.clone();
        simulate(&mut vars, wires.clone());

        vars.into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .sorted()
            .rev()
            .fold(0usize, |acc, (_, v)| (acc << 1) | (v as usize))
            .to_string()
    };

    let solution2: Solution = {
        let mut outputs = HashMap::new();
        let mut xysum = HashMap::new();
        let mut xycarry = HashMap::new();
        let mut xyfullcarry = HashMap::new();
        let mut carry = HashMap::new();

        for (&signal, &gate) in &wires {
            let in1 = gate.in1();
            let in2 = gate.in2();

            if in1.starts_with(['x', 'y']) || in2.starts_with(['x', 'y']) {
                match gate {
                    Gate::And(_, _) => _ = xycarry.insert(signal, gate),
                    Gate::Xor(_, _) => _ = xysum.insert(signal, gate),
                    _ => (),
                }
            } else {
                match gate {
                    Gate::And(_, _) => _ = xyfullcarry.insert(signal, gate),
                    Gate::Xor(_, _) => _ = outputs.insert(signal, gate),
                    Gate::Or(_, _) => _ = carry.insert(signal, gate),
                }
            }
        }

        let mut incorrect = Vec::new();

        for &signal in outputs.keys() {
            if !signal.starts_with('z') {
                incorrect.push(signal);
            }
        }

        for (&signal, gate) in &xysum {
            // x00 ^ y00 is correct case
            let in1 = gate.in1();
            if in1 == "x00" || in1 == "y00" {
                continue;
            }

            let used_in_output = outputs
                .iter()
                .any(|(_, gate)| gate.in1() == signal || gate.in2() == signal);
            if signal.starts_with('z') || !used_in_output {
                incorrect.push(signal);
            }
        }

        for (&signal, gate) in &xycarry {
            // x00 & y00 is correct case
            let in1 = gate.in1();
            if in1 == "x00" || in1 == "y00" {
                continue;
            }

            let used_in_carry = carry
                .iter()
                .any(|(_, gate)| gate.in1() == signal || gate.in2() == signal);
            if signal.starts_with('z') || !used_in_carry {
                incorrect.push(signal);
            }
        }

        for &signal in carry.keys() {
            if signal.starts_with('z') && signal != "z45" {
                incorrect.push(signal);
            }
        }

        for &signal in xyfullcarry.keys() {
            if signal.starts_with('z') {
                incorrect.push(signal);
            }
        }

        incorrect.into_iter().sorted().join(",")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
