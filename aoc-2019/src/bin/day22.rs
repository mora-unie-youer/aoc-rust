use aoc_2019::*;

const DAY: i32 = 22;
type Solution = usize;

enum Shuffle {
    New,
    Cut(isize),
    Increment(usize),
}

impl From<&str> for Shuffle {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split_ascii_whitespace().collect();
        match parts[0] {
            "cut" => Self::Cut(parts[1].parse().unwrap()),
            _ if parts[2] == "increment" => Self::Increment(parts[3].parse().unwrap()),
            _ if parts[2] == "new" => Self::New,
            _ => unreachable!(),
        }
    }
}

fn shuffle_cards(mut cards: Vec<usize>, shuffle: &Shuffle) -> Vec<usize> {
    match shuffle {
        Shuffle::New => {
            cards.reverse();
            cards
        }
        Shuffle::Cut(n) => {
            let n = if *n > 0 { *n } else { cards.len() as isize + n } as usize;

            let (first, second) = cards.split_at(n);
            let mut new_cards = vec![];
            new_cards.extend(second);
            new_cards.extend(first);
            new_cards
        }
        Shuffle::Increment(step) => {
            let step = *step;
            let mut new_cards = vec![std::usize::MAX; cards.len()];
            let mut i = 0;
            cards.reverse();

            while !cards.is_empty() {
                new_cards[i] = cards.pop().unwrap();
                i = (i + step) % new_cards.len();
            }

            new_cards
        }
    }
}

fn solve_part1(input: &str, size: usize) -> Vec<usize> {
    let steps: Vec<_> = input.lines().map(Shuffle::from).collect();
    let mut cards: Vec<_> = (0..size).collect();
    cards = steps.iter().fold(cards, shuffle_cards);
    cards
}

fn modular_inverse(a: i128, m: i128) -> i128 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = m;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        t -= quotient * newt;
        r -= quotient * newr;
        std::mem::swap(&mut t, &mut newt);
        std::mem::swap(&mut r, &mut newr);
    }

    if r > 1 {
        panic!("invalid n");
    }

    if t < 0 {
        t += m;
    }

    t
}

fn modular_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    assert!(modulus > 0 && (modulus - 1) < std::u64::MAX as i128);
    if modulus == 1 {
        return 0;
    }

    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if (exp % 2) == 1 {
            res = (res * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    res
}

fn solve_part2(input: &str, size: usize, loops: usize, value: usize) -> i128 {
    let size = size as i128;

    let steps: Vec<_> = input.lines().map(Shuffle::from).collect();
    let mut increment: i128 = 1; // step between two numbers
    let mut offset: i128 = 0; // offset of first number

    for step in steps.into_iter().rev() {
        match step {
            Shuffle::New => {
                // // Reverse
                // increment *= -1;
                // increment %= size;
                // // Shift
                // offset += increment;
                // offset %= size;
                offset += 1;
                let x = size - 1;
                increment = (increment * x) % size;
                offset = (offset * x) % size;
            }
            Shuffle::Cut(n) => {
                let x = if n < 0 { n + size as isize } else { n } as i128;
                offset = (offset + x) % size;
                // offset += n as i128 * increment;
                // offset %= size;
            }
            Shuffle::Increment(n) => {
                // increment *= n as i128;
                let inverse = modular_inverse(n as i128, size);
                increment = (increment * inverse) % size;
                offset = (offset * inverse) % size;
            }
        }
    }

    let mx = modular_pow(increment, loops as i128, size);
    let pmx = (value as i128 * mx) % size;
    let amx = (offset * mx) % size;
    let inv = modular_inverse(increment - 1, size);
    let res = (pmx + (amx - offset) * inv) % size;

    if res < 0 {
        res + size
    } else {
        res
    }
}

fn main() {
    let input = get_input_text(DAY);
    // let steps: Vec<_> = input.lines().map(Shuffle::from).collect();

    let solution1: Solution = {
        let cards = solve_part1(&input, 10007);
        cards.into_iter().position(|v| v == 2019).unwrap()
    };

    let solution2: Solution = {
        const SIZE: usize = 119_315_717_514_047;
        const LOOPS: usize = 101_741_582_076_661;
        solve_part2(&input, SIZE, LOOPS, 2020) as usize
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve_part1;

    #[test]
    fn test_solve_part1() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        assert_eq!(solve_part1(input, 10), [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let input = "cut 6
deal with increment 7
deal into new stack";
        assert_eq!(solve_part1(input, 10), [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let input = "deal with increment 7
deal with increment 9
cut -2";
        assert_eq!(solve_part1(input, 10), [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        assert_eq!(solve_part1(input, 10), [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
