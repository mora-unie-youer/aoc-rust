use aoc_2020::*;

const DAY: i32 = 13;
type Solution = usize;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = get_input_text(DAY);
    let (start, buses) = input.trim().split_once('\n').unwrap();
    let start: usize = start.parse().unwrap();

    let solution1: Solution = {
        let mut buses: Vec<usize> = buses
            .split(',')
            .filter_map(|line| line.parse().ok())
            .collect();
        buses.sort_by_key(|v| (start / v + 1) * v);

        let bus = *buses.first().unwrap();
        let time = (start / bus + 1) * bus;
        bus * (time - start)
    };

    let solution2: Solution = {
        let buses: Vec<(usize, usize)> = buses
            .split(',')
            .enumerate()
            .filter_map(|(offset, bus)| bus.parse().ok().map(|v| (offset, v)))
            .collect();

        let mut delta = 1;
        let mut time = 0;
        for (offset, bus) in buses {
            while (time + offset) % bus != 0 {
                time += delta;
            }

            delta = lcm(delta, bus);
        }

        time
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
