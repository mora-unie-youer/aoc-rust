use std::iter::Peekable;

use aoc_2015::*;

const DAY: i32 = 10;
type Solution = usize;

struct LookAndSay<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    peekable: Peekable<I>,
}

impl<I> Iterator for LookAndSay<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let current = match self.peekable.next() {
            Some(v) => v,
            None => return None,
        };

        let mut counter = 1;
        loop {
            match self.peekable.peek() {
                Some(v) if &current == v => {
                    self.peekable.next();
                    counter += 1;
                }
                _ => break,
            }
        }

        Some((counter, current))
    }
}

trait IteratorTools: Iterator {
    fn look_and_say(self) -> LookAndSay<Self>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        LookAndSay {
            peekable: self.peekable(),
        }
    }
}
impl<T> IteratorTools for T where T: Iterator {}

fn look_and_say(s: String) -> String {
    s.chars()
        .look_and_say()
        .fold(String::new(), |mut s, (count, ch)| {
            s.push_str(&count.to_string());
            s.push(ch);
            s
        })
}

fn main() {
    let input = get_input_text(DAY);

    let s = (0..40).fold(input.trim().to_string(), |acc, _| look_and_say(acc));
    let solution1: Solution = s.len();

    let s = (0..10).fold(s, |acc, _| look_and_say(acc));
    let solution2: Solution = s.len();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
