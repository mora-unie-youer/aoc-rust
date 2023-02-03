use aoc_2015::*;

const DAY: i32 = 5;
type Solution = usize;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .filter(|s| s.chars().filter(|ch| VOWELS.contains(ch)).count() >= 3)
        .filter(|s| !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")))
        .filter(|s| {
            s.chars().collect::<Vec<_>>().windows(2).any(|pair| {
                let (first, second) = (pair[0], pair[1]);
                first == second
            })
        })
        .count();
    let solution2: Solution = input
        .lines()
        .filter(|s| {
            s.chars()
                .collect::<Vec<_>>()
                .windows(3)
                .any(|triplet| triplet[0] == triplet[2])
        })
        .filter(|s| {
            s.chars()
                .collect::<Vec<_>>()
                .windows(2)
                .enumerate()
                .any(|(i, pair)| s[i + 2..].contains(&pair.iter().collect::<String>()))
        })
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
