use std::iter::Peekable;

use aoc_2020::*;

const DAY: i32 = 18;
type Solution = usize;

fn apply_operator(op1: Solution, op: char, op2: Solution) -> Solution {
    match op {
        '+' => op1 + op2,
        '*' => op1 * op2,
        _ => unreachable!(),
    }
}

fn eval_number<I>(iter: &mut Peekable<I>) -> Option<Solution>
where
    I: Iterator<Item = char>,
{
    let mut value = iter.next()?.to_digit(10)? as Solution;
    loop {
        match iter.peek() {
            None => break Some(value),
            Some(ch) => {
                if let Some(next_digit) = ch.to_digit(10) {
                    value = 10 * value + next_digit as Solution;
                    iter.next()?;
                } else {
                    break Some(value);
                }
            }
        }
    }
}

fn eval_expression_or_number<I>(iter: &mut Peekable<I>, part2: bool) -> Option<Solution>
where
    I: Iterator<Item = char>,
{
    match iter.peek()? {
        '(' => {
            iter.next()?;

            let expr = eval_expression(iter, part2);
            if iter.next() == Some(')') {
                expr
            } else {
                None
            }
        }
        _ => eval_number(iter),
    }
}

fn eval_expression<I>(iter: &mut Peekable<I>, part2: bool) -> Option<Solution>
where
    I: Iterator<Item = char>,
{
    let mut value = eval_expression_or_number(iter, part2)?;
    loop {
        if matches!(iter.peek(), None | Some(')')) {
            break Some(value);
        }

        let op = iter.next()?;
        let operand2 = match op {
            '*' if part2 => eval_expression(iter, part2),
            _ => eval_expression_or_number(iter, part2),
        }?;
        value = apply_operator(value, op, operand2);
    }
}

fn eval(value: &str, part2: bool) -> Solution {
    let mut chars = value.chars().filter(|&ch| ch != ' ').peekable();
    eval_expression(&mut chars, part2).unwrap()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input.lines().map(|line| eval(line, false)).sum();
    let solution2: Solution = input.lines().map(|line| eval(line, true)).sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
