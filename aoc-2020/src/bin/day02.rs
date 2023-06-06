use aoc_2020::*;

const DAY: i32 = 2;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let passwords: Vec<(usize, usize, char, &str)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let (from, to) = parts.next().unwrap().split_once('-').unwrap();
            let ch = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();
            (from.parse().unwrap(), to.parse().unwrap(), ch, password)
        })
        .collect();

    let solution1: Solution = passwords
        .iter()
        .filter(|&&(from, to, ch, password)| {
            let count = password.chars().filter(|&c| c == ch).count();
            (from..=to).contains(&count)
        })
        .count();

    let solution2: Solution = passwords
        .iter()
        .filter(|&&(first, second, ch, password)| {
            let mut chars = password.chars();
            let (a, b) = (
                chars.nth(first - 1).unwrap(),
                chars.nth(second - first - 1).unwrap(),
            );

            (a == ch) ^ (b == ch)
        })
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
