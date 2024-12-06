use std::collections::HashSet;

use aoc_2024::*;

const DAY: i32 = 6;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = {
        let mut start = (0, 0);
        'main: for (y, row) in map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == '^' {
                    start = (x as isize, y as isize);
                    break 'main;
                }
            }
        }
        start
    };

    let solution1: Solution = {
        let mut visited = HashSet::new();
        let (mut cx, mut cy) = start;
        let (mut dx, mut dy) = (0, -1);

        loop {
            visited.insert((cx, cy));

            let mut nx = cx + dx;
            let mut ny = cy + dy;
            if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize {
                break visited.len();
            }

            loop {
                let next = map[ny as usize][nx as usize];
                if next == '#' {
                    (dx, dy) = (-dy, dx);
                    nx = cx + dx;
                    ny = cy + dy;
                } else {
                    break;
                }
            }

            cx = nx;
            cy = ny;
        }
    };

    let solution2: Solution = {
        const MAX_ITERATION: usize = 10000;
        let mut map = map;
        let mut count = 0;

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                let ch = map[y][x];
                if ch == '#' || ch == '^' {
                    continue;
                }

                map[y][x] = '#';
                let (mut cx, mut cy) = start;
                let (mut dx, mut dy) = (0, -1);

                let mut iteration = 0;
                while iteration < MAX_ITERATION {
                    iteration += 1;
                    let mut nx = cx + dx;
                    let mut ny = cy + dy;
                    if nx < 0 || ny < 0 || nx >= map[0].len() as isize || ny >= map.len() as isize {
                        break;
                    }

                    loop {
                        let next = map[ny as usize][nx as usize];
                        if next == '#' {
                            (dx, dy) = (-dy, dx);
                            nx = cx + dx;
                            ny = cy + dy;
                        } else {
                            break;
                        }
                    }

                    cx = nx;
                    cy = ny;
                }

                if iteration >= MAX_ITERATION {
                    count += 1;
                }

                map[y][x] = '.';
            }
        }

        count
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
