use aoc_2022::*;

const DAY: i32 = 3;
type Solution = usize;

fn char_to_priority(ch: char) -> Solution {
    match ch {
        'A'..='Z' => ch as usize - 38, // -65 + 27
        'a'..='z' => ch as usize - 96, // -97 + 1
        _ => unreachable!(),
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .map(|s| {
            let (first, second) = s.split_at(s.len() / 2);
            first
                .chars()
                .find(|&c| second.contains(c))
                .map(char_to_priority)
                .unwrap()
        })
        .sum();

    let solution2: Solution = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let [first, second, third] = &group[0..3] else { unreachable!() };
            first
                .chars()
                .find(|&c| second.contains(c) && third.contains(c))
                .map(char_to_priority)
                .unwrap()
        })
        .sum();

    show_solution(1, solution1);
    show_solution(2, solution2);
}
