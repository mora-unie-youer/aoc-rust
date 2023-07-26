use aoc_2022::*;

const DAY: i32 = 2;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    #[allow(clippy::identity_op)]
    let solutions = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(a, b)| (a.chars().next().unwrap(), b.chars().next().unwrap()))
        .fold((0, 0), |(s1, s2), (a, b)| match (a, b) {
            ('A', 'X') => (3 + 1 + s1, 0 + 3 + s2),
            ('A', 'Y') => (6 + 2 + s1, 3 + 1 + s2),
            ('A', 'Z') => (0 + 3 + s1, 6 + 2 + s2),
            ('B', 'X') => (0 + 1 + s1, 0 + 1 + s2),
            ('B', 'Y') => (3 + 2 + s1, 3 + 2 + s2),
            ('B', 'Z') => (6 + 3 + s1, 6 + 3 + s2),
            ('C', 'X') => (6 + 1 + s1, 0 + 2 + s2),
            ('C', 'Y') => (0 + 2 + s1, 3 + 3 + s2),
            ('C', 'Z') => (3 + 3 + s1, 6 + 1 + s2),
            _ => unreachable!(),
        });

    let solution1: Solution = solutions.0;
    let solution2: Solution = solutions.1;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
