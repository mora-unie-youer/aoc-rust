use aoc_2017::*;

const DAY: i32 = 3;
type Solution = usize;

fn find_coordinates(input: usize) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;

    for i in 1.. {
        if i == input {
            break;
        }

        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }

    (x, y)
}

fn find_larger(input: usize, input_x: isize, input_y: isize) -> Solution {
    // Grid should be smaller than this :P
    let center = input_x.max(input_y).abs();
    let mut grid = vec![vec![0; (2 * center + 1) as usize]; (2 * center + 1) as usize];
    let (mut x, mut y) : (isize, isize) = (1, 0);
    let (mut dx, mut dy) = (1, 0);
    grid[center as usize][center as usize] = 1;

    loop {
        let mut sum = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let ny = y + center + i;
                let nx = x + center + j;
                sum += grid[ny as usize][nx as usize];
            }
        }

        if sum > input {
            return sum;
        }

        grid[(y + center) as usize][(x + center) as usize] = sum;

        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }
}

fn main() {
    let input = get_input_text(DAY).trim().parse().unwrap();
    let (x, y) = find_coordinates(input);

    let solution1: Solution = (x.abs() + y.abs()) as usize;
    let solution2: Solution = find_larger(input, x, y);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
