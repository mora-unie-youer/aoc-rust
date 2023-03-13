use aoc_2016::*;
use itertools::Itertools;

const DAY: i32 = 18;
type Solution = usize;

trait IsTrap {
    fn is_trap(&self) -> bool;
}

impl IsTrap for char {
    fn is_trap(&self) -> bool {
        *self == '^'
    }
}

impl IsTrap for (char, char, char) {
    fn is_trap(&self) -> bool {
        let (left, center, right) = (self.0.is_trap(), self.1.is_trap(), self.2.is_trap());

        [
            left && center && !right,
            !left && center && right,
            left && !center && !right,
            !left && !center && right,
        ]
        .iter()
        .any(|&rule| rule)
    }
}

trait CountSafe {
    fn count_safe(&self) -> usize;
}

impl CountSafe for String {
    fn count_safe(&self) -> usize {
        self.chars().filter(|&ch| ch == '.').count()
    }
}

trait NextRow {
    fn next_row(&self) -> Self;
}

impl NextRow for String {
    fn next_row(&self) -> Self {
        let extended_row = format!(".{}.", self);
        extended_row
            .chars()
            .tuple_windows()
            .map(|triad: (char, char, char)| if triad.is_trap() { '^' } else { '.' })
            .collect()
    }
}

fn solve(mut row: String, row_count: usize) -> Solution {
    let mut safe_count = row.count_safe();
    for _ in 0..row_count - 1 {
        row = row.next_row();
        safe_count += row.count_safe();
    }

    safe_count
}

fn main() {
    let input = get_input_text(DAY);
    let row = input.trim();

    let solution1: Solution = solve(row.to_string(), 40);
    let solution2: Solution = solve(row.to_string(), 400000);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
