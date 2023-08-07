use aoc_2022::*;

const DAY: i32 = 14;
type Solution = usize;

fn drip(x_start: usize, y_start: usize, grid: &mut Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut x_fall = x_start;
    for y_fall in y_start..grid.len() {
        if grid[y_fall][x_fall] == '.' {
            continue;
        }

        // Hit something. Checking if we can move diagonally
        if grid[y_fall][x_fall - 1] == '.' {
            x_fall -= 1;
            continue;
        }

        if grid[y_fall][x_fall + 1] == '.' {
            x_fall += 1;
            continue;
        }

        // Sand has nowhere to go -> resting
        let y_level = y_fall - 1;
        grid[y_level][x_fall] = 'o';
        return Some((x_fall, y_level));
    }

    None
}

fn main() {
    let input = get_input_text(DAY);

    let (_x_min, x_max, _y_min, y_max) = {
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (
            std::usize::MAX,
            std::usize::MIN,
            std::usize::MAX,
            std::usize::MIN,
        );

        for line in input.lines() {
            let points = line
                .split(" -> ")
                .filter_map(|point| point.split_once(','))
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
            for (x, y) in points {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        (x_min, x_max, y_min, y_max)
    };

    let solution1: Solution = {
        let mut grid = vec![vec!['.'; x_max + 1]; y_max + 1];
        for line in input.lines() {
            let mut points = line
                .split(" -> ")
                .filter_map(|point| point.split_once(','))
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
            let (mut ox, mut oy): (usize, usize) = points.next().unwrap();

            for (x, y) in points {
                let (start_x, end_x) = (ox.min(x), ox.max(x));
                let (start_y, end_y) = (oy.min(y), oy.max(y));

                for row in grid.iter_mut().take(end_y + 1).skip(start_y) {
                    for cell in row.iter_mut().take(end_x + 1).skip(start_x) {
                        *cell = '#';
                    }
                }

                (ox, oy) = (x, y);
            }
        }

        loop {
            if drip(500, 0, &mut grid).is_none() {
                break;
            }
        }

        grid.iter().flatten().filter(|&&ch| ch == 'o').count()
    };

    let solution2: Solution = {
        let mut grid = vec![vec!['.'; x_max * 2]; y_max + 3];
        for line in input.lines() {
            let mut points = line
                .split(" -> ")
                .filter_map(|point| point.split_once(','))
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
            let (mut ox, mut oy): (usize, usize) = points.next().unwrap();

            for (x, y) in points {
                let (start_x, end_x) = (ox.min(x), ox.max(x));
                let (start_y, end_y) = (oy.min(y), oy.max(y));

                for row in grid.iter_mut().take(end_y + 1).skip(start_y) {
                    for cell in row.iter_mut().take(end_x + 1).skip(start_x) {
                        *cell = '#';
                    }
                }

                (ox, oy) = (x, y);
            }
        }

        // Filling bottom line (floor)
        for cell in grid[y_max + 2].iter_mut() {
            *cell = '#';
        }

        loop {
            if drip(500, 0, &mut grid) == Some((500, 0)) {
                break;
            }
        }

        grid.iter().flatten().filter(|&&ch| ch == 'o').count()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
