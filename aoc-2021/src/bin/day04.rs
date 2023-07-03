use aoc_2021::*;

const DAY: i32 = 4;
type Solution = usize;

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Option<usize>>>,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let board = value
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|v| v.parse().ok())
                    .collect()
            })
            .collect();

        Self { board }
    }
}

impl Board {
    fn won(&self) -> bool {
        let size = self.board.len();

        (0..size).any(|i| {
            let (mut row, mut col) = (true, true);

            for j in 0..size {
                row &= self.board[i][j].is_none();
                col &= self.board[j][i].is_none();
            }

            row | col
        })
    }

    fn cross(&mut self, number: usize) -> bool {
        'main: for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                if cell == &Some(number) {
                    // Found cell, replace with None
                    *cell = None;
                    break 'main;
                }
            }
        }

        self.won()
    }
}

fn main() {
    let input = get_input_text(DAY);
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers: Vec<usize> = numbers.split(',').map(|v| v.parse().unwrap()).collect();
    let boards: Vec<_> = boards.trim().split("\n\n").map(Board::from).collect();

    let solution1: Solution = {
        let mut boards = boards.clone();

        let (mut win_number, mut uncrossed) = (0, 0);
        'main: for &number in &numbers {
            for board in &mut boards {
                if board.cross(number) {
                    win_number = number;
                    uncrossed = board.board.iter().flatten().filter_map(|&v| v).sum();
                    break 'main;
                }
            }
        }

        win_number * uncrossed
    };

    let solution2: Solution = {
        let mut boards = boards;

        let (mut win_number, mut uncrossed) = (0, 0);
        for &number in &numbers {
            for board in &mut boards {
                if board.cross(number) {
                    win_number = number;
                    uncrossed = board.board.iter().flatten().filter_map(|&v| v).sum();
                }
            }

            boards.retain(|board| !board.won());
        }

        win_number * uncrossed
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
