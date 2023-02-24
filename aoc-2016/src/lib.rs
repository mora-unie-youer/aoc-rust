use std::borrow::Cow;

pub use aoc::show_solution;

pub const YEAR: i32 = 2016;

pub fn get_input_text(day: i32) -> Cow<'static, str> {
    aoc::get_input_text(YEAR, day)
}
