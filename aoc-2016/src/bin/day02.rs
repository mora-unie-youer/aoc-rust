use aoc_2016::*;

const DAY: i32 = 2;
type Solution = String;

trait Code {
    type Value;

    fn next(&mut self, ch: char);
    fn value(&self) -> Self::Value;
}

// Grid is vertically mirrored for easy calculations
// 7 8 9
// 4 5 6
// 1 2 3
struct Code1 {
    x: usize,
    y: usize,
}

impl Default for Code1 {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl Code for Code1 {
    type Value = usize;

    fn next(&mut self, ch: char) {
        match ch {
            'U' if self.y > 0 => self.y -= 1,
            'D' if self.y < 2 => self.y += 1,
            'L' if self.x > 0 => self.x -= 1,
            'R' if self.x < 2 => self.x += 1,
            _ => (),
        }
    }

    fn value(&self) -> usize {
        self.y * 3 + self.x + 1
    }
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
struct Code2 {
    x: isize,
    y: isize,
}

impl Default for Code2 {
    fn default() -> Self {
        Self { x: -2, y: 0 }
    }
}

impl Code for Code2 {
    type Value = char;

    fn next(&mut self, ch: char) {
        let (new_x, new_y) = match ch {
            'U' => (self.x, self.y - 1),
            'D' => (self.x, self.y + 1),
            'L' => (self.x - 1, self.y),
            'R' => (self.x + 1, self.y),
            _ => unreachable!(),
        };

        if new_x.abs() + new_y.abs() <= 2 {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn value(&self) -> Self::Value {
        match (self.y, self.x) {
            (-2, 0) => '1',
            (-1, -1) => '2',
            (-1, 0) => '3',
            (-1, 1) => '4',
            (0, -2) => '5',
            (0, -1) => '6',
            (0, 0) => '7',
            (0, 1) => '8',
            (0, 2) => '9',
            (1, -1) => 'A',
            (1, 0) => 'B',
            (1, 1) => 'C',
            (2, 0) => 'D',
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .fold((0, Code1::default()), |(result, prev), instr| {
            let mut current = prev;
            instr.chars().for_each(|ch| current.next(ch));
            (result * 10 + current.value(), current)
        })
        .0
        .to_string();

    let solution2: Solution = input
        .lines()
        .fold(
            (String::new(), Code2::default()),
            |(mut result, prev), instr| {
                let mut current = prev;
                instr.chars().for_each(|ch| current.next(ch));
                result.push(current.value());
                (result, current)
            },
        )
        .0;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
