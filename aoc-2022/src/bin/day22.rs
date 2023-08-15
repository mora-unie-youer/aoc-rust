use aoc_2022::*;

const DAY: i32 = 22;
type Solution = usize;

const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);

fn wrap(x: isize, y: isize, dx: isize, dy: isize, grid: &[Vec<u8>]) -> (isize, isize) {
    let (mut nx, mut ny) = (x + dx, y + dy);

    let wrap_y_up =
        |x: usize, grid: &[Vec<u8>]| grid.iter().position(|row| row[x] != b' ').unwrap();
    let wrap_y_down =
        |x: usize, grid: &[Vec<u8>]| grid.iter().rposition(|row| row[x] != b' ').unwrap();
    let wrap_x_left =
        |y: usize, grid: &[Vec<u8>]| grid[y].iter().position(|&ch| ch != b' ').unwrap();
    let wrap_x_right =
        |y: usize, grid: &[Vec<u8>]| grid[y].iter().rposition(|&ch| ch != b' ').unwrap();

    if ny < 0 {
        (nx, wrap_y_down(nx as usize, grid) as isize)
    } else if ny as usize >= grid.len() {
        (nx, wrap_y_up(nx as usize, grid) as isize)
    } else if nx < 0 {
        (wrap_x_right(ny as usize, grid) as isize, ny)
    } else if nx as usize >= grid[ny as usize].len() {
        (wrap_x_left(ny as usize, grid) as isize, ny)
    } else if grid[ny as usize][nx as usize] == b' ' {
        match (dx, dy) {
            RIGHT => nx = wrap_x_left(ny as usize, grid) as isize,
            DOWN => ny = wrap_y_up(nx as usize, grid) as isize,
            LEFT => nx = wrap_x_right(ny as usize, grid) as isize,
            UP => ny = wrap_y_down(nx as usize, grid) as isize,

            _ => unreachable!(),
        }

        (nx, ny)
    } else {
        (nx, ny)
    }
}

fn wrap_on_cube(x: isize, y: isize, dx: isize, dy: isize) -> ((isize, isize), (isize, isize)) {
    match (dx, dy) {
        RIGHT => match (x, y) {
            (149, 0..=49) => ((99, 149 - y), LEFT),
            (99, 50..=99) => ((50 + y, 49), UP),
            (99, 100..=149) => ((149, 149 - y), LEFT),
            (49, 150..=199) => ((y - 100, 149), UP),

            _ => ((x + dx, y + dy), (dx, dy)),
        },
        DOWN => match (x, y) {
            (0..=49, 199) => ((x + 100, 0), DOWN),
            (50..=99, 149) => ((49, 100 + x), LEFT),
            (100..=149, 49) => ((99, x - 50), LEFT),
            _ => ((x + dx, y + dy), (dx, dy)),
        },
        LEFT => match (x, y) {
            (50, 0..=49) => ((0, 149 - y), RIGHT),
            (50, 50..=99) => ((y - 50, 100), DOWN),
            (0, 100..=149) => ((50, 149 - y), RIGHT),
            (0, 150..=199) => ((y - 100, 0), DOWN),
            _ => ((x + dx, y + dy), (dx, dy)),
        },
        UP => match (x, y) {
            (0..=49, 100) => ((50, 50 + x), RIGHT),
            (50..=99, 0) => ((0, 100 + x), RIGHT),
            (100..=149, 0) => ((x - 100, 199), UP),
            _ => ((x + dx, y + dy), (dx, dy)),
        },

        _ => unreachable!(),
    }
}

fn solve(input: &str, part2: bool) -> Solution {
    let (map, path) = input.trim_end().split_once("\n\n").unwrap();
    let max_x = map.lines().map(|line| line.len()).max().unwrap();
    let max_y = map.lines().count();
    let mut grid = vec![vec![b' '; max_x]; max_y];

    for (y, line) in map.lines().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            grid[y][x] = ch;
        }
    }

    let initial_y = 0;
    let initial_x = grid[initial_y].iter().position(|&ch| ch != b' ').unwrap();

    let (mut x, mut y) = (initial_x as isize, initial_y as isize);
    let (mut dx, mut dy) = RIGHT;
    let mut path = path.chars().peekable();

    // Processing input
    while let Some(ch) = path.peek() {
        if !ch.is_ascii_digit() {
            // Rotation
            match path.next().unwrap() {
                'L' => (dx, dy) = (dy, -dx),
                'R' => (dx, dy) = (-dy, dx),

                _ => unreachable!(),
            }

            continue;
        }

        // Movement
        let mut steps = 0;
        while let Some(ch) = path.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            let ch = path.next().unwrap();
            steps = steps * 10 + (ch as u8 - b'0') as usize;
        }

        // Stepping to direction
        for _ in 0..steps {
            let ((nx, ny), (ndx, ndy)) = if part2 {
                wrap_on_cube(x, y, dx, dy)
            } else {
                (wrap(x, y, dx, dy, &grid), (dx, dy))
            };

            // We can move only if there's no wall
            match grid[ny as usize][nx as usize] {
                b'.' => (x, y, dx, dy) = (nx, ny, ndx, ndy),
                b'#' => (),
                b' ' => panic!("Stepped on whitespace, which shouldn't occur"),

                _ => unreachable!(),
            }
        }
    }

    // Generating password
    let facing = match (dx, dy) {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,

        _ => unreachable!(),
    };

    let row = y as usize + 1;
    let col = x as usize + 1;

    1000 * row + 4 * col + facing
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(input.trim_end(), false);
    let solution2: Solution = solve(input.trim_end(), true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
