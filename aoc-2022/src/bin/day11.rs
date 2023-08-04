use std::cell::RefCell;

use aoc_2022::*;

const DAY: i32 = 11;
type Solution = usize;

#[derive(Clone, Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: RefCell<Vec<usize>>,
    operation: Operation,
    divisible: usize,
    targets: [usize; 2],
}

fn solve(monkeys: Vec<Monkey>, rounds: u32, worry_fn: impl Fn(Solution) -> Solution) -> Solution {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.items.borrow().iter() {
                inspections[i] += 1;
                let new_item = worry_fn(match monkey.operation {
                    Operation::Add(v) => item + v,
                    Operation::Mul(v) => item * v,
                    Operation::Square => item * item,
                });

                monkeys[monkey.targets[(new_item % monkey.divisible != 0) as usize]]
                    .items
                    .borrow_mut()
                    .push(new_item);
            }

            monkey.items.borrow_mut().clear();
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

fn main() {
    let input = get_input_text(DAY);

    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|def| {
            let mut lines = def.lines().skip(1);
            let items: Vec<_> = lines.next().unwrap()[18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            let (operator, operand) = lines.next().unwrap()[23..].split_once(' ').unwrap();
            let divisible = lines.next().unwrap()[21..].parse().unwrap();
            let true_target = lines.next().unwrap()[29..].parse().unwrap();
            let false_target = lines.next().unwrap()[30..].parse().unwrap();

            let operation = match (operator, operand) {
                ("+", _) => Operation::Add(operand.parse().unwrap()),
                ("*", "old") => Operation::Square,
                ("*", _) => Operation::Mul(operand.parse().unwrap()),
                _ => unreachable!(),
            };

            Monkey {
                items: RefCell::new(items),
                operation,
                divisible,
                targets: [true_target, false_target],
            }
        })
        .collect();
    let modulus: Solution = monkeys.iter().map(|monkey| monkey.divisible).product();

    let solution1: Solution = solve(monkeys.clone(), 20, |x| x / 3);
    let solution2: Solution = solve(monkeys, 10000, |x| x % modulus);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
