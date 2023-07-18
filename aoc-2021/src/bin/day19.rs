use std::{
    cmp::Ordering,
    collections::VecDeque,
    ops::{Add, Sub},
};

use aoc_2021::*;
use itertools::Itertools;

const DAY: i32 = 19;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector {
    fn from(input: &str) -> Self {
        let mut parts = input.split(',');
        let (x, y, z) = (
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        );

        Self { x, y, z }
    }
}

impl From<(isize, isize, isize)> for Vector {
    fn from(value: (isize, isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .cmp(&other.x)
            .then(self.y.cmp(&other.y))
            .then(self.z.cmp(&other.z))
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vector {
    fn distance(&self, other: &Self) -> usize {
        let dx = (self.x - other.x).unsigned_abs();
        let dy = (self.y - other.y).unsigned_abs();
        let dz = (self.z - other.z).unsigned_abs();
        dx + dy + dz
    }
}

#[derive(Clone)]
struct Scanner {
    beacons: Vec<Vector>,
    differences: Vec<Vector>,
}

impl From<&str> for Scanner {
    fn from(input: &str) -> Self {
        let beacons: Vec<_> = input.lines().skip(1).map(Vector::from).collect();
        Self::new(beacons)
    }
}

impl Scanner {
    fn new(mut beacons: Vec<Vector>) -> Self {
        beacons.sort_unstable();
        let differences = beacon_differences(&beacons);

        Self {
            beacons,
            differences,
        }
    }

    fn merge(&mut self, other: &Scanner) {
        self.beacons.extend(&other.beacons);
        self.beacons.sort_unstable();
        self.beacons.dedup();
        self.differences = beacon_differences(&self.beacons);
    }

    fn align_to(&mut self, other: &Scanner) -> Option<Vector> {
        const ALIGNMENT_REQUIREMENT: usize = 12;
        const EDGE_REQUIREMENT: usize = ALIGNMENT_REQUIREMENT * (ALIGNMENT_REQUIREMENT - 1) / 2;
        #[rustfmt::skip]
        const TRANSFORMATIONS: [fn(Vector) -> Vector; 24] = [
            |vec| ( vec.x,  vec.y,  vec.z).into(),
            |vec| (-vec.y,  vec.x,  vec.z).into(),
            |vec| ( vec.y, -vec.x,  vec.z).into(),
            |vec| (-vec.x, -vec.y,  vec.z).into(),

            |vec| ( vec.x, -vec.z,  vec.y).into(),
            |vec| ( vec.z,  vec.x,  vec.y).into(),
            |vec| (-vec.z, -vec.x,  vec.y).into(),
            |vec| (-vec.x,  vec.z,  vec.y).into(),

            |vec| ( vec.x,  vec.z, -vec.y).into(),
            |vec| (-vec.z,  vec.x, -vec.y).into(),
            |vec| ( vec.z, -vec.x, -vec.y).into(),
            |vec| (-vec.x, -vec.z, -vec.y).into(),

            |vec| ( vec.x, -vec.y, -vec.z).into(),
            |vec| ( vec.y,  vec.x, -vec.z).into(),
            |vec| (-vec.y, -vec.x, -vec.z).into(),
            |vec| (-vec.x,  vec.y, -vec.z).into(),

            |vec| ( vec.z,  vec.y, -vec.x).into(),
            |vec| (-vec.y,  vec.z, -vec.x).into(),
            |vec| ( vec.y, -vec.z, -vec.x).into(),
            |vec| (-vec.z, -vec.y, -vec.x).into(),

            |vec| (-vec.z,  vec.y,  vec.x).into(),
            |vec| (-vec.y, -vec.z,  vec.x).into(),
            |vec| ( vec.y,  vec.z,  vec.x).into(),
            |vec| ( vec.z, -vec.y,  vec.x).into(),
        ];

        // Finding orientation
        let mut transformed_beacons = None;
        for transformation in TRANSFORMATIONS {
            let transformed_scanner =
                Scanner::new(self.beacons.iter().cloned().map(transformation).collect());

            let equal_differences =
                count_equal_vectors(&other.differences, &transformed_scanner.differences);

            if equal_differences >= EDGE_REQUIREMENT {
                transformed_beacons = Some(transformed_scanner.beacons);
                break;
            }
        }

        // This can exit only if beacons do not overlap enough
        let transformed_beacons = match transformed_beacons {
            Some(beacons) => beacons,
            _ => return None,
        };

        // Finding position
        let mut stack = transformed_beacons.clone();
        let mut positioned_beacons = None;
        let mut position = None;
        'position: while let Some(beacon) = stack.pop() {
            for &other_beacon in &other.beacons {
                let offset = other_beacon - beacon;
                let aligned_beacons: Vec<_> = transformed_beacons
                    .iter()
                    .cloned()
                    .map(|beacon| beacon + offset)
                    .collect();

                let equal_beacons = count_equal_vectors(&other.beacons, &aligned_beacons);
                if equal_beacons >= ALIGNMENT_REQUIREMENT {
                    positioned_beacons = Some(aligned_beacons);
                    position = Some(offset);
                    break 'position;
                }
            }
        }

        // This can exit only if beacons do not match
        let beacons = match positioned_beacons {
            Some(beacons) => beacons,
            _ => return None,
        };

        self.beacons = beacons;
        position
    }
}

fn beacon_differences(beacons: &[Vector]) -> Vec<Vector> {
    let mut differences = Vec::with_capacity(((beacons.len() - 1) * beacons.len()) / 2);
    let mut stack: Vec<_> = beacons.to_vec();

    while let Some(beacon) = stack.pop() {
        for &other_beacon in &stack {
            differences.push(beacon - other_beacon);
        }
    }

    differences.sort_unstable();
    differences
}

// `a` and `b` must be sorted
fn count_equal_vectors(a: &[Vector], b: &[Vector]) -> usize {
    let mut count = 0;
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
            Ordering::Greater => j += 1,
            Ordering::Less => i += 1,
        }
    }

    count
}

fn main() {
    let input = get_input_text(DAY);

    let scanners: Vec<Scanner> = input.split("\n\n").map(Scanner::from).collect();
    let mut positions = vec![(0, 0, 0).into()];

    let mut aligned = scanners[0].clone();
    let mut queue: VecDeque<Scanner> = scanners.into_iter().skip(1).collect();

    while let Some(mut scanner) = queue.pop_front() {
        if let Some(position) = scanner.align_to(&aligned) {
            // We aligned scanner, now we need to merge
            aligned.merge(&scanner);
            positions.push(position);
        } else {
            // As we couldn't align scanner,
            queue.push_back(scanner);
        }
    }

    let solution1: Solution = aligned.beacons.len();
    let solution2: Solution = positions
        .into_iter()
        .combinations(2)
        .map(|v| v[0].distance(&v[1]))
        .max()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
