#![feature(array_windows)]

use aoc_2016::*;

const DAY: i32 = 18;
type Solution = usize;

trait IsTrap {
    fn is_trap(&self) -> bool;
}

impl IsTrap for [bool; 3] {
    fn is_trap(&self) -> bool {
        // 1)  left &&  center && !right,
        // 3)  left && !center && !right,
        // 2) !left &&  center &&  right,
        // 4) !left && !center &&  right,
        // // Center doesn't play any role, so simplify
        self[0] != self[2]
    }
}

trait CountSafe {
    fn count_safe(&self) -> usize;
}

impl CountSafe for Vec<bool> {
    fn count_safe(&self) -> usize {
        self.iter().filter(|&&ch| !ch).count()
    }
}

trait NextRow {
    fn next_row(&self) -> Self;
}

impl NextRow for Vec<bool> {
    fn next_row(&self) -> Self {
        let mut extended_row = vec![false];
        extended_row.extend(self);
        extended_row.push(false);

        extended_row
            .array_windows::<3>()
            .map(|triad| triad.is_trap())
            .collect()
    }
}

fn solve(mut row: Vec<bool>, row_count: usize) -> Solution {
    let mut safe_count = row.count_safe();
    for _ in 0..row_count - 1 {
        row = row.next_row();
        safe_count += row.count_safe();
    }

    safe_count
}

fn main() {
    let input = get_input_text(DAY);
    let row: Vec<_> = input.trim().chars().map(|ch| ch == '^').collect();

    let solution1: Solution = solve(row.clone(), 40);
    let solution2: Solution = solve(row, 400000);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
