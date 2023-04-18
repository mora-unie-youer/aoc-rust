use std::collections::HashMap;

use aoc_2018::*;
use chrono::NaiveDateTime;

const DAY: i32 = 4;
type Solution = usize;

// Number of seconds from Unix epoch
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp(i64);

impl From<&str> for Timestamp {
    fn from(value: &str) -> Self {
        let dt = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M").unwrap();
        Self(dt.timestamp())
    }
}

impl Timestamp {
    fn minute(&self) -> i64 {
        let minutes = self.0 / 60;
        let minute = minutes % 60;
        minute + 60 * (minute < 0) as i64
    }
}

#[derive(PartialEq, Eq)]
enum EntryEvent {
    Asleep,
    Shift(usize),
    Wakes,
}

impl From<&str> for EntryEvent {
    fn from(value: &str) -> Self {
        if value.starts_with("Guard") {
            Self::Shift(value.split(' ').nth(1).unwrap()[1..].parse().unwrap())
        } else if value.starts_with("falls") {
            Self::Asleep
        } else {
            Self::Wakes
        }
    }
}

#[derive(Eq)]
struct Entry {
    timestamp: Timestamp,
    event: EntryEvent,
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        let timestamp: Timestamp = value[1..17].into();
        let event: EntryEvent = value[19..].into();
        Self { timestamp, event }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = get_input_text(DAY);
    let mut entries: Vec<_> = input.lines().map(Entry::from).collect();
    entries.sort();

    let guards = {
        let mut guards: HashMap<usize, [usize; 60]> = HashMap::new();
        let mut guard_id = 0;
        let mut asleep_minute = 0;
        for entry in &entries {
            match entry.event {
                EntryEvent::Shift(id) => guard_id = id,
                EntryEvent::Asleep => asleep_minute = entry.timestamp.minute(),
                EntryEvent::Wakes => {
                    let guard = guards.entry(guard_id).or_insert([0; 60]);
                    for i in asleep_minute..entry.timestamp.minute() {
                        guard[i as usize] += 1;
                    }
                }
            }
        }

        guards
    };

    let solution1: Solution = {
        let sleepy_guard = *guards
            .iter()
            .max_by_key(|(_, sleeps)| sleeps.iter().sum::<usize>())
            .unwrap()
            .0;
        let sleepy_minute = guards[&sleepy_guard]
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .unwrap()
            .0;
        sleepy_guard * sleepy_minute
    };

    let solution2: Solution = {
        let guard = guards
            .iter()
            .map(|(i, minutes)| {
                let sleepy_minute = minutes
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &count)| count)
                    .unwrap();
                (i, sleepy_minute)
            })
            .max_by_key(|(_, (_, &count))| count)
            .unwrap();

        let guard_id = guard.0;
        let sleepy_minute = guard.1 .0;
        guard_id * sleepy_minute
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
