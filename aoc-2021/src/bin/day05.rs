use std::ops::RangeInclusive;

use aoc_2021::*;

const DAY: i32 = 5;
type Solution = usize;

#[derive(Clone)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once(" -> ").unwrap();
        let (start_x, start_y) = start.split_once(',').unwrap();
        let (end_x, end_y) = end.split_once(',').unwrap();

        Self {
            start: (start_x.parse().unwrap(), start_y.parse().unwrap()),
            end: (end_x.parse().unwrap(), end_y.parse().unwrap()),
        }
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        let vertical = self.start.0 == self.end.0;
        let horizontal = self.start.1 == self.end.1;
        horizontal | vertical
    }

    fn x_range(&self) -> RangeInclusive<usize> {
        if self.start.0 > self.end.0 {
            self.end.0..=self.start.0
        } else {
            self.start.0..=self.end.0
        }
    }

    fn y_range(&self) -> RangeInclusive<usize> {
        if self.start.1 > self.end.1 {
            self.end.1..=self.start.1
        } else {
            self.start.1..=self.end.1
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let lines: Vec<_> = input.lines().map(Line::from).collect();
    let max_x = lines
        .iter()
        .map(|line| *line.x_range().end())
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|line| *line.y_range().end())
        .max()
        .unwrap();

    let solution1: Solution = {
        let straight_lines: Vec<_> = lines
            .iter()
            .filter(|line| line.is_straight())
            .cloned()
            .collect();

        let mut map = vec![vec![0; max_x + 1]; max_y + 1];
        for line in straight_lines {
            for y in line.y_range() {
                for x in line.x_range() {
                    map[y][x] += 1;
                }
            }
        }

        map.iter().flatten().filter(|&&count| count > 1).count()
    };

    let solution2: Solution = {
        let mut map = vec![vec![0; max_x + 1]; max_y + 1];
        for line in lines {
            if line.is_straight() {
                for y in line.y_range() {
                    for x in line.x_range() {
                        map[y][x] += 1;
                    }
                }
            } else {
                let start = (line.start.0 as isize, line.start.1 as isize);
                let end = (line.end.0 as isize, line.end.1 as isize);

                let dx = (end.0 - start.0).signum();
                let dy = (end.1 - start.1).signum();

                let mut pos = start;
                while pos != end {
                    map[pos.1 as usize][pos.0 as usize] += 1;
                    pos.0 += dx;
                    pos.1 += dy;
                }

                // Including end
                map[pos.1 as usize][pos.0 as usize] += 1;
            }
        }

        map.iter().flatten().filter(|&&count| count > 1).count()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
