use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_2018::*;

const DAY: i32 = 6;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(", ").unwrap();
        Self(x.parse().unwrap(), y.parse().unwrap())
    }
}

impl Point {
    fn distance(&self, other: Point) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

fn main() {
    let input = get_input_text(DAY);
    let points: Vec<_> = input.lines().map(Point::from).collect();

    let (min_x, min_y, max_x, max_y) = {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            std::isize::MAX,
            std::isize::MAX,
            std::isize::MIN,
            std::isize::MIN,
        );

        for point in &points {
            min_x = min_x.min(point.0);
            min_y = min_y.min(point.1);
            max_x = max_x.max(point.0);
            max_y = max_y.max(point.1);
        }

        (min_x, min_y, max_x, max_y)
    };

    let solution1: Solution = {
        let mut area_sizes = HashMap::new();
        let mut infinite_areas = HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let current_point = Point(x, y);
                let mut closest_point: Option<Point> = None;
                let mut closest_distance = std::usize::MAX;
                let mut same_distance = false;

                for &point in &points {
                    let distance = current_point.distance(point);

                    match distance.cmp(&closest_distance) {
                        Ordering::Less => {
                            closest_distance = distance;
                            closest_point = Some(point);
                            same_distance = false;
                        }
                        Ordering::Equal => {
                            same_distance = true;
                        }
                        _ => (),
                    }
                }

                if let Some(closest) = closest_point {
                    if same_distance {
                        continue;
                    }

                    if x == min_x || x == max_x || y == min_y || y == max_y {
                        infinite_areas.insert(closest);
                    }

                    *area_sizes.entry(closest).or_insert(0) += 1;
                }
            }
        }

        area_sizes
            .into_iter()
            .filter(|(center, _)| !infinite_areas.contains(center))
            .max_by_key(|&(_, size)| size)
            .unwrap()
            .1
    };

    let solution2: Solution = {
        let mut region_size = 0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let current_point = Point(x, y);

                let distances: usize = points
                    .iter()
                    .map(|point| point.distance(current_point))
                    .sum();

                if distances < 10_000 {
                    region_size += 1;
                }
            }
        }

        region_size
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
