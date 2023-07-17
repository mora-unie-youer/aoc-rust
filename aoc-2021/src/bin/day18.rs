#![feature(array_windows)]

use aoc_2021::*;
use itertools::Itertools;

const DAY: i32 = 18;
type Solution = usize;

// Vector containing tuples of value and depth
type Snailfish = Vec<(usize, usize)>;

fn parse_snailfish(input: &str) -> Snailfish {
    let mut snailfish = Snailfish::new();
    let mut depth = 0;

    for ch in input.chars() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            num => snailfish.push(((num as u8 - b'0') as usize, depth)),
        }
    }

    snailfish
}

fn explode(snailfish: &mut Snailfish) -> bool {
    for (i, [a, b]) in snailfish.array_windows().enumerate() {
        let &(a_num, a_depth) = a;
        let &(b_num, b_depth) = b;

        // If we have pair on 5-th "rank"
        if a_depth == 5 && b_depth == 5 {
            // Add to left
            if i != 0 {
                snailfish[i - 1].0 += a_num;
            }

            // Add to right
            if i < snailfish.len() - 2 {
                snailfish[i + 2].0 += b_num;
            }

            // Remove this pair
            snailfish.remove(i);
            snailfish.remove(i);

            // Insert 0 into this place
            snailfish.insert(i, (0, 4));

            // We can do only one explode due to iterators
            return true;
        }
    }

    false
}

fn split(snailfish: &mut Snailfish) -> bool {
    for (i, &(num, depth)) in snailfish.iter().enumerate() {
        if num > 9 {
            // Split number
            let (a, b) = (num / 2, num / 2 + num % 2);

            // Remove this number
            snailfish.remove(i);

            // Insert new pair
            snailfish.insert(i, (a, depth + 1));
            snailfish.insert(i + 1, (b, depth + 1));

            // We can do only one split due to iterators
            return true;
        }
    }

    false
}

fn reduce(snailfish: &mut Snailfish) {
    while explode(snailfish) || split(snailfish) {}
}

fn add(mut a: Snailfish, b: Snailfish) -> Snailfish {
    a.extend(b);
    a.iter_mut().for_each(|(_, depth)| *depth += 1);
    reduce(&mut a);
    a
}

fn magnitude(mut snailfish: Snailfish) -> usize {
    // We have max depth == 4. Reducing on depths for top
    for depth in (1..=4).rev() {
        // Reducing process
        'reduce: loop {
            for (i, [a, b]) in snailfish.array_windows().enumerate() {
                let &(a_num, a_depth) = a;
                let &(b_num, b_depth) = b;

                // We reduce only depth from variable
                if a_depth == depth && b_depth == depth {
                    // Setting new value
                    snailfish[i] = (a_num * 3 + b_num * 2, depth - 1);
                    // Removing paired value
                    snailfish.remove(i + 1);
                    // We can do only one reduce due to iterators
                    continue 'reduce;
                }
            }

            break 'reduce;
        }
    }

    // Magnitude is stored in the first value
    snailfish[0].0
}

fn main() {
    let input = get_input_text(DAY);
    let numbers: Vec<_> = input.lines().map(parse_snailfish).collect();

    let solution1: Solution = {
        let numbers = numbers.clone();
        let first = numbers[0].clone();
        let sum = numbers.into_iter().skip(1).fold(first, add);
        magnitude(sum)
    };

    let solution2: Solution = numbers
        .into_iter()
        .permutations(2)
        .map(|nums| magnitude(add(nums[0].clone(), nums[1].clone())))
        .max()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
