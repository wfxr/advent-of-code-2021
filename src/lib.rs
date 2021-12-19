#![feature(drain_filter)]
#![feature(bool_to_option)]
#![feature(int_abs_diff)]
#![feature(iter_intersperse)]
#![feature(array_windows)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(box_syntax)]
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
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
); // +SOLUTIONS+

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Solution {
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
    pub input: &'static str,
}
