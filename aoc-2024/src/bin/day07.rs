use aoc_2024::*;

const DAY: i32 = 7;
type Solution = usize;

fn solve(result: usize, operands: &[usize], part2: bool) -> bool {
    fn solve_inner(
        result: usize,
        sum: usize,
        solved: &mut bool,
        operands: &[usize],
        operations: &[char],
    ) {
        if sum > result {
            return;
        } else if operands.is_empty() {
            *solved = sum == result;
            return;
        }

        for op in operations {
            let new_sum = match op {
                '+' => sum + operands[0],
                '*' => sum * operands[0],
                '|' => {
                    let op2 = operands[0];
                    let shift = match op2 {
                        // Numbers are lower than 10k
                        (1000..) => 10000,
                        (100..) => 1000,
                        (10..) => 100,
                        _ => 10,
                    };
                    sum * shift + op2
                }

                _ => unreachable!(),
            };

            solve_inner(result, new_sum, solved, &operands[1..], operations);
            if *solved {
                return;
            }
        }
    }

    const OPS1: &[char] = &['+', '*'];
    const OPS2: &[char] = &['+', '*', '|'];
    let mut solved = false;
    let operations = if part2 { OPS2 } else { OPS1 };
    solve_inner(result, operands[0], &mut solved, &operands[1..], operations);
    solved
}

fn main() {
    let input = get_input_text(DAY);

    let equations: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let (result, operands) = line.split_once(": ").unwrap();
            let result = result.parse().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (result, operands)
        })
        .collect();

    let solution1: Solution = equations
        .iter()
        .filter(|(result, operands)| solve(*result, operands, false))
        .map(|(result, _)| result)
        .sum();

    let solution2: Solution = equations
        .iter()
        .filter(|(result, operands)| solve(*result, operands, true))
        .map(|(result, _)| result)
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
