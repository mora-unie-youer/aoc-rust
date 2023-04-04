use aoc_2017::*;

const DAY: i32 = 15;
type Solution = usize;

struct Generator1 {
    factor: usize,
    previous: usize,
}

impl Generator1 {
    fn new(factor: usize, start: usize) -> Self {
        Self { factor, previous: start }
    }
}

impl Iterator for Generator1 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.previous * self.factor) % 2147483647;
        self.previous = current;
        Some(current)
    }
}

struct Generator2 {
    factor: usize,
    multiple: usize,
    previous: usize,
}

impl Generator2 {
    fn new(factor: usize, multiple: usize, start: usize) -> Self {
        Self { factor, multiple, previous: start }
    }
}

impl Iterator for Generator2 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.previous;
        loop {
            current = (current * self.factor) % 2147483647;
            if current % self.multiple == 0 {
                self.previous = current;
                break Some(current);
            }
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let starts: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .nth(4)
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();

    let solution1: Solution = {
        let mut a = Generator1::new(16807, starts[0]);
        let mut b = Generator1::new(48271, starts[1]);

        let mut count = 0;
        for _ in 0..40_000_000 {
            let av = a.next().unwrap();
            let bv = b.next().unwrap();

            if av & 0xffff == bv & 0xffff {
                count += 1;
            }
        }

        count
    };

    let solution2: Solution = {
        let mut a = Generator2::new(16807, 4, starts[0]);
        let mut b = Generator2::new(48271, 8, starts[1]);

        let mut count = 0;
        for _ in 0..5_000_000 {
            let av = a.next().unwrap();
            let bv = b.next().unwrap();

            if av & 0xffff == bv & 0xffff {
                count += 1;
            }
        }

        count
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
