use aoc_2015::*;

const DAY: i32 = 15;
type Solution = isize;

#[derive(Clone, Copy)]
struct Ingridient {
    capacity: Solution,
    durability: Solution,
    flavor: Solution,
    texture: Solution,
    calories: Solution,
}

impl Ingridient {
    fn with_amount(self, amount: isize) -> Self {
        Self {
            capacity: self.capacity * amount,
            durability: self.durability * amount,
            flavor: self.flavor * amount,
            texture: self.texture * amount,
            calories: self.calories * amount,
        }
    }
}

impl From<&str> for Ingridient {
    fn from(value: &str) -> Self {
        let mut split = value.split([' ', ',']);
        Self {
            capacity: split.nth(2).unwrap().parse().unwrap(),
            durability: split.nth(2).unwrap().parse().unwrap(),
            flavor: split.nth(2).unwrap().parse().unwrap(),
            texture: split.nth(2).unwrap().parse().unwrap(),
            calories: split.nth(2).unwrap().parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct AmountIterator {
    amounts: [isize; 4],
    finished: bool,
}

impl AmountIterator {
    fn new() -> Self {
        Self {
            amounts: [1, 1, 1, 97],
            finished: false,
        }
    }
}

const MAX_INGREDIENTS: isize = 100;
impl Iterator for AmountIterator {
    type Item = [isize; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut amount_iter = self.amounts.iter_mut().rev();
        let a3 = amount_iter.next().unwrap();
        let a2 = amount_iter.next().unwrap();
        *a3 -= 1;
        *a2 += 1;

        if *a3 == 0 {
            *a2 = 1;

            let a1 = amount_iter.next().unwrap();
            let a0 = amount_iter.next().unwrap();
            *a1 += 1;
            if *a1 > MAX_INGREDIENTS - 2 - *a0 {
                *a1 = 1;
                *a0 += 1;
                if *a0 == MAX_INGREDIENTS - 3 {
                    self.finished = true;
                }
            }

            *a3 = MAX_INGREDIENTS - *a0 - *a1 - *a2;
        }

        Some(self.amounts.clone())
    }
}

fn solve(
    amounts_iter: AmountIterator,
    ingridients: &Vec<Ingridient>,
    filter_calories: bool,
) -> Solution {
    amounts_iter
        .map(|amounts| {
            let props = ingridients
                .iter()
                .zip(amounts)
                .map(|(ingridient, amount)| ingridient.with_amount(amount))
                .fold([0, 0, 0, 0, 0], |mut props, ing| {
                    props[0] += ing.capacity;
                    props[1] += ing.durability;
                    props[2] += ing.flavor;
                    props[3] += ing.texture;
                    props[4] += ing.calories;
                    props
                })
                .map(|prop| prop.max(0));
            (props[..4].iter().product(), props[4])
        })
        .filter(|(_, calories)| !filter_calories || *calories == 500)
        .map(|(score, _)| score)
        .max()
        .unwrap()
}

fn main() {
    let input = get_input_text(DAY);

    let ingridients: Vec<Ingridient> = input.lines().map(|line| line.into()).collect();
    let amounts_iter = AmountIterator::new();

    let solution1: Solution = solve(amounts_iter.clone(), &ingridients, false);
    let solution2: Solution = solve(amounts_iter.clone(), &ingridients, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
