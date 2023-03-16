use std::collections::VecDeque;

use aoc_2016::*;

const DAY: i32 = 21;
type Solution = String;

enum Instruction {
    // i, j
    SwapPosition(usize, usize),
    // letter, letter
    SwapLetter(char, char),
    // steps
    RotateLeft(usize),
    // steps
    RotateRight(usize),
    // letter
    RotateBased(char),
    // from, to
    Reverse(usize, usize),
    // from, to
    Move(usize, usize),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let splits: Vec<_> = input.split(' ').collect();
        match (splits[0], splits[1]) {
            ("swap", "position") => {
                Self::SwapPosition(splits[2].parse().unwrap(), splits[5].parse().unwrap())
            }
            ("swap", "letter") => Self::SwapLetter(
                splits[2].chars().next().unwrap(),
                splits[5].chars().next().unwrap(),
            ),
            ("rotate", "left") => Self::RotateLeft(splits[2].parse().unwrap()),
            ("rotate", "right") => Self::RotateRight(splits[2].parse().unwrap()),
            ("rotate", "based") => Self::RotateBased(splits[6].chars().next().unwrap()),
            ("reverse", _) => Self::Reverse(splits[2].parse().unwrap(), splits[4].parse().unwrap()),
            ("move", _) => Self::Move(splits[2].parse().unwrap(), splits[5].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    fn apply(&self, password: &mut VecDeque<char>) {
        match *self {
            Instruction::SwapPosition(i, j) => password.swap(i, j),
            Instruction::RotateLeft(steps) => password.rotate_left(steps),
            Instruction::RotateRight(steps) => password.rotate_right(steps),

            Instruction::SwapLetter(a, b) => {
                let i = password.iter().position(|&ch| ch == a).unwrap();
                let j = password.iter().position(|&ch| ch == b).unwrap();
                password.swap(i, j);
            }
            Instruction::RotateBased(letter) => {
                let i = password.iter().position(|&ch| ch == letter).unwrap();
                let steps = 1 + i + if i >= 4 { 1 } else { 0 };
                password.rotate_right(steps % password.len());
            }
            Instruction::Reverse(from, to) => {
                let slice = password.make_contiguous()[from..=to].as_mut();
                slice.reverse();
            }
            Instruction::Move(from, to) => {
                let ch = password.remove(from).unwrap();
                password.insert(to, ch);
            }
        }
    }

    fn unapply(&self, password: &mut VecDeque<char>) {
        match *self {
            Instruction::SwapPosition(i, j) => password.swap(i, j),
            Instruction::RotateLeft(steps) => password.rotate_right(steps),
            Instruction::RotateRight(steps) => password.rotate_left(steps),

            Instruction::SwapLetter(a, b) => {
                let i = password.iter().position(|&ch| ch == a).unwrap();
                let j = password.iter().position(|&ch| ch == b).unwrap();
                password.swap(i, j);
            }
            Instruction::RotateBased(letter) => {
                let index = password.iter().position(|&ch| ch == letter).unwrap();
                // Bruteforcing rotations
                // P.S.: for my solution `rev()` was needed. Maybe in some solutions it will give
                //       incorrect answer...
                for i in (0..password.len()).rev() {
                    let mut old_password = password.clone();
                    old_password.rotate_right(i);
                    // Searching for "old position" and calculating steps
                    let old_index = old_password.iter().position(|&ch| ch == letter).unwrap();
                    let steps = 1 + old_index + if old_index >= 4 { 1 } else { 0 };
                    // If it moves perfectly - using as a result
                    if (old_index + steps) % password.len() == index {
                        password.rotate_left(steps % password.len());
                        break;
                    }
                }
            }
            Instruction::Reverse(from, to) => {
                let slice = password.make_contiguous()[from..=to].as_mut();
                slice.reverse();
            }
            Instruction::Move(from, to) => {
                let ch = password.remove(to).unwrap();
                password.insert(from, ch);
            }
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let solution1: Solution = {
        let mut password: VecDeque<_> = "abcdefgh".chars().collect();
        instructions
            .iter()
            .for_each(|instruction| instruction.apply(&mut password));
        password.iter().collect()
    };

    let solution2: Solution = {
        let mut password: VecDeque<_> = "fbgdceah".chars().collect();
        instructions
            .iter()
            .rev()
            .for_each(|instruction| instruction.unapply(&mut password));
        password.iter().collect()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::Instruction;

    #[test]
    fn test_scramble() {
        let instructions = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        let instructions: Vec<_> = instructions.lines().map(Instruction::from).collect();

        let mut password: VecDeque<_> = "abcde".chars().collect();
        instructions
            .iter()
            .for_each(|instruction| instruction.apply(&mut password));
        assert_eq!(password.iter().collect::<String>(), "decab");
    }

    #[test]
    fn test_unscramble() {
        let instructions = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        let instructions: Vec<_> = instructions.lines().map(Instruction::from).collect();

        let mut password: VecDeque<_> = "decab".chars().collect();
        instructions
            .iter()
            .rev()
            .for_each(|instruction| instruction.unapply(&mut password));
        assert_eq!(password.iter().collect::<String>(), "abcde");
    }
}
