use aoc_2015::*;

const DAY: i32 = 6;
type Solution = usize;

enum Operation {
    On,
    Off,
    Toggle,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "e" => Self::Toggle,
            "on" => Self::On,
            "off" => Self::Off,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let mut grid = vec![false; 1000 * 1000];
        for instruction in input.lines() {
            let instruction = &instruction[5..];
            let parts: Vec<_> = instruction.split_ascii_whitespace().collect();
            let operation = parts[0].into();
            let (x1, y1) = parts[1].split_once(',').unwrap();
            let (x1, y1): (usize, usize) = (x1.parse().unwrap(), y1.parse().unwrap());
            let (x2, y2) = parts[3].split_once(',').unwrap();
            let (x2, y2): (usize, usize) = (x2.parse().unwrap(), y2.parse().unwrap());

            let mut grid_worker = |func: fn(bool) -> bool| {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        grid[y * 1000 + x] = func(grid[y * 1000 + x]);
                    }
                }
            };

            match operation {
                Operation::On => grid_worker(|_| true),
                Operation::Off => grid_worker(|_| false),
                Operation::Toggle => grid_worker(|v| !v),
            }
        }

        grid.iter().filter(|&&v| v).count()
    };

    let solution2: Solution = {
        let mut grid = vec![0; 1000 * 1000];
        for instruction in input.lines() {
            let instruction = &instruction[5..];
            let parts: Vec<_> = instruction.split_ascii_whitespace().collect();
            let operation = parts[0].into();
            let (x1, y1) = parts[1].split_once(',').unwrap();
            let (x1, y1): (usize, usize) = (x1.parse().unwrap(), y1.parse().unwrap());
            let (x2, y2) = parts[3].split_once(',').unwrap();
            let (x2, y2): (usize, usize) = (x2.parse().unwrap(), y2.parse().unwrap());

            let mut grid_worker = |func: fn(usize) -> usize| {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        grid[y * 1000 + x] = func(grid[y * 1000 + x]);
                    }
                }
            };

            match operation {
                Operation::On => grid_worker(|v| v + 1),
                Operation::Off => grid_worker(|v| if v > 0 { v - 1 } else { 0 }),
                Operation::Toggle => grid_worker(|v| v + 2),
            }
        }

        grid.iter().sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
