use aoc_2017::*;

const DAY: i32 = 19;
type Solution = String;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    // [up, right, down, left]
    fn index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn next(&self, chars: [char; 4]) -> Self {
        let mut directions: Vec<_> = chars.iter().enumerate().collect();
        let opposite = (self.index() + 2) % 4;
        directions.remove(opposite);

        // "junctions" are surrounded by whitespaces
        let direction = directions
            .iter()
            .find(|(_, ch)| !ch.is_ascii_whitespace())
            .unwrap();
        Direction::from(direction.0)
    }

    fn step(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn start_pos(&self) -> (usize, usize) {
        let x = self.grid[0].iter().position(|&ch| ch != ' ').unwrap();
        (x, 0)
    }

    fn at(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn next_direction(&self, x: usize, y: usize, dir: Direction) -> Direction {
        if self.at(x, y) != '+' {
            return dir;
        }

        // There're walls made of whitespaces, no need to check for out of bounds
        let chars = [
            self.at(x, y - 1), // Up
            self.at(x + 1, y), // Right
            self.at(x, y + 1), // Down
            self.at(x - 1, y), // Left
        ];

        dir.next(chars)
    }
}

fn solve(input: &str) -> (Solution, Solution) {
    let grid = Grid::new(input);

    let mut s = String::new();

    let mut dir = Direction::Down;
    let (mut x, mut y) = grid.start_pos();
    let mut steps = 0;

    loop {
        steps += 1;
        (x, y) = dir.step(x, y);
        dir = grid.next_direction(x, y, dir);

        let ch = grid.at(x, y);
        if ch.is_ascii_whitespace() {
            break (s, steps.to_string());
        } else if ch.is_alphabetic() {
            s.push(ch);
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let (solution1, solution2) = solve(&input);
    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
                ";

        let (s, steps) = solve(input);
        assert_eq!(s, "ABCDEF");
        assert_eq!(steps, "38");
    }
}
