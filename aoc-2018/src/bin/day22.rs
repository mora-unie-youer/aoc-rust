use aoc_2018::*;
use pathfinding::{directed::astar, prelude::Matrix};

const DAY: i32 = 22;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let (depth, (tx, ty)): (usize, (usize, usize)) = {
        let mut lines = input.lines();
        let depth = lines
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let target = lines
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .split_once(',')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()));
        (depth, target.unwrap())
    };

    const MAP_SIZE: usize = 1000;
    let map = {
        let mut map = Matrix::new(MAP_SIZE + 1, MAP_SIZE + 1, 0);
        for x in 0..=MAP_SIZE {
            for y in 0..=MAP_SIZE {
                let geologic_index = if (x == 0 && y == 0) || (x == tx && y == ty) {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    map[(x - 1, y)] * map[(x, y - 1)]
                };
                map[(x, y)] = (geologic_index + depth) % 20183;
            }
        }

        map.as_mut().iter_mut().for_each(|n| *n %= 3);
        map
    };

    let solution1: Solution = map
        .iter()
        .take(ty + 1)
        .map(|row| row.iter().take(tx + 1).sum::<Solution>())
        .sum();

    const NEITHER: usize = 0b001;
    const TORCH: usize = 0b010;
    const GEAR: usize = 0b100;
    const ALLOWED: [usize; 3] = [TORCH + GEAR, NEITHER + GEAR, NEITHER + TORCH];
    let solution2: Solution = astar::astar(
        &((0, 0), TORCH),
        |&((x, y), eq)| {
            map.neighbours((x, y), false)
                .filter(|&(nx, ny)| ALLOWED[map[(nx, ny)]] & eq == eq)
                .map(|(nx, ny)| (((nx, ny), eq), 1))
                .chain(std::iter::once((((x, y), ALLOWED[map[(x, y)]] - eq), 7)))
                .collect::<Vec<_>>()
        },
        |&((x, y), _)| (x as isize - tx as isize).abs() + (y as isize - ty as isize).abs(),
        |&((x, y), eq)| x == tx && y == ty && eq == TORCH,
    )
    .unwrap()
    .1 as usize;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
