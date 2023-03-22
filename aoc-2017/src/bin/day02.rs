use aoc_2017::*;

const DAY: i32 = 2;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let mut line: Vec<_> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            line.sort();
            line
        })
        .collect();

    let solution1: Solution = input
        .iter()
        .map(|line| {
            let min = line.first().unwrap();
            let max = line.last().unwrap();
            max - min
        })
        .sum();

    let solution2: Solution = input
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .find_map(|(i, a)| line[i + 1..].iter().find(|&b| b % a == 0).map(|b| b / a))
                .unwrap()
        })
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
