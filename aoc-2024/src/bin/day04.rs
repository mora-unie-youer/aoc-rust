use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 4;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    dbg!(&input);

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let solution1: Solution = {
        let mut count = 0;

        let (width, height) = (grid[0].len(), grid.len());
        for base_y in 0..height {
            for base_x in 0..width {
                let mut strings = Vec::with_capacity(4);

                if base_y < height - 3 {
                    let vertical: String = grid
                        .iter()
                        .skip(base_y)
                        .take(4)
                        .map(|row| row[base_x])
                        .collect();
                    strings.push(vertical);
                }

                if base_x < width - 3 {
                    let horizontal: String = grid[base_y][base_x..=base_x + 3].iter().collect();
                    strings.push(horizontal);
                }

                if base_x >= 3 && base_y < height - 3 {
                    let diagonal_left: String = grid
                        .iter()
                        .skip(base_y)
                        .take(4)
                        .enumerate()
                        .map(|(i, row)| row[base_x - i])
                        .collect();
                    strings.push(diagonal_left);
                }

                if base_x < width - 3 && base_y < height - 3 {
                    let diagonal_right: String = grid
                        .iter()
                        .skip(base_y)
                        .take(4)
                        .enumerate()
                        .map(|(i, row)| row[base_x + i])
                        .collect();
                    strings.push(diagonal_right);
                }

                count += strings
                    .into_iter()
                    .filter(|s| s == "XMAS" || s == "SAMX")
                    .count();
            }
        }

        count
    };

    let solution2: Solution = {
        let mut count = 0;

        let (width, height) = (grid[0].len(), grid.len());
        for base_y in 0..height - 2 {
            for base_x in 0..width - 2 {
                const POS: [(usize, usize); 5] = [(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)];
                let found_x: Vec<char> = POS
                    .into_iter()
                    .map(|(x, y)| grid[base_y + y][base_x + x])
                    .collect();

                let diagonal_right = format!("{}{}{}", found_x[0], found_x[2], found_x[4]);
                let diagonal_left = format!("{}{}{}", found_x[1], found_x[2], found_x[3]);
                let strings = [diagonal_left, diagonal_right];
                count += strings.into_iter().all(|s| s == "MAS" || s == "SAM") as usize;
            }
        }

        count
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
