use aoc_2018::*;

const DAY: i32 = 25;
type Solution = usize;

struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
            w: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Point {
    fn distance(&self, other: &Self) -> usize {
        let dx = (self.x - other.x).unsigned_abs();
        let dy = (self.y - other.y).unsigned_abs();
        let dz = (self.z - other.z).unsigned_abs();
        let dw = (self.w - other.w).unsigned_abs();
        dx + dy + dz + dw
    }
}

fn main() {
    let input = get_input_text(DAY);
    let points: Vec<_> = input.lines().map(Point::from).collect();

    let mut constellations: Vec<Vec<Point>> = vec![];
    for point in points {
        let joined: Vec<_> = constellations
            .iter()
            .enumerate()
            .filter(|(_, constellation)| constellation.iter().any(|p| p.distance(&point) <= 3))
            .map(|(i, _)| i)
            .collect();

        match joined.len() {
            0 => constellations.push(vec![point]),
            1 => constellations[joined[0]].push(point),
            _ => {
                let mut merged = vec![point];

                for &i in &joined {
                    merged.append(&mut constellations[i]);
                }

                constellations.retain(|constellation| !constellation.is_empty());
                constellations.push(merged);
            }
        }
    }

    let solution1: Solution = constellations.len();
    show_solution(DAY, solution1);
}
