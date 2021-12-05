#![feature(drain_filter)]
#![feature(bool_to_option)]
#![feature(int_abs_diff)]
#![feature(test)]
extern crate test;

mod macros;
#[cfg(test)]
mod testmacros;

#[rustfmt::skip]
crate::aoc!(
    day01,
    day02,
    day03,
    day04,
    day05,
); // +SOLUTIONS+

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Solution {
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}
