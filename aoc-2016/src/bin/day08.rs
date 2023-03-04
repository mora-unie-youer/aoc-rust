use aoc_2016::*;

const DAY: i32 = 8;
type Solution = String;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

#[derive(Debug)]
enum Operation {
    Rectangle(usize, usize),
    RotateCol(usize, usize),
    RotateRow(usize, usize),
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let splits: Vec<_> = line.split([' ', 'x', '=']).collect();
        match splits[0] {
            "rect" => Self::Rectangle(splits[1].parse().unwrap(), splits[2].parse().unwrap()),
            _ if splits[1] == "row" => {
                Self::RotateRow(splits[3].parse().unwrap(), splits[5].parse().unwrap())
            }
            _ if splits[1] == "column" => {
                Self::RotateCol(splits[4].parse().unwrap(), splits[6].parse().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

struct Grid([[bool; WIDTH]; HEIGHT]);
impl Default for Grid {
    fn default() -> Self {
        Self([[false; WIDTH]; HEIGHT])
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { '#' } else { ' ' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Grid {
    fn count(&self) -> usize {
        self.0.iter().flatten().filter(|&&pixel| pixel).count()
    }

    fn execute(&mut self, operation: &Operation) {
        match *operation {
            Operation::Rectangle(width, height) => {
                for y in 0..height {
                    for x in 0..width {
                        self.0[y][x] = true;
                    }
                }
            }
            Operation::RotateRow(row, step) => {
                let mut new_row = vec![false; WIDTH];
                for col in 0..WIDTH {
                    let new_col = (col + step) % WIDTH;
                    new_row[new_col] = self.0[row][col];
                }

                self.0[row][..].copy_from_slice(&new_row[..]);
            }
            Operation::RotateCol(col, step) => {
                let mut new_col = vec![false; HEIGHT];
                for row in 0..HEIGHT {
                    let new_row = (row + step) % HEIGHT;
                    new_col[new_row] = self.0[row][col];
                }

                for (row, pixel) in new_col.into_iter().enumerate() {
                    self.0[row][col] = pixel;
                }
            }
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let operations: Vec<_> = input.lines().map(Operation::from).collect();

    let mut grid = Grid::default();
    operations
        .iter()
        .for_each(|operation| grid.execute(operation));
    let solution1: Solution = grid.count().to_string();
    let solution2: Solution = grid.to_string();

    show_solution(DAY, solution1);
    show_solution(DAY, "\n".to_string() + &solution2);
}
