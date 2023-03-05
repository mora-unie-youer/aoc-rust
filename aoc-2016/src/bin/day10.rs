use std::collections::HashMap;

use aoc_2016::*;

const DAY: i32 = 10;
type Solution = usize;

type Bots = HashMap<usize, Bot>;
type Outputs = HashMap<usize, usize>;

enum Destination {
    Bot(usize),
    Output(usize),
}

impl From<&[&str]> for Destination {
    fn from(input: &[&str]) -> Self {
        let id = input[1].parse().unwrap();
        match input[0] {
            "bot" => Self::Bot(id),
            "output" => Self::Output(id),
            _ => unreachable!(),
        }
    }
}

impl Destination {
    fn write(&self, value: usize, bots: &mut Bots, outputs: &mut Outputs) {
        match *self {
            Destination::Bot(i) => bots.entry(i).or_insert(Bot::default()).put(value),
            Destination::Output(i) => {
                outputs.insert(i, value);
            }
        }
    }
}

enum Command {
    Take(usize, usize),
    Give(usize, Destination, Destination),
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let splits: Vec<_> = input.split(' ').collect();

        if splits.len() == 6 {
            let value = splits[1].parse().unwrap();
            let bot = splits[5].parse().unwrap();
            Self::Take(value, bot)
        } else {
            let bot = splits[1].parse().unwrap();
            let low_dest = Destination::from(&splits[5..=6]);
            let high_dest = Destination::from(&splits[10..=11]);
            Self::Give(bot, low_dest, high_dest)
        }
    }
}

#[derive(Default)]
struct Bot(Option<usize>, Option<usize>);

impl Bot {
    fn is_ready(&self) -> bool {
        // Left is filled earlier than right
        self.1.is_some()
    }

    fn get(&mut self) -> (usize, usize) {
        if !self.is_ready() {
            panic!("Bot doesn't have two chips");
        }

        let a = self.0.unwrap();
        let b = self.1.unwrap();

        (a.min(b), a.max(b))
    }

    fn put(&mut self, value: usize) {
        match *self {
            Self(None, _) => self.0 = Some(value),
            Self(_, None) => self.1 = Some(value),
            _ => panic!("Bot already has two chips"),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let mut responsible_bot = 0;
    let mut bots: Bots = HashMap::new();
    let mut outputs: Outputs = HashMap::new();

    let mut commands: Vec<_> = input.lines().map(Command::from).collect();
    while !commands.is_empty() {
        commands.retain(|command| match command {
            Command::Take(value, bot) => {
                bots.entry(*bot).or_insert(Bot::default()).put(*value);
                false
            }
            Command::Give(bot, low, high) => {
                let (lv, hv) = {
                    let bot = bots.entry(*bot).or_insert(Bot::default());

                    if !bot.is_ready() {
                        return true;
                    }

                    bot.get()
                };

                if lv == 17 && hv == 61 {
                    responsible_bot = *bot;
                }

                low.write(lv, &mut bots, &mut outputs);
                high.write(hv, &mut bots, &mut outputs);

                false
            }
        });
    }

    let solution1: Solution = responsible_bot;
    let solution2: Solution = outputs[&0] * outputs[&1] * outputs[&2];

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
