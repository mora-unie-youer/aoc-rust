#![feature(once_cell)]

use aoc_2016::*;

const DAY: i32 = 4;
type Solution = usize;

trait Rotate {
    fn rotate(&self, times: u8) -> Self;
}

impl Rotate for char {
    // Doesn't support `times` bigger than one cycle.
    // And it doesn't need to do so :P
    fn rotate(&self, times: u8) -> Self {
        let mut new_code = *self as u8 + times;
        if new_code > b'z' {
            new_code = b'a' + new_code % b'z' - 1;
        }

        new_code as Self
    }
}

struct Room<'input> {
    id: usize,
    name: &'input str,
    checksum: &'input str,
}

impl<'input> From<&'input str> for Room<'input> {
    fn from(value: &'input str) -> Self {
        let regex = regex!(r"(.*)-(\d{3})\[([a-z]{5})\]");
        let captures = regex.captures(value).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let id = captures.get(2).unwrap().as_str().parse().unwrap();
        let checksum = captures.get(3).unwrap().as_str();
        Self { id, name, checksum }
    }
}

impl Room<'_> {
    fn is_real(&self) -> bool {
        let mut counts = [0usize; 26];
        self.name
            .chars()
            .filter(|&ch| ch != '-')
            .map(|ch| (ch as u8 - b'a') as usize)
            .for_each(|ch| counts[ch] += 1);

        let mut char_counts: Vec<_> = counts
            .iter()
            .enumerate()
            .map(|(i, count)| ((i as u8 + b'a') as char, count))
            .collect();

        char_counts.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        let checksum: String = char_counts.iter().take(5).map(|(ch, _)| ch).collect();
        checksum == self.checksum
    }

    fn is_northpole_storage(&self) -> bool {
        let rotate_count = (self.id % 26) as u8;
        let new_name: String = self
            .name
            .chars()
            .map(|ch| {
                if ch == '-' {
                    ' '
                } else {
                    ch.rotate(rotate_count)
                }
            })
            .collect();
        new_name == "northpole object storage"
    }
}

fn main() {
    let input = get_input_text(DAY);
    let rooms: Vec<Room> = input.lines().map(|line| line.into()).collect();

    let solution1: Solution = rooms
        .iter()
        .filter(|room| room.is_real())
        .map(|room| room.id)
        .sum();
    let solution2: Solution = rooms
        .iter()
        .find(|room| room.is_northpole_storage())
        .unwrap()
        .id;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Room, Rotate};

    #[test]
    fn test_char_rotate() {
        assert_eq!('x'.rotate(1), 'y');
        assert_eq!('x'.rotate(2), 'z');
        assert_eq!('x'.rotate(3), 'a');
        assert_eq!('z'.rotate(1), 'a');
    }

    #[test]
    fn test_real_room() {
        assert!(Room::from("aaaaa-bbb-z-y-x-123[abxyz]").is_real());
        assert!(Room::from("a-b-c-d-e-f-g-h-987[abcde]").is_real());
        assert!(Room::from("not-a-real-room-404[oarel]").is_real());
        assert!(!Room::from("totally-real-room-200[decoy]").is_real());
    }
}
