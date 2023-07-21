use std::ops::RangeInclusive;

use aoc_2021::*;

const DAY: i32 = 22;
type Solution = usize;

struct Step {
    enable: bool,
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    zs: RangeInclusive<isize>,
}

impl From<&str> for Step {
    fn from(line: &str) -> Self {
        let (action, ranges) = line.split_once(' ').unwrap();

        let mut ranges = ranges.split(',').map(|range| {
            let (_, range) = range.split_once('=').unwrap();
            let (start, end) = range.split_once("..").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        });

        Self {
            enable: action == "on",
            xs: ranges.next().unwrap(),
            ys: ranges.next().unwrap(),
            zs: ranges.next().unwrap(),
        }
    }
}

struct Cube {
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    zs: RangeInclusive<isize>,
    holes: Vec<Cube>,
    is_empty: bool,
}

impl From<Step> for Cube {
    fn from(value: Step) -> Self {
        Self {
            xs: value.xs,
            ys: value.ys,
            zs: value.zs,
            holes: vec![],
            is_empty: false,
        }
    }
}

impl Cube {
    fn remove_inner(&mut self, other: &Self) {
        if self.is_empty {
            return;
        }

        for hole in &mut self.holes {
            hole.remove_inner(other);
        }

        if let Some(hole) = self.calculate_intersection(other) {
            if self.is_same_region(&hole) {
                self.is_empty = true;
            } else {
                self.holes.push(hole);
            }
        }
    }

    fn calculate_intersection(&self, other: &Self) -> Option<Self> {
        let intersection = |range: &RangeInclusive<isize>, others: &RangeInclusive<isize>| {
            *range.start().max(others.start())..=*range.end().min(others.end())
        };

        let xs = intersection(&self.xs, &other.xs);
        let ys = intersection(&self.ys, &other.ys);
        let zs = intersection(&self.zs, &other.zs);

        if xs.is_empty() || ys.is_empty() || zs.is_empty() {
            None
        } else {
            Some(Self {
                xs,
                ys,
                zs,
                holes: vec![],
                is_empty: false,
            })
        }
    }

    fn is_same_region(&self, hole: &Self) -> bool {
        hole.xs == self.xs && hole.ys == self.ys && hole.zs == self.zs
    }

    fn size(self) -> usize {
        if self.is_empty || self.xs.is_empty() || self.ys.is_empty() || self.zs.is_empty() {
            return 0;
        }

        let xs = self.xs.end() - self.xs.start() + 1;
        let ys = self.ys.end() - self.ys.start() + 1;
        let zs = self.zs.end() - self.zs.start() + 1;
        let volume = (xs * ys * zs) as usize;
        let hole_volume: usize = self.holes.into_iter().map(|cube| cube.size()).sum();

        volume - hole_volume
    }
}

fn solve(input: &str, part2: bool) -> Solution {
    let mut steps: Vec<_> = input.lines().map(Step::from).collect();

    // Trimming ranges
    if !part2 {
        const MIN: isize = -50;
        const MAX: isize = 50;

        let trim = |range: &mut RangeInclusive<isize>| {
            *range = RangeInclusive::new(*range.start().max(&MIN), *range.end().min(&MAX));
        };

        for step in &mut steps {
            trim(&mut step.xs);
            trim(&mut step.ys);
            trim(&mut step.zs);
        }
    }

    let mut enabled_cubes: Vec<Cube> = vec![];
    for step in steps {
        let enable = step.enable;
        let cube = Cube::from(step);

        // Remove "cube" from other cubes
        for other in &mut enabled_cubes {
            other.remove_inner(&cube);
        }

        if enable {
            enabled_cubes.push(cube);
        }
    }

    enabled_cubes.into_iter().map(|cube| cube.size()).sum()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(&input, false);
    let solution2: Solution = solve(&input, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
