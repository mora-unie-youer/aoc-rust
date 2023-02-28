#![feature(slice_flatten)]

use aoc_2015::*;

const DAY: i32 = 18;
type Solution = usize;

struct Grid {
    corners_on: bool,
    read: Box<[[bool; 100]; 100]>,
    write: Box<[[bool; 100]; 100]>,
}

impl Grid {
    fn new(input: &str, corners_on: bool) -> Self {
        let mut read = Box::new([[false; 100]; 100]);
        let mut write = Box::new([[false; 100]; 100]);

        input.lines().enumerate().for_each(|(i, line)| {
            line.chars()
                .enumerate()
                .for_each(|(j, ch)| read[i][j] = ch == '#')
        });

        if corners_on {
            write[0][0] = true;
            write[0][99] = true;
            write[99][0] = true;
            write[99][99] = true;
        }

        Self {
            corners_on,
            read,
            write,
        }
    }

    fn update_cell(&mut self, row: usize, column: usize) {
        if self.corners_on && (row == 0 || row == 99) && (column == 0 || column == 99) {
            return;
        }

        let x1 = if column > 1 { column - 1 } else { 0 };
        let x2 = if column < 99 { column + 1 } else { 99 };
        let y1 = if row > 1 { row - 1 } else { 0 };
        let y2 = if row < 99 { row + 1 } else { 99 };

        let mut neighbors = 0;
        let mut state = false;
        for y in y1..=y2 {
            for x in x1..=x2 {
                let cell = self.read[y][x];
                if y != row || x != column {
                    neighbors += cell as usize;
                } else {
                    state = cell;
                }
            }
        }

        self.write[row][column] = match state {
            true if neighbors != 2 && neighbors != 3 => false,
            false if neighbors == 3 => true,
            v => v,
        };
    }

    fn update(&mut self) {
        for y in 0..100 {
            for x in 0..100 {
                self.update_cell(y, x);
            }
        }

        std::mem::swap(&mut self.read, &mut self.write);
    }
}

fn solve(input: &str, corners_on: bool) -> Solution {
    let mut grid = Grid::new(input, corners_on);
    (0..100).for_each(|_| grid.update());
    grid.read.flatten().iter().filter(|&&on| on).count()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(&input, false);
    let solution2: Solution = solve(&input, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
