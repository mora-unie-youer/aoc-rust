use aoc_2017::*;

const DAY: i32 = 1;
type Solution = usize;

fn solve(input: &str, halfway: bool) -> Solution {
    let input = input.trim();
    let offset = if halfway { input.len() / 2 } else { 1 };
    let digits: Vec<_> = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect();

    digits
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, digit)| (digit, digits[(i + offset) % digits.len()]))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(input.trim(), false);
    let solution2: Solution = solve(input.trim(), true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
