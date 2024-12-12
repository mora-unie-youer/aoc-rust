use std::collections::{HashSet, VecDeque};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 12;
type Solution = usize;

type Point = (isize, isize);

fn get_cell(grid: &[Vec<char>], (x, y): Point) -> char {
    if x < 0 || y < 0 || y as usize >= grid.len() || x as usize >= grid[0].len() {
        return '#';
    }

    grid[y as usize][x as usize]
}

fn neighbors((x, y): Point) -> [Point; 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn shape(grid: &[Vec<char>], visited: &mut HashSet<Point>, start: Point) -> HashSet<Point> {
    let mut queue = VecDeque::from([start]);
    let mut shape = HashSet::from([start]);

    while let Some(point) = queue.pop_front() {
        let same_neighbors = neighbors(point)
            .into_iter()
            .filter(|&p| get_cell(grid, point) == get_cell(grid, p));

        for neighbor in same_neighbors {
            if visited.insert(neighbor) {
                shape.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    shape
}

fn perimeter(grid: &[Vec<char>], shape: &HashSet<Point>) -> usize {
    shape
        .iter()
        .map(|&point| {
            4 - neighbors(point)
                .into_iter()
                .filter(|&p| get_cell(grid, point) == get_cell(grid, p))
                .count()
        })
        .sum()
}

fn sides(shape: &HashSet<Point>) -> usize {
    let mut edges = HashSet::new();

    for &point in shape.iter() {
        for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let p = (point.0 + dx, point.1 + dy);
            if shape.contains(&p) {
                continue;
            }

            // Perpendicular direction
            let (pdx, pdy) = (dy, -dx);

            let mut p = point;
            loop {
                let next_p = (p.0 + dx, p.1 + dy);
                let next_pp = (p.0 + pdx, p.1 + pdy);
                if !shape.contains(&next_pp) || shape.contains(&next_p) {
                    break;
                }

                p = next_pp;
            }

            edges.insert((p, dx, dy));
        }
    }

    edges.len()
}

fn main() {
    let input = get_input_text(DAY);

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let solution1: Solution = {
        let mut visited = HashSet::new();

        let mut sum = 0;
        for (i, line) in input.lines().enumerate() {
            for (j, _) in line.chars().enumerate() {
                if visited.contains(&(j as isize, i as isize)) {
                    continue;
                }

                let shape = shape(&grid, &mut visited, (j as isize, i as isize));
                let perimeter = perimeter(&grid, &shape);
                sum += shape.len() * perimeter;
            }
        }

        sum
    };

    let solution2: Solution = {
        let mut visited = HashSet::new();

        let mut sum = 0;
        for (i, line) in input.lines().enumerate() {
            for (j, _) in line.chars().enumerate() {
                if visited.contains(&(j as isize, i as isize)) {
                    continue;
                }

                let shape = shape(&grid, &mut visited, (j as isize, i as isize));
                let sides = sides(&shape);
                sum += shape.len() * sides;
            }
        }

        sum
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
