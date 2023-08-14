use std::collections::HashMap;

use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 21;
type Solution = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        }
    }
}

impl Operation {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Yell {
    Value(usize),
    Parent(Box<(Yell, Operation, Yell)>),
    Human(usize),
}

type Yells<'input> = HashMap<&'input str, &'input str>;

impl Yell {
    fn parse(id: &str, yells: &Yells) -> Self {
        let yell = yells[id];

        if let Ok(value) = yell.parse() {
            if id == "humn" {
                Self::Human(value)
            } else {
                Self::Value(value)
            }
        } else {
            let parts = yell.split_ascii_whitespace().collect_vec();

            let operation = parts[1].into();
            let op1 = Yell::parse(parts[0], yells);
            let op2 = Yell::parse(parts[2], yells);
            Self::Parent(Box::new((op1, operation, op2)))
        }
    }

    fn children(self) -> (Yell, Operation, Yell) {
        if let Yell::Parent(b) = self {
            *b
        } else {
            panic!("Can't get children of non-parent yell")
        }
    }

    fn value(&self) -> usize {
        match self {
            Yell::Value(n) | Yell::Human(n) => *n,
            Yell::Parent(b) => b.1.apply(b.0.value(), b.2.value()),
        }
    }

    fn make_immediates(&mut self) {
        if let Yell::Parent(b) = self {
            let (lhs, _, rhs) = b.as_mut();

            lhs.make_immediates();
            rhs.make_immediates();

            if matches!((lhs, rhs), (Yell::Value(_), Yell::Value(_))) {
                *self = Yell::Value(self.value());
            }
        }
    }

    fn undo(self, human: &mut usize) -> Yell {
        let (lhs, op, rhs) = self.children();

        match op {
            Operation::Add => {
                let (constant, variable) = if matches!(lhs, Yell::Value(_)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };

                *human -= constant.value();
                variable
            }

            Operation::Sub => match (lhs, rhs) {
                (Yell::Value(value), rhs) => {
                    *human = value - *human;
                    rhs
                }
                (lhs, Yell::Value(value)) => {
                    *human += value;
                    lhs
                }

                _ => unreachable!(),
            },

            Operation::Mul => {
                let (constant, variable) = if matches!(lhs, Yell::Value(_)) {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };

                *human /= constant.value();
                variable
            }

            Operation::Div => match (lhs, rhs) {
                (Yell::Value(value), rhs) => {
                    *human = value / *human;
                    rhs
                }
                (lhs, Yell::Value(value)) => {
                    *human *= value;
                    lhs
                }

                _ => unreachable!(),
            },
        }
    }

    fn find_humn(mut self) -> usize {
        self.make_immediates();

        let (lhs, _, rhs) = self.children();
        let (constant_side, mut human_side) = if matches!(lhs, Yell::Value(_)) {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };

        let mut human = constant_side.value();
        while !matches!(human_side, Yell::Human(_)) {
            human_side = human_side.undo(&mut human);
        }

        human
    }
}

fn main() {
    let input = get_input_text(DAY);

    let mut yells = Yells::new();
    for line in input.lines() {
        let (id, yell_expr) = line.split_once(": ").unwrap();
        yells.insert(id, yell_expr);
    }

    let root = Yell::parse("root", &yells);
    let solution1: Solution = root.value();
    let solution2: Solution = root.find_humn();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
