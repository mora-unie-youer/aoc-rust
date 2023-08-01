use aoc_2022::*;

const DAY: i32 = 8;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect();

    let solution1: Solution = grid.iter().enumerate().skip(1).take(grid.len() - 2).fold(
        2 * (grid.len() + grid[0].len()) - 4,
        |acc, (i, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(row.len() - 2)
                .fold(acc, |acc, (j, cell)| {
                    let left = row.iter().take(j).all(|v| v < cell);
                    let right = row.iter().skip(j + 1).all(|v| v < cell);
                    let up = grid.iter().take(i).map(|r| &r[j]).all(|v| v < cell);
                    let down = grid.iter().skip(i + 1).map(|r| &r[j]).all(|v| v < cell);
                    acc + (left || right || up || down) as usize
                })
        },
    );

    let solution2: Solution = grid
        .iter()
        .enumerate()
        .skip(1)
        .take(grid.len() - 2)
        .filter_map(|(i, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(row.len() - 2)
                .map(|(j, cell)| {
                    let left = row
                        .iter()
                        .take(j)
                        .rev()
                        .position(|v| v >= cell)
                        .map(|v| v + 1)
                        .unwrap_or(j);
                    let right = row
                        .iter()
                        .skip(j + 1)
                        .position(|v| v >= cell)
                        .map(|v| v + 1)
                        .unwrap_or(row.len() - j - 1);
                    let up = grid
                        .iter()
                        .take(i)
                        .rev()
                        .map(|r| &r[j])
                        .position(|v| v >= cell)
                        .map(|v| v + 1)
                        .unwrap_or(i);
                    let down = grid
                        .iter()
                        .skip(i + 1)
                        .map(|r| &r[j])
                        .position(|v| v >= cell)
                        .map(|v| v + 1)
                        .unwrap_or(grid.len() - i - 1);
                    left * right * up * down
                })
                .max()
        })
        .max()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
