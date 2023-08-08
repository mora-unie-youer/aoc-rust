use aoc_2022::*;

const DAY: i32 = 15;
type Solution = isize;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,

    beacon_x: isize,
    beacon_y: isize,

    radius: isize,
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let mut parts = value.split([' ', ',', ':']);

        let x: isize = parts.nth(2).unwrap()[2..].parse().unwrap();
        let y: isize = parts.nth(1).unwrap()[2..].parse().unwrap();
        let beacon_x: isize = parts.nth(5).unwrap()[2..].parse().unwrap();
        let beacon_y: isize = parts.nth(1).unwrap()[2..].parse().unwrap();
        let radius = (beacon_x - x).abs() + (beacon_y - y).abs();

        Self {
            x,
            y,
            beacon_x,
            beacon_y,
            radius,
        }
    }
}

fn find_ranges_for_y(sensors: &[Sensor], y: isize) -> Vec<(isize, isize)> {
    let mut ranges = vec![];
    for sensor in sensors {
        let y_distance = (y - sensor.y).abs();

        if sensor.radius >= y_distance {
            let half_length = sensor.radius - y_distance;
            let start = sensor.x - half_length;
            let end = sensor.x + half_length;
            ranges.push((start, end));
        }
    }

    // Merging ranges
    ranges.sort();
    let mut merged_ranges = vec![ranges[0]];
    for range in ranges.into_iter().skip(1) {
        let last_range = merged_ranges.last_mut().unwrap();

        if last_range.1 >= range.0 {
            let new_end = last_range.1.max(range.1);
            last_range.1 = new_end;
            // Ranges intersect, we can merge
        } else {
            // Ranges do not intersect -> inserting
            merged_ranges.push(range);
        }
    }

    merged_ranges
}

fn main() {
    let input = get_input_text(DAY);
    let sensors: Vec<_> = input.lines().map(Sensor::from).collect();

    let mut beacons: Vec<_> = sensors
        .iter()
        .map(|sensor| (sensor.beacon_x, sensor.beacon_y))
        .collect();
    beacons.sort();
    beacons.dedup();

    let solution1: Solution = {
        const Y: isize = 2_000_000;
        let ranges = find_ranges_for_y(&sensors, Y);

        let beacons_on_range = beacons
            .iter()
            .filter(|(x, y)| y == &Y && ranges.iter().any(|(start, end)| start <= x && x <= end))
            .count();

        let ranges_total_length: isize =
            ranges.into_iter().map(|(start, end)| end - start + 1).sum();

        ranges_total_length - beacons_on_range as isize
    };

    let solution2: Solution = {
        let mut result = 0;

        // As we have such huge Y range, we should search from the end :^)
        for y in (0..=4_000_000).rev() {
            let ranges = find_ranges_for_y(&sensors, y);

            if ranges.len() == 2 {
                let x = ranges[0].1 + 1;
                result = 4000000 * x + y;
                break;
            }
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
