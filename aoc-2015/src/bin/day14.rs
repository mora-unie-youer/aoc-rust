use aoc_2015::*;

const DAY: i32 = 14;
type Solution = usize;

#[derive(Clone)]
struct Reindeer {
    speed: Solution,
    run_time: usize,
    rest_time: usize,

    running: bool,
    until_change: usize,
    distance: Solution,
    points: Solution,
}

impl Reindeer {
    fn new(line: &str) -> Self {
        let split: Vec<_> = line.split(' ').collect();
        let run_time = split[6].parse().unwrap();
        Self {
            speed: split[3].parse().unwrap(),
            run_time,
            rest_time: split[13].parse().unwrap(),

            running: true,
            until_change: run_time,
            distance: 0,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if self.running {
            self.distance += self.speed;
        }

        self.until_change -= 1;
        if self.until_change == 0 {
            self.running = !self.running;
            self.until_change = if self.running {
                self.run_time
            } else {
                self.rest_time
            };
        }
    }
}

const PASSED_TIME: usize = 2503;
fn main() {
    let input = get_input_text(DAY);
    let mut reindeers: Vec<_> = input.lines().map(|line| Reindeer::new(line)).collect();

    //// Solution without simulating the game. Unused because we have to simulate anyway
    // let solution1: Solution = reindeers
    //     .iter()
    //     .map(|reindeer| {
    //         let Reindeer {
    //             speed,
    //             run_time,
    //             rest_time,
    //             ..
    //         } = *reindeer;
    //
    //         let cycle = run_time + rest_time;
    //         let (cycles, last_cycle) = (PASSED_TIME / cycle, PASSED_TIME % cycle);
    //         speed * (cycles * run_time + last_cycle.min(run_time))
    //     })
    //     .max()
    //     .unwrap();

    for _ in 0..PASSED_TIME {
        reindeers.iter_mut().for_each(|reindeer| reindeer.tick());

        let lead = reindeers
            .iter()
            .max_by_key(|reindeer| reindeer.distance)
            .unwrap()
            .distance;
        reindeers
            .iter_mut()
            .filter(|reindeer| reindeer.distance == lead)
            .for_each(|reindeer| reindeer.points += 1);
    }

    let solution1: Solution = reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.distance)
        .unwrap()
        .distance;
    let solution2: Solution = reindeers
        .iter()
        .max_by_key(|reindeer| reindeer.points)
        .unwrap()
        .points;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
