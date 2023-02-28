use aoc_2015::*;
use phf::phf_map;

const DAY: i32 = 16;
type Solution = usize;

enum Count {
    Equal(usize),
    Greater(usize),
    Less(usize),
}

impl Count {
    fn value(&self) -> usize {
        match *self {
            Self::Equal(v) => v,
            Self::Greater(v) => v,
            Self::Less(v) => v,
        }
    }

    fn matches(&self, count: usize) -> bool {
        match *self {
            Self::Equal(v) => count == v,
            Self::Greater(v) => count > v,
            Self::Less(v) => count < v,
        }
    }
}

static INFO: phf::Map<&'static str, Count> = phf_map! {
    "children" => Count::Equal(3),
    "cats" => Count::Greater(7),
    "samoyeds" => Count::Equal(2),
    "pomeranians" => Count::Less(3),
    "akitas" => Count::Equal(0),
    "vizslas" => Count::Equal(0),
    "goldfish" => Count::Less(5),
    "trees" => Count::Greater(3),
    "cars" => Count::Equal(2),
    "perfumes" => Count::Equal(1),
};

#[derive(Debug)]
struct Aunt<'input> {
    props: Vec<(&'input str, usize)>,
}

impl Aunt<'_> {
    fn matches(&self, exact: bool) -> bool {
        let condition = if exact {
            |&(name, count)| INFO.get(name).unwrap().value() == count
        } else {
            |&(name, count)| INFO.get(name).unwrap().matches(count)
        };

        self.props.iter().all(condition)
    }
}

impl<'input> From<&'input str> for Aunt<'input> {
    fn from(input: &'input str) -> Self {
        let (_, props) = input.split_once(": ").unwrap();
        Self {
            props: props
                .split(", ")
                .map(|prop| {
                    let (name, count) = prop.split_once(": ").unwrap();
                    (name, count.parse().unwrap())
                })
                .collect(),
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let aunts = input.lines().map(Aunt::from);

    let solution1: Solution = aunts
        .clone()
        .enumerate()
        .find(|(_, aunt)| aunt.matches(true))
        .unwrap()
        .0
        + 1;

    let solution2: Solution = aunts
        .clone()
        .enumerate()
        .find(|(_, aunt)| aunt.matches(false))
        .unwrap()
        .0
        + 1;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
