use std::borrow::Cow;

pub use aoc::show_solution;

pub const YEAR: i32 = 2024;

pub fn get_input_text(day: i32) -> Cow<'static, str> {
    aoc::get_input_text(YEAR, day)
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
