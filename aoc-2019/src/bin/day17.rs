use aoc_2019::*;

// Took code from day 11
const DAY: i32 = 17;
type Solution = isize;

#[derive(Clone)]
struct Program {
    ip: isize,
    relative_base: isize,

    data: Vec<isize>,

    input: Vec<isize>,
    output: Vec<isize>,
}

impl Program {
    fn new(data: &[isize], input: &[isize]) -> Self {
        Self {
            ip: 0,
            relative_base: 0,

            data: data.to_vec(),

            input: input.to_vec(),
            output: Vec::new(),
        }
    }

    fn run(&mut self) -> bool {
        loop {
            let opcode = self.data[self.ip as usize] % 100;
            match opcode {
                1 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = op1 + op2;
                    self.ip += 4;
                }
                2 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = op1 * op2;
                    self.ip += 4;
                }
                3 => {
                    if self.input.is_empty() {
                        // If there's no input -> pausing
                        break false;
                    }

                    let out = self.get_offset(1) as usize;
                    *self.get_value_mut(out) = self.input.remove(0);
                    self.ip += 2;
                }
                4 => {
                    let op = self.get_parameter(1);
                    self.output.push(op);
                    self.ip += 2;
                }
                5 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    if op1 != 0 {
                        self.ip = op2;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    if op1 == 0 {
                        self.ip = op2;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = (op1 < op2) as isize;
                    self.ip += 4;
                }
                8 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = (op1 == op2) as isize;
                    self.ip += 4;
                }
                9 => {
                    let op1 = self.get_parameter(1);
                    self.relative_base += op1;
                    self.ip += 2;
                }
                99 => break true,
                _ => panic!("Something went wrong"),
            }
        }
    }

    fn get_mode(&self, parameter: isize) -> u32 {
        self.data[self.ip as usize]
            .to_string()
            .chars()
            .rev()
            .nth((parameter + 1) as usize)
            .unwrap_or('0')
            .to_digit(10)
            .unwrap()
    }

    fn get_offset_with_mode(&self, offset: isize, mode: u32) -> isize {
        match mode {
            2 => self.data[(self.ip + offset) as usize] + self.relative_base,
            _ => self.data[(self.ip + offset) as usize],
        }
    }

    fn get_offset(&self, offset: isize) -> isize {
        let mode = self.get_mode(offset);
        self.get_offset_with_mode(offset, mode)
    }

    fn get_value(&mut self, address: usize) -> isize {
        // Now we usually need to allocate memory
        if self.data.len() <= address {
            self.data.resize(address + 1, 0);
        }

        self.data[address]
    }

    fn get_value_mut(&mut self, address: usize) -> &mut isize {
        // Now we usually need to allocate memory
        if self.data.len() <= address {
            self.data.resize(address + 1, 0);
        }

        &mut self.data[address]
    }

    fn get_parameter(&mut self, parameter: isize) -> isize {
        let mode = self.get_mode(parameter);
        let parameter = self.get_offset_with_mode(parameter, mode);
        match mode {
            0 | 2 => self.get_value(parameter as usize),
            1 => parameter,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let map = {
        let mut program = Program::new(&data, &[]);
        program.run();

        let mut map = vec![];
        let mut row = vec![];
        program.output.pop(); // Remove latest new line
        for tile in program.output {
            if tile == 10 {
                map.push(row);
                row = vec![];
            } else {
                row.push(char::from_u32(tile as u32).unwrap());
                // row.push(if tile == 35 {
                //     '#'
                // } else if tile == 46 {
                //     '.'
                // } else {

                // });
            }
        }

        map
    };

    let solution1: Solution = {
        let mut result = 0;
        for y in 1..map.len() - 1 {
            for x in 1..map[0].len() - 1 {
                let tiles = [
                    map[y - 1][x],
                    map[y][x - 1],
                    map[y][x],
                    map[y][x + 1],
                    map[y + 1][x],
                ];

                if tiles.into_iter().all(|tile| tile == '#') {
                    result += x * y;
                }
            }
        }

        result as isize
    };

    // println!(
    //     "{}",
    //     map.iter()
    //         .map(|row| row.iter().collect::<String>())
    //         .map(|row| format!("{}\n", row))
    //         .collect::<String>()
    // );

    let solution2: Solution = {
        let (mut x, mut y) = (0..map.len())
            .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
            .find(|&(x, y)| map[y][x] == '^')
            .unwrap();
        let (mut dx, mut dy) = (0, -1);

        let mut commands = vec![];
        let mut steps = 0;
        'main: loop {
            let (nx, ny) = (x as isize + dx, y as isize + dy);

            if nx < 0
                || ny < 0
                || nx as usize >= map[0].len()
                || ny as usize >= map.len()
                || map[ny as usize][nx as usize] != '#'
            {
                let choices = [(dy, -dx, "L"), (-dy, dx, "R")];
                for (ndx, ndy, rotation) in choices {
                    let (nx, ny) = (x as isize + ndx, y as isize + ndy);
                    if nx >= 0
                        && ny >= 0
                        && (nx as usize) < map[0].len()
                        && (ny as usize) < map.len()
                        && map[ny as usize][nx as usize] == '#'
                    {
                        (dx, dy) = (ndx, ndy);
                        (x, y) = (nx as usize, ny as usize);

                        if steps != 0 {
                            commands.push(steps.to_string());
                        }

                        steps = 1;
                        commands.push(rotation.to_owned());
                        continue 'main;
                    }
                }

                // Nowhere to turn -> end
                commands.push(steps.to_string());
                break 'main;
            } else {
                (x, y) = (nx as usize, ny as usize);
                steps += 1;
            }
        }

        // Looked at my commands
        // let commands: Vec<_> = commands
        //     .chunks(2)
        //     .map(|chunk| format!("{},{}", chunk[0], chunk[1]))
        //     .collect();

        let a = "R,6,L,10,R,8\n";
        let b = "R,8,R,12,L,8,L,8\n";
        let c = "L,10,R,6,R,6,L,8\n";
        let commands = "A,B,A,B,C,A,B,C,A,C\n";

        let mut program = Program::new(&data, &[]);
        program.data[0] = 2;
        program
            .input
            .extend(commands.as_bytes().iter().map(|&v| v as isize));
        program
            .input
            .extend(a.as_bytes().iter().map(|&v| v as isize));
        program
            .input
            .extend(b.as_bytes().iter().map(|&v| v as isize));
        program
            .input
            .extend(c.as_bytes().iter().map(|&v| v as isize));
        program
            .input
            .extend("n\n".as_bytes().iter().map(|&v| v as isize));
        program.run();

        program.output.pop().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
