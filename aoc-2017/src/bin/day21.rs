use aoc_2017::*;

const DAY: i32 = 21;
type Solution = usize;

#[derive(Clone)]
struct Pattern(Vec<Vec<bool>>);

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self(
            value
                .split('/')
                .map(|row| row.chars().map(|ch| ch == '#').collect())
                .collect(),
        )
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Pattern {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn rotate(&self) -> Self {
        let size = self.size();
        let mut pattern = vec![vec![false; size]; size];

        for (i, row) in pattern.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = self.0[size - j - 1][i];
            }
        }

        Self(pattern)
    }

    fn flip(&self) -> Self {
        let size = self.size();
        let mut pattern = vec![vec![false; size]; size];

        for (i, row) in pattern.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = self.0[i][size - j - 1];
            }
        }

        Self(pattern)
    }

    fn split(&self) -> Vec<Self> {
        if self.size() == 2 || self.size() == 3 {
            return vec![Pattern(self.0.clone())];
        }

        let split_size = if self.size() % 2 == 0 { 2 } else { 3 };
        let mut splits = vec![];
        for split_y in (0..self.size()).step_by(split_size) {
            for split_x in (0..self.size()).step_by(split_size) {
                let pattern = Self(
                    (split_y..split_y + split_size)
                        .map(|i| {
                            (split_x..split_x + split_size)
                                .map(|j| self.0[i][j])
                                .collect()
                        })
                        .collect(),
                );

                splits.push(pattern);
            }
        }

        splits
    }

    fn join(splits: &[Pattern]) -> Self {
        let size = (1..).find(|x| x * x == splits.len()).unwrap();
        let split_size = splits[0].size();

        let mut pattern = vec![];
        for y in 0..size {
            for split_y in 0..split_size {
                let mut row = vec![];
                for x in 0..size {
                    let split = &splits[y * size + x];
                    for split_x in 0..split_size {
                        row.push(split.0[split_y][split_x]);
                    }
                }
                pattern.push(row);
            }
        }

        Self(pattern)
    }

    fn apply(&self, rules: &[Rule]) -> Self {
        if self.size() > 3 {
            let splits = self.split();
            let new_splits: Vec<_> = splits.iter().map(|pattern| pattern.apply(rules)).collect();
            Self::join(&new_splits)
        } else {
            for rule in rules.iter() {
                if rule.matches(self) {
                    return rule.to.clone();
                }
            }

            unreachable!()
        }
    }
}

struct Rule {
    from: Pattern,
    to: Pattern,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
    }
}

impl Rule {
    fn parse(value: &str) -> Vec<Self> {
        let (from, to) = value.split_once(" => ").unwrap();
        let (mut from, to) = (Pattern::from(from), Pattern::from(to));
        let mut rules = vec![];

        for _ in 0..4 {
            rules.push(Rule {
                from: from.clone(),
                to: to.clone(),
            });
            from = from.rotate();
        }

        from = from.flip();
        for _ in 0..4 {
            rules.push(Rule {
                from: from.clone(),
                to: to.clone(),
            });
            from = from.rotate();
        }

        rules
    }

    fn matches(&self, pattern: &Pattern) -> bool {
        if self.from.size() != pattern.size() {
            return false;
        }

        &self.from == pattern
    }
}

fn main() {
    let input = get_input_text(DAY);
    let mut rules: Vec<_> = input.lines().flat_map(Rule::parse).collect();
    rules.dedup(); // Some rules might repeat

    // .#.
    // ..#
    // ###
    let start_pattern = Pattern(vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ]);

    let solution1: Solution = {
        let mut pattern = start_pattern.clone();
        (0..5).for_each(|_| pattern = pattern.apply(&rules));
        pattern.0.iter().flatten().filter(|&&pixel| pixel).count()
    };

    let solution2: Solution = {
        let mut pattern = start_pattern;
        (0..18).for_each(|_| pattern = pattern.apply(&rules));
        pattern.0.iter().flatten().filter(|&&pixel| pixel).count()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
