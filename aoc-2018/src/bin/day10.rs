use std::ops::{Add, Mul};

use aoc_2018::*;

const DAY: i32 = 10;
type Solution = String;

#[derive(Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let coords = value.split(['<', '>']).nth(1).unwrap();
        let mut coords = coords.split(',').map(|v| v.trim().parse().unwrap());

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

struct Point {
    position: Vector,
    velocity: Vector,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut vectors = value.split("> ").map(Vector::from);

        Self {
            position: vectors.next().unwrap(),
            velocity: vectors.next().unwrap(),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let points: Vec<_> = input.lines().map(Point::from).collect();

    let mut image_time = 0;
    let mut min_image_size = std::isize::MAX;
    let mut borders = (0, 0, 0, 0);
    for time in 0..20000 {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MIN,
            std::isize::MIN,
        );

        for point in &points {
            let point_in_time = point.position + point.velocity * time;
            min_x = min_x.min(point_in_time.x);
            min_y = min_y.min(point_in_time.y);
            max_x = max_x.max(point_in_time.x);
            max_y = max_y.max(point_in_time.y);
        }

        let image_size = (max_x - min_x) * (max_y - min_y);
        if image_size < min_image_size {
            image_time = time;
            min_image_size = image_size;
            borders = (min_x, min_y, max_x, max_y);
        }
    }

    let solution1: Solution = {
        let (min_x, min_y, max_x, max_y) = borders;
        let mut map = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        for point in &points {
            let point_in_time = point.position + point.velocity * image_time;
            let x = (point_in_time.x - min_x) as usize;
            let y = (point_in_time.y - min_y) as usize;
            map[y][x] = '#';
        }

        let mut result = String::from("\n");
        result.push_str(
            &map.iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        );

        result
    };
    let solution2: Solution = image_time.to_string();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
