use aoc_2015::*;

const DAY: i32 = 25;
type Solution = usize;

const FIRST: usize = 20151125;
const MULTIPLIER: usize = 252533;
const DIVISOR: usize = 33554393;
fn solve(find_row: usize, find_col: usize) -> Solution {
    // Starting from second row
    let mut row = 2;
    let mut value = FIRST;
    loop {
        for cur_col in 1..=row {
            value = (value * MULTIPLIER) % DIVISOR;
            let cur_row = row - cur_col + 1;
            if cur_row == find_row && cur_col == find_col {
                return value;
            }
        }

        row += 1;
    }
}

fn main() {
    let input = get_input_text(DAY);
    let mut splits = input.split([' ', ',', '.']);
    let (row, col) = (splits.nth(18).unwrap(), splits.nth(2).unwrap());
    let (row, col): (usize, usize) = (row.parse().unwrap(), col.parse().unwrap());

    let solution1: Solution = solve(row, col);
    show_solution(DAY, solution1);
}
