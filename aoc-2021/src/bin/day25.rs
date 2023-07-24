use aoc_2021::*;

const DAY: i32 = 25;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let mut grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let (height, width) = (grid.len(), grid[0].len());

        let mut step = 0;
        let mut moved = true;
        while moved {
            moved = false;

            for (dir, (x, y)) in [('>', (1, 0)), ('v', (0, 1))] {
                let mut next = grid.clone();
                for i in 0..height {
                    for j in 0..width {
                        let (next_row, next_col) = ((i + y) % height, (j + x) % width);
                        if grid[i][j] == dir && grid[next_row][next_col] == '.' {
                            next[next_row][next_col] = dir;
                            next[i][j] = '.';
                            moved = true;
                        }
                    }
                }
                grid = next;
            }

            step += 1;
        }

        step
    };

    show_solution(DAY, solution1);
}
