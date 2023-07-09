use aoc_2021::*;

const DAY: i32 = 10;
type Solution = usize;

fn open_to_close(ch: char) -> Option<char> {
    match ch {
        '(' => Some(')'),
        '[' => Some(']'),
        '<' => Some('>'),
        '{' => Some('}'),
        _ => None,
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|line| {
            let mut stack = vec![];
            let mut incorrect = 0;

            for ch in line.chars() {
                if let Some(close_ch) = open_to_close(ch) {
                    stack.push(close_ch);
                } else if stack.last() == Some(&ch) {
                    stack.pop();
                } else {
                    incorrect = match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                    break;
                }
            }

            incorrect
        })
        .sum();

    let solution2: Solution = {
        let mut scores: Vec<_> = input
            .lines()
            .filter_map(|line| {
                let mut stack = vec![];
                let mut interrupted = false;

                for ch in line.chars() {
                    if let Some(close_ch) = open_to_close(ch) {
                        stack.push(close_ch);
                    } else if stack.last() == Some(&ch) {
                        stack.pop();
                    } else {
                        interrupted = true;
                        break;
                    }
                }

                if interrupted {
                    None
                } else {
                    let complete = stack
                        .iter()
                        .rev()
                        .map(|ch| match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        })
                        .fold(0, |acc, v| acc * 5 + v);
                    Some(complete)
                }
            })
            .collect();
        scores.sort();
        scores[scores.len() / 2]
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
