use std::collections::HashSet;

use aoc_2022::*;

const DAY: i32 = 17;
type Solution = usize;

type Cell = (isize, isize);
const SHAPES: [&[Cell]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

struct Cave<'input> {
    occupied: HashSet<Cell>,
    shapes: Box<dyn Iterator<Item = &'static [Cell]>>,
    movements: &'input mut dyn Iterator<Item = isize>,

    highest_y: isize,
}

impl<'input> Cave<'input> {
    fn new(movements: &'input mut dyn Iterator<Item = isize>) -> Self {
        Self {
            shapes: Box::new(SHAPES.into_iter().cycle()),
            movements,
            occupied: HashSet::new(),

            highest_y: 0,
        }
    }
}

impl Cave<'_> {
    fn can_move(&self, shape: &[Cell], dx: isize, dy: isize) -> bool {
        const CAVE_WIDTH: isize = 7;

        shape.iter().all(|(x, y)| {
            let new_x = x + dx;
            let new_y = y + dy;
            (0..CAVE_WIDTH).contains(&new_x)
                && new_y > 0
                && !self.occupied.contains(&(new_x, new_y))
        })
    }

    fn drop_rock(&mut self) {
        const BASE_X: isize = 2;
        const BASE_Y: isize = 4;

        let mut shape = self.shapes.next().unwrap().to_vec();
        // Moving the shape to needed coordinate
        for (x, y) in &mut shape {
            *x += BASE_X;
            *y += BASE_Y + self.highest_y;
        }

        // Dropping shape until it can't move
        let mut down = false;
        loop {
            // Getting movement for this tick
            let (dx, dy) = if down {
                (0, -1)
            } else {
                (self.movements.next().unwrap(), 0)
            };

            // Checking if we can move the shape
            let can_move = self.can_move(&shape, dx, dy);

            if can_move {
                // If we can move, doing so
                for (x, y) in &mut shape {
                    *x += dx;
                    *y += dy;
                }
            } else if down {
                // If we can't move and the direction was down -> it fell completely
                break;
            }

            // Toggling movement
            down = !down;
        }

        // Occupying new space
        for cell in shape {
            self.occupied.insert(cell);
            self.highest_y = self.highest_y.max(cell.1);
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        const ROCKS: usize = 2022;

        let mut movements = input
            .trim()
            .bytes()
            .map(|ch| if ch == b'>' { 1 } else { -1 })
            .cycle();
        let mut cave = Cave::new(&mut movements);

        for _ in 0..ROCKS {
            cave.drop_rock();
        }

        cave.highest_y as usize
    };

    let solution2: Solution = {
        const ROCKS: usize = 1000000000000;
        const ROCKS_TO_APPROXIMATE: usize = 10000;
        const SKIP_FIRST_ROCKS: usize = 1000;

        let mut movements = input
            .trim()
            .bytes()
            .map(|ch| if ch == b'>' { 1 } else { -1 })
            .cycle();
        let mut cave = Cave::new(&mut movements);

        let mut deltas = Vec::with_capacity(ROCKS_TO_APPROXIMATE);
        for _ in 0..ROCKS_TO_APPROXIMATE {
            let prev_height = cave.highest_y;
            cave.drop_rock();
            deltas.push((cave.highest_y - prev_height) as usize);
        }

        let skipped_deltas = &deltas[SKIP_FIRST_ROCKS..];
        let pattern_length = (1..=skipped_deltas.len() / 2)
            .find(|&len| {
                let pattern = &skipped_deltas[..len];
                (len..skipped_deltas.len()).all(|i| skipped_deltas[i] == pattern[i % len])
            })
            .unwrap();

        let pre_skip_delta: usize = deltas[..SKIP_FIRST_ROCKS].iter().sum();

        let pattern_delta: usize = skipped_deltas[..pattern_length].iter().sum();
        let pattern_count = (ROCKS - SKIP_FIRST_ROCKS) / pattern_length;

        let pattern_remaining = (ROCKS - SKIP_FIRST_ROCKS) % pattern_length;
        let remaining_delta: usize = skipped_deltas[..pattern_remaining].iter().sum();

        pre_skip_delta + pattern_delta * pattern_count + remaining_delta
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
