mod macros;
#[cfg(test)]
mod testmacros;

#[rustfmt::skip]
crate::aoc!(
    day01,
    day02,
    day03,
); // +SOLUTIONS+

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Solution {
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}
